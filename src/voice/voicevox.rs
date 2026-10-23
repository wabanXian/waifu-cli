//! VOICEVOX CORE C API integration for Windows x64.
//! No Python wheel, DLL installation, or global PATH modification required.
//! Libraries, dictionaries and models are discovered from the project root.

use std::env;
use std::ffi::{c_char, c_void, CStr, CString, OsStr};
use std::fs;
use std::hash::{Hash, Hasher};
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryExW(path: *const u16, file: *mut c_void, flags: u32) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, proc_name: *const c_char) -> *const c_void;
    fn FreeLibrary(module: *mut c_void) -> i32;
    fn AddDllDirectory(new_directory: *const u16) -> *mut c_void;
    fn RemoveDllDirectory(cookie: *mut c_void) -> i32;
    fn SetDefaultDllDirectories(directory_flags: u32) -> i32;
}
#[link(name = "winmm")]
extern "system" {
    fn PlaySoundW(sound: *const u16, module: *mut c_void, flags: u32) -> i32;
}

const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x0000_0100;
const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x0000_1000;
const SND_MEMORY: u32 = 0x0004;
const SND_NODEFAULT: u32 = 0x0002;
const PREFIX: &str = "voicevox_core-windows-x64-";

type Ptr = *mut c_void;
type ResultCode = i32;

#[repr(C)]
#[derive(Clone, Copy)]
struct OrtOptions {
    filename: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InitOptions {
    acceleration_mode: i32,
    cpu_num_threads: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ModelOptions {
    on_existing: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TtsOptions {
    enable_interrogative_upspeak: bool,
}

type OrtDefault = unsafe extern "C" fn() -> OrtOptions;
type OrtLoad = unsafe extern "C" fn(OrtOptions, *mut *const c_void) -> ResultCode;
type OrtName = unsafe extern "C" fn() -> *const c_char;
type JtalkNew = unsafe extern "C" fn(*const c_char, *mut Ptr) -> ResultCode;
type JtalkDelete = unsafe extern "C" fn(Ptr);
type InitDefault = unsafe extern "C" fn() -> InitOptions;
type SynthNew = unsafe extern "C" fn(*const c_void, *const c_void, InitOptions, *mut Ptr) -> ResultCode;
type SynthDelete = unsafe extern "C" fn(Ptr);
type SynthGpuMode = unsafe extern "C" fn(*const c_void) -> bool;
type ModelOpen = unsafe extern "C" fn(*const c_char, *mut Ptr) -> ResultCode;
type ModelDelete = unsafe extern "C" fn(Ptr);
type ModelLoadDefault = unsafe extern "C" fn() -> ModelOptions;
type SynthLoadModel = unsafe extern "C" fn(*const c_void, *const c_void, ModelOptions) -> ResultCode;
type TtsDefault = unsafe extern "C" fn() -> TtsOptions;
type Tts = unsafe extern "C" fn(*const c_void, *const c_char, u32, TtsOptions, *mut usize, *mut *mut u8) -> ResultCode;
type BytesFree = unsafe extern "C" fn(*mut u8);
type ErrorMessage = unsafe extern "C" fn(ResultCode) -> *const c_char;

/// Register GPU runtime dependency folders without editing the system PATH.
/// Kept alive for the duration of synthesis and DLL unload.
struct DllDirectories(Vec<Ptr>);

impl DllDirectories {
    fn register(paths: &[PathBuf]) -> Result<Self, String> {
        let mut dirs = Self(Vec::new());
        if unsafe { SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS) } == 0 {
            return Err(format!(
                "无法配置 Windows DLL 搜索路径：{}",
                std::io::Error::last_os_error()
            ));
        }
        for path in paths {
            if !path.is_dir() { continue; }
            let absolute = fs::canonicalize(path)
                .map_err(|e| format!("无法解析 DLL 目录 {}：{e}", path.display()))?;
            let wide: Vec<u16> = absolute.as_os_str().encode_wide().chain(Some(0)).collect();
            let cookie = unsafe { AddDllDirectory(wide.as_ptr()) };
            if cookie.is_null() {
                return Err(format!(
                    "无法添加 DLL 搜索目录 {}：{}",
                    absolute.display(),
                    std::io::Error::last_os_error()
                ));
            }
            dirs.0.push(cookie);
        }
        Ok(dirs)
    }
}

impl Drop for DllDirectories {
    fn drop(&mut self) {
        for cookie in self.0.drain(..).rev() {
            unsafe { RemoveDllDirectory(cookie); }
        }
    }
}

struct Library(Ptr);

impl Library {
    fn open(path: &Path) -> Result<Self, String> {
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        // Search the loaded DLL's directory for its dependencies, not PATH/CWD.
        let ptr = unsafe {
            LoadLibraryExW(
                wide.as_ptr(),
                ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )
        };
        if ptr.is_null() {
            return Err(format!("加载 {} 失败：{}", path.display(), std::io::Error::last_os_error()));
        }
        Ok(Self(ptr))
    }

    unsafe fn symbol<T: Copy>(&self, name: &'static [u8]) -> Result<T, String> {
        let ptr = GetProcAddress(self.0, name.as_ptr().cast());
        if ptr.is_null() {
            return Err(format!("C API 缺少符号 {}", String::from_utf8_lossy(&name[..name.len() - 1])));
        }
        if std::mem::size_of::<T>() != std::mem::size_of::<*const c_void>() {
            return Err("VOICEVOX C API 函数指针大小不匹配".into());
        }
        Ok(std::mem::transmute_copy(&ptr))
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.0); }
    }
}

struct Api {
    ort_default: OrtDefault,
    ort_load: OrtLoad,
    ort_name: OrtName,
    jtalk_new: JtalkNew,
    jtalk_delete: JtalkDelete,
    init_default: InitDefault,
    synth_new: SynthNew,
    synth_delete: SynthDelete,
    synth_gpu_mode: SynthGpuMode,
    model_open: ModelOpen,
    model_delete: ModelDelete,
    model_load_default: ModelLoadDefault,
    synth_load_model: SynthLoadModel,
    tts_default: TtsDefault,
    tts: Tts,
    bytes_free: BytesFree,
    error_message: ErrorMessage,
}

impl Api {
    unsafe fn load(lib: &Library) -> Result<Self, String> {
        macro_rules! sym {
            ($name:literal) => { lib.symbol(concat!($name, "\0").as_bytes())? };
        }
        Ok(Self {
            ort_default: sym!("voicevox_make_default_load_onnxruntime_options"),
            ort_load: sym!("voicevox_onnxruntime_load_once"),
            ort_name: sym!("voicevox_get_onnxruntime_lib_recommended_versioned_filename"),
            jtalk_new: sym!("voicevox_open_jtalk_rc_new"),
            jtalk_delete: sym!("voicevox_open_jtalk_rc_delete"),
            init_default: sym!("voicevox_make_default_initialize_options"),
            synth_new: sym!("voicevox_synthesizer_new"),
            synth_delete: sym!("voicevox_synthesizer_delete"),
            synth_gpu_mode: sym!("voicevox_synthesizer_is_gpu_mode"),
            model_open: sym!("voicevox_voice_model_file_open"),
            model_delete: sym!("voicevox_voice_model_file_delete"),
            model_load_default: sym!("voicevox_make_default_load_voice_model_options"),
            synth_load_model: sym!("voicevox_synthesizer_load_voice_model"),
            tts_default: sym!("voicevox_make_default_tts_options"),
            tts: sym!("voicevox_synthesizer_tts"),
            bytes_free: sym!("voicevox_bytes_free"),
            error_message: sym!("voicevox_error_result_to_message"),
        })
    }

    fn check(&self, result: ResultCode, operation: &str) -> Result<(), String> {
        if result == 0 {
            return Ok(());
        }
        let msg = unsafe {
            let p = (self.error_message)(result);
            if p.is_null() {
                String::from("unknown error")
            } else {
                CStr::from_ptr(p).to_string_lossy().into_owned()
            }
        };
        Err(format!("{operation} 失败（{result}）：{msg}"))
    }
}

// Drop the engine objects BEFORE the DLL is unloaded.
struct Resources<'a> {
    api: &'a Api,
    jtalk: Ptr,
    synth: Ptr,
    model: Ptr,
}
impl Drop for Resources<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.synth.is_null() { (self.api.synth_delete)(self.synth); }
            if !self.model.is_null() { (self.api.model_delete)(self.model); }
            if !self.jtalk.is_null() { (self.api.jtalk_delete)(self.jtalk); }
        }
    }
}

#[derive(Debug)]
struct Assets {
    core: PathBuf,
    dll: PathBuf,
    ort_dir: PathBuf,
    additional_dll_dirs: Vec<PathBuf>,
    dict: PathBuf,
    model: PathBuf,
    style: u32,
}

fn version(name: &str) -> Vec<u64> {
    name.strip_prefix(PREFIX).unwrap_or("")
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|part| part.parse().ok())
        .collect()
}

fn versioned_dirs(root: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(root) else { return vec![] };
    let mut dirs: Vec<PathBuf> = entries.flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.file_name()
            .and_then(OsStr::to_str).is_some_and(|n| n.starts_with(PREFIX)))
        .collect();
    dirs.sort_by(|a, b| {
        let av = version(&a.file_name().unwrap().to_string_lossy());
        let bv = version(&b.file_name().unwrap().to_string_lossy());
        bv.cmp(&av).then_with(|| b.cmp(a))
    });
    dirs
}

fn dll_in(core: &Path) -> Option<PathBuf> {
    [
        core.join("lib/voicevox_core.dll"),
        core.join("c_api/lib/voicevox_core.dll"),
        core.join("voicevox_core.dll"),
    ].into_iter().find(|p| p.is_file())
}

fn find_installation() -> Result<(PathBuf, PathBuf, PathBuf), String> {
    // The executable may be at project root, target/debug, or target/release.
    // The current working directory is only a fallback, never a requirement.
    let mut roots = Vec::new();
    if let Some(exe_dir) = env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)) {
        roots.extend(exe_dir.ancestors().take(5).map(Path::to_path_buf));
    }
    if let Ok(cwd) = env::current_dir() {
        roots.extend(cwd.ancestors().take(5).map(Path::to_path_buf));
    }
    if let Some(custom) = env::var_os("WAIFU_PROJECT_ROOT") {
        roots.insert(0, PathBuf::from(custom));
    }
    roots.dedup();
    for root in roots {
        for core in versioned_dirs(&root) {
            if let Some(dll) = dll_in(&core) {
                return Ok((root, core, dll));
            }
        }
    }
    Err(format!(
        "找不到 {PREFIX}*\\lib\\voicevox_core.dll；请将解压后的 C API 文件夹放在项目根目录，或设置 WAIFU_PROJECT_ROOT"
    ))
}

fn find_named_dir(base: &Path, prefix: &str) -> Option<PathBuf> {
    fs::read_dir(base).ok()?
        .flatten()
        .map(|e| e.path())
        .find(|path| path.is_dir() && path.file_name()
            .and_then(OsStr::to_str).is_some_and(|s| s.starts_with(prefix)))
}

fn find_model(base: &Path) -> Option<PathBuf> {
    let mut items: Vec<PathBuf> = fs::read_dir(base).ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(OsStr::to_str)
            .is_some_and(|ext| ext.eq_ignore_ascii_case("vvm")))
        .collect();
    items.sort_by(|a, b| {
        // Prefer 0.vvm, then 1.vvm, and so on; no hardcoded model filename.
        let num = |p: &PathBuf| p.file_stem().and_then(OsStr::to_str)
            .and_then(|s| s.parse::<u64>().ok()).unwrap_or(u64::MAX);
        num(a).cmp(&num(b)).then_with(|| a.cmp(b))
    });
    items.into_iter().next()
}

impl Assets {
    fn discover() -> Result<Self, String> {
        let (root, core, dll) = find_installation()?;
        // Downloader resources can be under the unpacked version directory or
        // under a sibling voicevox_core directory downloaded separately.
        let bases = [core.clone(), root.join("voicevox_core"), root.clone()];
        let dict = bases.iter().find_map(|base| {
            find_named_dir(&base.join("dict"), "open_jtalk_dic_")
                .or_else(|| find_named_dir(base, "open_jtalk_dic_"))
        }).ok_or_else(|| {
            format!("没有找到 Open JTalk 词典。请放到 {}\\dict\\open_jtalk_dic_*", core.display())
        })?;

        let model = if let Some(path) = env::var_os("WAIFU_VOICEVOX_MODEL") {
            let path = PathBuf::from(path);
            if path.is_absolute() { path } else { root.join(path) }
        } else {
            bases.iter().find_map(|base| {
                find_model(&base.join("models/vvms"))
                    .or_else(|| find_model(&base.join("models")))
                    .or_else(|| find_model(&base.join("vvms")))
            }).ok_or_else(|| {
                format!("找不到 .vvm 声音模型。请放到 {}\\models\\vvms\\", core.display())
            })?
        };
        if !model.is_file() {
            return Err(format!("VOICEVOX 模型不存在：{}", model.display()));
        }

        // Prefer the official Downloader runtime over an older CPU-only DLL
        // stored next to the C API versioned folder.
        let ort_dir = if let Some(custom) = env::var_os("WAIFU_VOICEVOX_ORT_DIR") {
            let path = PathBuf::from(custom);
            let path = if path.is_absolute() { path } else { root.join(path) };
            if !path.is_dir() {
                return Err(format!("指定的 ONNX Runtime 目录不存在：{}", path.display()));
            }
            path
        } else {
            [root.join("voicevox_core"), core.clone(), root.clone()]
                .iter()
                .find_map(|base| {
                    let path = base.join("onnxruntime/lib");
                    if path.is_dir() { Some(path) } else { None }
                })
                .ok_or_else(|| {
                    format!("找不到 ONNX Runtime 目录：{}/onnxruntime/lib", core.display())
                })?
        };

        // CUDA/cuDNN and DirectML dependencies are in additional_libraries.
        let mut additional_dll_dirs = vec![ort_dir.clone()];
        if let Some(parent) = dll.parent() {
            additional_dll_dirs.push(parent.to_path_buf());
        }
        for base in &bases {
            let path = base.join("additional_libraries");
            if path.is_dir() && !additional_dll_dirs.contains(&path) {
                additional_dll_dirs.push(path);
            }
        }
        let style = env::var("WAIFU_VOICEVOX_STYLE_ID")
            .unwrap_or_else(|_| "0".into())
            .parse::<u32>()
            .map_err(|_| "WAIFU_VOICEVOX_STYLE_ID 必须是非负整数".to_string())?;

        Ok(Self { core, dll, ort_dir, additional_dll_dirs, dict, model, style })
    }
}

fn cpath(path: &Path) -> Result<CString, String> {
    CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| format!("文件路径包含 NUL：{}", path.display()))
}

fn cache_file(assets: &Assets, text: &str) -> Option<PathBuf> {
    let dir = env::var_os("WAIFU_VOICE_CACHE_DIR")
        .map(PathBuf::from)
        .or_else(|| env::var_os("LOCALAPPDATA")
            .map(|v| PathBuf::from(v).join("waifu-cli/voice-cache")))?;
    let metadata = fs::metadata(&assets.model).ok()?;
    let modified = metadata.modified().ok()?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    assets.core.hash(&mut hasher);
    assets.model.hash(&mut hasher);
    metadata.len().hash(&mut hasher);
    modified.hash(&mut hasher);
    assets.style.hash(&mut hasher);
    text.hash(&mut hasher);
    Some(dir.join(format!("{:016x}.wav", hasher.finish())))
}

fn valid_wav(wav: &[u8]) -> bool {
    wav.len() >= 44 && wav.starts_with(b"RIFF") && &wav[8..12] == b"WAVE"
}

fn play_wav(wav: &[u8]) -> Result<(), String> {
    if !valid_wav(wav) {
        return Err("VOICEVOX 返回的音频不是有效的 RIFF WAV".into());
    }
    // SND_MEMORY is synchronous here: wav stays alive for the entire playback.
    let ok = unsafe { PlaySoundW(wav.as_ptr().cast(), ptr::null_mut(), SND_MEMORY | SND_NODEFAULT) };
    if ok == 0 {
        return Err(format!("Windows WAV 播放失败：{}", std::io::Error::last_os_error()));
    }
    Ok(())
}

fn synthesize_with_gpu(assets: &Assets, text: &str) -> Result<(Vec<u8>, bool), String> {
    // Register GPU libraries for this process only; they may be loaded on demand
    // by the ONNX Runtime execution provider after the main DLL is loaded.
    let _dll_dirs = DllDirectories::register(&assets.additional_dll_dirs)?;
    let lib = Library::open(&assets.dll)?;
    let api = unsafe { Api::load(&lib)? };
    let mut resources = Resources {
        api: &api,
        jtalk: ptr::null_mut(),
        synth: ptr::null_mut(),
        model: ptr::null_mut(),
    };

    let name_ptr = unsafe { (api.ort_name)() };
    if name_ptr.is_null() {
        return Err("VOICEVOX 无法返回推荐的 ONNX Runtime DLL 文件名".into());
    }
    let name = unsafe { CStr::from_ptr(name_ptr) }.to_string_lossy();
    let ort_path = assets.ort_dir.join(name.as_ref());
    if !ort_path.is_file() {
        return Err(format!("找不到配套 ONNX Runtime DLL：{}", ort_path.display()));
    }

    let ort_c = cpath(&ort_path)?;
    let mut ort = ptr::null();
    let mut opts = unsafe { (api.ort_default)() };
    opts.filename = ort_c.as_ptr();
    api.check(unsafe { (api.ort_load)(opts, &mut ort) }, "加载 ONNX Runtime")?;

    let dict_c = cpath(&assets.dict)?;
    api.check(unsafe { (api.jtalk_new)(dict_c.as_ptr(), &mut resources.jtalk) }, "加载日语词典")?;

    let opts = unsafe { (api.init_default)() };
    api.check(unsafe { (api.synth_new)(ort, resources.jtalk, opts, &mut resources.synth) }, "创建合成器")?;
    let gpu_active = unsafe { (api.synth_gpu_mode)(resources.synth) };

    let model_c = cpath(&assets.model)?;
    api.check(unsafe { (api.model_open)(model_c.as_ptr(), &mut resources.model) }, "读取语音模型")?;
    let opts = unsafe { (api.model_load_default)() };
    api.check(unsafe { (api.synth_load_model)(resources.synth, resources.model, opts) }, "加载语音模型")?;

    let text = CString::new(text).map_err(|_| "台词包含 NUL 字符".to_string())?;
    let opts = unsafe { (api.tts_default)() };
    let mut length = 0usize;
    let mut wav: *mut u8 = ptr::null_mut();
    let result = unsafe { (api.tts)(resources.synth, text.as_ptr(), assets.style, opts, &mut length, &mut wav) };
    if result != 0 {
        if !wav.is_null() { unsafe { (api.bytes_free)(wav); } }
        return Err(api.check(result, "合成语音").unwrap_err());
    }
    if wav.is_null() || length == 0 {
        return Err("VOICEVOX 返回空音频".into());
    }
    let bytes = unsafe { std::slice::from_raw_parts(wav, length).to_vec() };
    unsafe { (api.bytes_free)(wav); }
    Ok((bytes, gpu_active))
}

fn synthesize(assets: &Assets, text: &str) -> Result<Vec<u8>, String> {
    synthesize_with_gpu(assets, text).map(|(wav, _)| wav)
}

/// Lightweight checks only: no GPU inference, DLL loads or speech synthesis.
pub(super) fn inspect() -> Vec<(String, String, bool)> {
    let mut results = Vec::new();
    match find_installation() {
        Ok((_, core, dll)) => results.push((
            "VOICEVOX C API".into(),
            format!("{} (DLL: {})", core.display(), dll.display()),
            true,
        )),
        Err(err) => {
            results.push(("VOICEVOX C API".into(), err, false));
            return results;
        }
    }
    match Assets::discover() {
        Ok(assets) => {
            results.push(("日语词典".into(), assets.dict.display().to_string(), true));
            results.push(("声音模型".into(), format!("{} · Style ID {}", assets.model.display(), assets.style), true));
            let dlls = fs::read_dir(&assets.ort_dir).ok().into_iter().flatten().flatten()
                .filter(|e| e.path().extension().and_then(OsStr::to_str)
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("dll")))
                .count();
            results.push((
                "ONNX Runtime".into(),
                format!("{} · 找到 {dlls} 个 DLL（尚未加载）", assets.ort_dir.display()),
                dlls > 0,
            ));
            results.push(("GPU".into(), "未测试；运行 waifu doctor --voice-test 查询实际运行模式".into(), true));
        }
        Err(err) => results.push(("语音资源".into(), err, false)),
    }
    results
}

/// Always synthesize an uncached sample without playing it, so CPU/GPU mode
/// is measured from the live synthesizer rather than inferred from GPU names.
pub(super) fn probe_gpu_mode() -> Result<bool, String> {
    let assets = Assets::discover()?;
    let (_, is_gpu) = synthesize_with_gpu(&assets, "音声合成のテストです。")?;
    Ok(is_gpu)
}

pub fn speak(text: &str) -> Result<(), String> {
    if text.trim().is_empty() {
        return Ok(());
    }
    let assets = Assets::discover()?;
    let cache = cache_file(&assets, text);
    if let Some(path) = &cache {
        if let Ok(bytes) = fs::read(path) {
            if valid_wav(&bytes) {
                return play_wav(&bytes);
            }
        }
    }
    let wav = synthesize(&assets, text)?;
    if let Some(path) = &cache {
        if let Some(dir) = path.parent() {
            if fs::create_dir_all(dir).is_ok() {
                let tmp = path.with_extension(format!("{}.tmp", std::process::id()));
                if fs::write(&tmp, &wav).is_ok() {
                    let _ = fs::rename(&tmp, path);
                }
                let _ = fs::remove_file(tmp);
            }
        }
    }
    play_wav(&wav)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_ordering() {
        assert!(version("voicevox_core-windows-x64-0.17.1") > version("voicevox_core-windows-x64-0.9.9"));
    }
    #[test]
    fn wav_validation() {
        assert!(!valid_wav(&[]));
        assert!(!valid_wav(&vec![0u8; 60]));
        let mut bytes = vec![0u8; 44];
        bytes[..4].copy_from_slice(b"RIFF");
        bytes[8..12].copy_from_slice(b"WAVE");
        assert!(valid_wav(&bytes));
    }
}
