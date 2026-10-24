# waifu-cli 🐾

**v0.1.1 · Windows x64 · Rust / PowerShell**

**English** | [简体中文](README.md)

A playful Windows command-line toolbox with colorful catgirl-style messages and optional Japanese speech. `waifu.exe` includes utilities for `ping`, `cd`, `ls`, `cat`, `echo`, `clear`, `ps`, and read-only Git summaries. An optional PowerShell wrapper makes them easier to use interactively, while `-Raw` preserves access to the native PowerShell commands.

**[Download v0.1.1 (Windows ZIP)](https://github.com/wabanXian/waifu-cli/releases/tag/v0.1.1)** · [All releases](https://github.com/wabanXian/waifu-cli/releases) · [Windows CI](https://github.com/wabanXian/waifu-cli/actions/workflows/ci.yml) · [Roadmap (#7)](https://github.com/wabanXian/waifu-cli/issues/7)

> **Platform:** Windows x64. The program's messages are primarily in Chinese and its optional speech is in Japanese; this English guide does not change the application's UI language.

## Quick start — no Rust required

1. Download `waifu-windows-x64.zip` from the [v0.1.1 release](https://github.com/wabanXian/waifu-cli/releases/tag/v0.1.1), then extract it.
2. Open PowerShell in the extracted directory and run:

```powershell
.\waifu.exe --version
.\waifu.exe doctor
.\install.ps1
```

3. Open a new PowerShell window and try:

```powershell
waifu git status
waifu git log --count 5
waifu ping 1.1.1.1 --count 2
waifu doctor
```

The installer does **not** require administrator privileges. It installs to `%LOCALAPPDATA%\Programs\waifu-cli`, adds that directory to your *user* PATH, and configures your current PowerShell host's profile (backing up the profile before changing it).

To install without editing your profile or PATH:

```powershell
.\install.ps1 -NoProfile -NoPath
```

**Speech is optional.** The lightweight release ZIP does **not** include VOICEVOX CORE, voice models, the Japanese dictionary, ONNX Runtime, or CUDA/cuDNN DLLs. All non-speech commands work without those components.

### Upgrade from v0.1.0

If you previously used the installer, you can update in place:

```powershell
waifu update
```

The updater downloads the latest Release ZIP and SHA-256 manifest, validates the archive, and replaces the installed files. Open a new PowerShell session afterward:

```powershell
waifu --version  # expected: waifu 0.1.1
waifu doctor
```

**v0.1.1** is a maintenance release that bundles both README languages and the MIT `LICENSE` with the Windows package. It does not introduce new CLI subcommands. The original v0.1.0 release artifacts remain unchanged.

## Commands

| Command | What it does |
| --- | --- |
| `waifu ping example.com --count 4` | Ping a host with colorful responses |
| `waifu ping 1.1.1.1 --miao` | Ping with optional Japanese voice |
| `waifu doctor` | Inspect the local environment without starting the synthesizer |
| `waifu doctor --voice-test` | Synthesize an uncached test and report actual GPU/CPU mode |
| `waifu git status` | Display Git branch and working-tree changes |
| `waifu git log --count 5` | Summarize recent commits, read-only |
| `waifu update` | Update an installed copy from the latest GitHub release |

The Git helper invokes Git read-only; it never runs `git add`, `git commit`, `git push`, or `git reset` for you.

After sourcing the PowerShell wrapper, you can also use:

```powershell
cd "D:\path with spaces"
cd .\project -Miao

ls
ls .\src

cat .\Cargo.toml
cat ".\path with spaces\note.txt" -Miao

echo hello world
clear

ps
ps -Top 20 -Sort mem
ps -Top 5 -Sort cpu -Miao
```

The wrapper defines `cd`, `ls`, `cat`, `echo`, `clear`, `cls`, and `ps` functions. They are *not* drop-in substitutes for every possible native PowerShell parameter. Use `-Raw` to forward arguments to the original command, for example:

```powershell
ls -Raw -Force
cat -Raw .\log.txt -Tail 20
ps -Raw -Name pwsh

# The original PowerShell command is always accessible directly:
Get-Process -Name pwsh
```

The Rust `cd`, `ls`, and `ps` subcommands intentionally emit machine-readable results for the PowerShell wrapper. For interactive behavior, use the wrapper after loading `waifu-shell.ps1`.

### PowerShell Tab completion

The wrapper loads `waifu-completion.ps1` when available. Tab completion supports common subcommands and options, Git subcommands, `doctor --voice-test`, and some paths. It registers completion for `waifu` and `waifu.exe` only, without replacing other PowerShell completions.

## Build from source

Install the stable Rust toolchain, clone the repository, then run:

```powershell
git clone https://github.com/wabanXian/waifu-cli.git
cd waifu-cli

cargo build --locked           # Debug build
cargo build --release --locked # Optimized build
```

The executable is written to `target\debug\waifu.exe` or `target\release\waifu.exe`. The build script copies `src\waifu.json` and `speak.py` next to the executable; keep these files alongside `waifu.exe`.

For a source checkout, add the build output to the current session's PATH and load the wrapper:

```powershell
$waifuDir = (Resolve-Path .\target\release).Path
$env:PATH = "$waifuDir;$env:PATH"
. .\waifu-shell.ps1
```

To load it automatically in future sessions, use `.\install.ps1` after building, or add an absolute-path dot-source line to your own PowerShell profile:

```powershell
. "D:\path\to\waifu-cli\waifu-shell.ps1"
```

## Install, update, and uninstall

You can run `.\install.ps1` either from the extracted official release ZIP or from a source checkout after building the release executable. Re-running the installer is designed not to duplicate its managed PowerShell profile block.

### Update

```powershell
waifu update
```

`waifu update` starts a separate PowerShell process to avoid replacing an executable while it is still running. The updater downloads the latest published GitHub Release, validates the ZIP against its `SHA256SUMS.txt`, and installs the updated files.

For an **offline update**, download both files from the same release and run:

```powershell
& "$env:LOCALAPPDATA\Programs\waifu-cli\update.ps1" `
    -PackagePath 'D:\Downloads\waifu-windows-x64.zip' `
    -ChecksumPath 'D:\Downloads\SHA256SUMS.txt'
```

### Uninstall

```powershell
# Preview the removal first
& "$env:LOCALAPPDATA\Programs\waifu-cli\uninstall.ps1" -WhatIf

# Uninstall
& "$env:LOCALAPPDATA\Programs\waifu-cli\uninstall.ps1"
```

The uninstaller removes the managed profile block, the specific user PATH entry, and known installed app files. It does **not** recursively remove arbitrary user files, voice models, or the speech cache.

## Optional Japanese speech with VOICEVOX CORE

Add `--miao` (or `-Miao` in the PowerShell wrapper) to supported commands to synthesize Japanese lines from `waifu.json`. The default backend directly loads the **VOICEVOX CORE Windows x64 C API** from Rust. It does not require a Python VOICEVOX wheel or a running VOICEVOX editor/HTTP engine.

### Required files and directories

Obtain the Windows x64 C API package using the [official VOICEVOX CORE documentation](https://github.com/VOICEVOX/voicevox_core), and extract it into your project/resource root. Keep the versioned folder name; for example:

```text
waifu-cli/
├── voicevox_core-windows-x64-0.17.1/   # Example, not a fixed version
│   └── lib/
│       └── voicevox_core.dll
└── voicevox_core/
    ├── onnxruntime/
    │   └── lib/...
    ├── dict/
    │   └── open_jtalk_dic_utf_8-1.11/...
    └── models/
        └── vvms/
            └── 0.vvm
```

The program scans `voicevox_core-windows-x64-*` directories and can use DLL layouts at `lib/voicevox_core.dll`, `c_api/lib/voicevox_core.dll`, or the package root. It searches for voice models and supporting assets in the versioned bundle and/or the sibling `voicevox_core` directory.

When using the installed `waifu.exe` while keeping voice assets in your source checkout, set the resource root explicitly:

```powershell
[Environment]::SetEnvironmentVariable(
    'WAIFU_PROJECT_ROOT',
    'D:\path\to\waifu-cli',
    'User'
)
# Open a new PowerShell window to pick up the setting.
```

To select a specific voice style or model:

```powershell
$env:WAIFU_VOICEVOX_STYLE_ID = '0'
$env:WAIFU_VOICEVOX_MODEL = 'voicevox_core\models\vvms\0.vvm'

waifu ping 1.1.1.1 --miao
```

The model path may be absolute or relative to the resource root. Without an override, the application discovers available `.vvm` files and defaults to Style ID 0. Make sure the selected model supports that style.

New lines can take longer to synthesize while models initialize. Subsequent playback of the same line can use cached WAV files stored under `%LOCALAPPDATA%\waifu-cli\voice-cache`. Set `WAIFU_VOICE_CACHE_DIR` to change the cache directory. Playback is currently blocking; speech failures should not prevent the main CLI command from running.

Please respect the licensing and usage terms of each voice model and character, particularly when redistributing models or synthesized audio.

### NVIDIA CUDA acceleration

VOICEVOX may detect an NVIDIA GPU but still use CPU if the loaded ONNX Runtime does not support CUDA. The diagnostic log:

```text
CUDA (device_id=0): 現在ロードされているONNX Runtimeでは利用できません
```

means the **loaded ONNX Runtime cannot use CUDA**. It is not, by itself, evidence of a broken GPU driver.

Download the CUDA ONNX Runtime and additional libraries using the [official VOICEVOX Downloader guide](https://github.com/VOICEVOX/voicevox_core/blob/main/docs/guide/user/downloader.md). If you already have the official `download.exe`:

```powershell
# Optional: authenticate if GitHub's anonymous API rate limit is reached
$env:GH_TOKEN = gh auth token

.\download.exe --devices cuda --only onnxruntime additional-libraries -o .\voicevox_core

Remove-Item Env:GH_TOKEN
```

The application prioritizes `voicevox_core\onnxruntime\lib` and registers the additional-library directories in the process's Windows DLL search path, without globally changing PATH.

You can override the runtime directory:

```powershell
$env:WAIFU_VOICEVOX_ORT_DIR = 'D:\path\to\waifu-cli\voicevox_core\onnxruntime\lib'
```

If you prefer manual/offline downloads, these **official** files are an example of a compatible version family for ONNX Runtime 1.17.x:

- [VOICEVOX ONNX Runtime 1.17.3 — Windows x64 CUDA (`.tgz`)](https://github.com/VOICEVOX/onnxruntime-builder/releases/download/voicevox_onnxruntime-1.17.3/voicevox_onnxruntime-win-x64-cuda-1.17.3.tgz)
- [VOICEVOX CUDA additional libraries 0.2.1 — Windows x64 (`.zip`)](https://github.com/VOICEVOX/voicevox_additional_libraries/releases/download/0.2.1/CUDA-windows-x64.zip)

Extract the necessary DLL files directly into `voicevox_core\onnxruntime\lib` and `voicevox_core\additional_libraries` respectively (avoid accidental extra nested directories). For cuDNN 8.x, a missing `zlibwapi.dll` may require installing the correct **64-bit Zlib DLL** separately; see the [NVIDIA cuDNN Windows installation guide](https://docs.nvidia.com/deeplearning/cudnn/archives/cudnn-890/install-guide/index.html#install-zlib-windows). Avoid untrusted DLL download websites. CUDA, cuDNN, the GPU driver, and ONNX Runtime must be mutually compatible.

**To verify actual GPU use, prefer:**

```powershell
waifu doctor --voice-test
```

This synthesizes a fresh test line without playing it or using the WAV cache, then queries the synthesizer's real GPU/CPU execution mode through the VOICEVOX C API. Replaying a previously cached phrase may produce no synthesis logs at all.

### Legacy CeVIO backend

An optional compatibility mode can still use CeVIO through the adjacent `speak.py` script. It requires Python, `pythonnet`, and a running/configured CeVIO Host:

```powershell
$env:WAIFU_TTS_BACKEND = 'cevio'
waifu ping 1.1.1.1 --miao

# Back to native VOICEVOX
$env:WAIFU_TTS_BACKEND = 'voicevox'
```

## Troubleshooting with `waifu doctor`

```powershell
waifu doctor
```

By default, the doctor performs lightweight file/configuration checks **without loading the synthesizer**. It inspects the executable, `waifu.json`, the PowerShell wrapper, selected speech backend, C API location, Japanese dictionary, model, and ONNX Runtime directories when available. It cannot definitively tell whether the current interactive PowerShell session has sourced the wrapper.

```powershell
waifu doctor --voice-test
```

The optional test initializes VOICEVOX, synthesizes an uncached Japanese sample without playing it, and reports actual CPU/GPU mode. A CPU success is still a successful synthesis, but triggers a GPU-related warning. The flag is only supported for the native Windows VOICEVOX backend, not legacy CeVIO.

Missing optional voice resources are warnings for the default doctor command, not blockers for the basic CLI.

## License

The original project source code and documentation are available under the **[MIT License](LICENSE)**. You may use, modify, and redistribute the project, including commercially, provided that the copyright and license notices are preserved. The software is provided *as is*, without warranty. **Copyright (c) 2026 wabanXian.**

**Third-party components and content are not relicensed under this MIT License.** VOICEVOX CORE, voice models/characters, ONNX Runtime, CUDA/cuDNN, CeVIO, and Rust/Python dependencies have their own licenses or terms. The release archive does not include VOICEVOX models or CUDA runtime assets.

## CI, releases, and roadmap

Windows GitHub Actions run `cargo check --locked`, `cargo test --locked`, Clippy, release builds, and PowerShell installer/completion checks. The release workflow builds a Windows x64 ZIP and accompanying SHA-256 checksum file. It is triggered by `v*` tags and also supports explicit `release: vX.Y.Z` commits on `master`.

```powershell
cargo check --locked
cargo test --locked
cargo clippy --locked --all-targets
```

**v0.1.1** is the first public Windows x64 release. VOICEVOX binaries and models remain separate downloads subject to their own licensing terms.

The future voice/style selector, persistent voice preferences, non-blocking speech, and mood/personality features are tracked in [Issue #7](https://github.com/wabanXian/waifu-cli/issues/7); they are **not** included in v0.1.1.
