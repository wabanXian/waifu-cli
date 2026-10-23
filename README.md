# waifu-ping

`waifu-ping` 是一个面向 Windows 终端的 Rust 工具箱。它为常用命令提供彩色、带随机台词的 waifu 风格输出，并通过 `waifu-shell.ps1` 将部分命令包装成更自然的 PowerShell 体验。

项目构建出的程序名为 `waifu.exe`。

## 构建

需要安装稳定版 Rust 工具链：

```powershell
cargo build --locked
```

发布构建：

```powershell
cargo build --release --locked
```

产物分别位于：

- 调试构建：`target\debug\waifu.exe`
- 发布构建：`target\release\waifu.exe`

`build.rs` 会自动把 `src\waifu.json` 和 `speak.py` 复制到对应的产物目录。运行时请让这两个文件与 `waifu.exe` 保持在同一目录。

## PowerShell 集成

先将构建目录加入当前 PowerShell 会话的 `PATH`：

```powershell
$waifuDir = (Resolve-Path .\target\debug).Path
$env:PATH = "$waifuDir;$env:PATH"
```

发布构建请将路径替换为 `target\release`。

然后加载 Shell 包装：

```powershell
. .\waifu-shell.ps1
```

如需每次启动 PowerShell 时自动加载，可把 PATH 设置和下面这行加入自己的 `$PROFILE`，并使用脚本的绝对路径：

```powershell
. "D:\path\to\waifu-ping\waifu-shell.ps1"
```

脚本会提供 `cd`、`ls`、`cat`、`echo`、`clear`、`cls` 和 `ps` 等 waifu 化入口。`Get-Process` 始终保持为 PowerShell 原生命令；waifu 化进程列表请使用 `ps`。

## 可选语音功能

`--miao`（以及 PowerShell 的 `-Miao`）播报 `src/waifu.json` 中的日文台词。默认后端是 **Rust 直接调用 VOICEVOX CORE C API（Windows x64）**，不需要 Python wheel、VOICEVOX 编辑器常驻或修改全局 PATH。不使用 `--miao` 时不会加载语音引擎。

### 解压位置：自动识别版本号

把官方 Windows x64 的 C API 压缩包解压到**项目根目录**，文件夹名保持 `voicevox_core-windows-x64-版本号`：

```text
waifu-ping/
├── Cargo.toml
├── voicevox_core-windows-x64-0.17.1/   # 示例；实际版本不固定
│   ├── lib/
│   │   └── voicevox_core.dll
│   └── include/
├── target/
│   └── release/
│       └── waifu.exe
└── ...
```

程序会扫描 `voicevox_core-windows-x64-*`，优先选择版本号最高且含 DLL 的文件夹；`lib/voicevox_core.dll`、`c_api/lib/voicevox_core.dll` 和文件夹根目录下的 DLL 都支持。不依赖 PowerShell 当前工作目录；会从 exe 位置向上定位项目。自定义位置可以设置 `WAIFU_PROJECT_ROOT` 指向项目根目录。

**C API DLL 不包含声音！** 还需要 VOICEVOX ONNX Runtime、Open JTalk 日语词典与 .vvm 声音模型。这些资源可放在上述版本文件夹下，也支持放在项目根目录的 `voicevox_core/` 文件夹中（官方 Downloader 常见布局）：

```text
waifu-ping/
├── voicevox_core-windows-x64-0.17.1/
│   └── lib/voicevox_core.dll
└── voicevox_core/
    ├── onnxruntime/lib/...
    ├── dict/open_jtalk_dic_utf_8-1.11/...
    └── models/vvms/0.vvm
```

如果模型、词典或运行库没找到，错误信息会指出缺少哪个目录或文件。

### 本地测试

```powershell
cd D:\BaiduNetdiskDownload\waifu-ping
git pull origin master
cargo build --release --locked

# 默认语音后端就是 Voicevox 的 C API
waifu ping 1.1.1.1 --miao

# 可选：指定语音风格 ID，需要与模型匹配
$env:WAIFU_VOICEVOX_STYLE_ID = '0'

# 可选：指定模型（支持相对项目根目录或绝对路径）
$env:WAIFU_VOICEVOX_MODEL = 'voicevox_core\models\vvms\0.vvm'
```

未指定模型时，先查找版本文件夹中的 .vvm，再找根目录同级的 `voicevox_core/models/vvms`；默认选编号最小的模型。默认风格 ID 为 0，如果选中的模型不支持它，请修改 `WAIFU_VOICEVOX_STYLE_ID`。

新台词第一次合成会加载模型，可能有短暂延迟；之后相同台词可直接使用 `%LOCALAPPDATA%\waifu-cli\voice-cache` 中缓存的 WAV。可通过 `WAIFU_VOICE_CACHE_DIR` 指定其他缓存位置。声音播放会等待完成；语音失败不会中断命令核心功能。

请遵守所使用声音模型与角色的授权规则，尤其在对外分发模型或合成音频时。

### 启用 NVIDIA RTX 显卡（CUDA）

若日志显示 `CUDA ... 現在ロードされているONNX Runtimeでは利用できません`，表示当前载入的 ONNX Runtime 是不支持 CUDA 的版本；只切换加速选项无法解决。需要通过 [官方 Downloader](https://github.com/VOICEVOX/voicevox_core/blob/main/docs/guide/user/downloader.md) 下载 **CUDA 版 ONNX Runtime** 和 **additional-libraries**。

在项目根目录、已有 `download.exe` 的情况下执行：

```powershell
# 之前遇到匿名 API 限流时使用已登录的 GitHub CLI
$env:GH_TOKEN = gh auth token

# 只下载 CUDA 的运行库与依赖，不重复下载模型
.\download.exe --devices cuda --only onnxruntime additional-libraries -o .\voicevox_core

# 不必修改系统 PATH
$env:WAIFU_TTS_BACKEND = 'voicevox'
cargo build --release --locked
.\target\release\waifu.exe ping 1.1.1.1 --miao

# 下载结束后清除终端环境变量中的临时令牌
Remove-Item Env:GH_TOKEN
```

更新后的加载顺序优先选择 `voicevox_core/onnxruntime/lib` 下的官方 Downloader 运行库。程序会将 `voicevox_core/additional_libraries` 与 ONNX Runtime 所在目录注册到当前进程的 Windows DLL 搜索路径，**无需全局修改 PATH**。默认初始化选项为 AUTO，当 GPU 后端可用且测试成功时 CORE 会选择 GPU。

如果有多个 ONNX Runtime 目录，可以明确指定一个：

```powershell
$env:WAIFU_VOICEVOX_ORT_DIR = 'D:\BaiduNetdiskDownload\waifu-ping\voicevox_core\onnxruntime\lib'
```

这只指定运行库所在目录，不是模型目录；`WAIFU_VOICEVOX_MODEL` 仍负责选择 .vvm。CUDA 运行可能依赖对应版本的 NVIDIA 驱动、CUDA 和 cuDNN 库，Downloader 的 `additional_libraries` 负责提供项目需要的一部分 DLL。

注意：已缓存的台词只播放 WAV，不会再触发 GPU 合成。如果想验证 GPU 日志，请使用一条尚未缓存的语音，或自行清空 `%LOCALAPPDATA%\waifu-cli\voice-cache` 中的 WAV。

### 旧 CeVIO 兼容模式

想继续使用 さとうささら 的 CeVIO 声音，手动选择：

```powershell
$env:WAIFU_TTS_BACKEND = 'cevio'
waifu ping 1.1.1.1 --miao

# 再切回无需 Python 的 Voicevox C API
$env:WAIFU_TTS_BACKEND = 'voicevox'
```

旧 CeVIO 模式仍调用 exe 同级的 `speak.py`，需要 Python、pythonnet、CeVIO Host。Rust 的 VOICEVOX C API 模式不调用 Python。独立分发 waifu.exe 时仍要带上 `waifu.json`；CeVIO 模式还需要 `speak.py`。

## 环境诊断：waifu doctor

新安装、升级 VOICEVOX 模型或 CUDA 运行库时，可以先运行：

```powershell
waifu doctor
```

默认**不会加载 VOICEVOX DLL 或合成语音**，只做轻量检查：

- `waifu.exe` 所在路径及 `waifu.json` 的读取、结构验证
- 附近是否存在 `waifu-shell.ps1` 与 PATH 中的 PowerShell 程序（**不能**验证当前 PowerShell 会话是否加载了包装函数）
- 所选的语音后端、VOICEVOX C API 版本文件夹
- Open JTalk 词典、.vvm 模型与 Style ID、ONNX Runtime DLL 文件
- CeVIO 兼容模式下的 `speak.py` 与 Python 可执行文件（不启动 Host）

**注意：找到 CUDA 运行库、显卡或 DLL 并不证明 GPU 正在工作。** 要验证真实的执行模式，执行：

```powershell
waifu doctor --voice-test
```

`--voice-test` 会绕过 WAV 缓存实际初始化 VOICEVOX、合成一条日文测试语音（不播放声音），并调用官方 C API 的 `voicevox_synthesizer_is_gpu_mode` 反馈 GPU 或 CPU。该操作可能需要加载模型并等待一段时间；仅支持 Windows 版 VOICEVOX 后端，CeVIO 不使用这个选项。

普通检查发现**可选语音资源缺失**仅报告提醒，不会让核心命令失效；`waifu.json` 错误或明确要求的语音合成测试失败时会返回非零退出码。CPU 合成成功视为合成有效，但报告提醒 GPU 未启用。

## 常用命令

Ping 域名或 IP：

```powershell
waifu ping example.com --count 4
waifu ping 1.1.1.1 --miao
```

加载 `waifu-shell.ps1` 后，可以使用包装命令：

```powershell
cd "D:\path with spaces"
cd .\project -Miao

ls
ls .\src

cat .\Cargo.toml
cat ".\path with spaces\note.txt" -Miao

echo hello world
echo -Message '保留 "引号" 的文本'

clear

ps
ps -Top 20 -Sort mem
ps -Top 5 -Sort cpu -Miao
```

原生进程命令仍可直接使用：

```powershell
Get-Process
Get-Process -Name pwsh
```

`cd`、`ls` 和 `ps` 的 Rust 子命令会在 stdout 返回供 PowerShell 包装解析的机器结果。需要完整交互行为时，应优先使用加载脚本后的 `cd`、`ls` 和 `ps`。

## 检查

```powershell
cargo check --locked
cargo test --locked
```
