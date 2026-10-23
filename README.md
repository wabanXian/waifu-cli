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

`--miao` 和 PowerShell 的 `-Miao` 通过 `speak.py` 播放对应的日语台词，中文台词仍在终端显示。语音后端可选，**不启用语音也能正常使用 CLI**。

### VOICEVOX CORE（无需常驻编辑器）

`VOICEVOX CORE` 使用官方 Python API（wheel）直接在命令调用时合成并播放日语语音，不需要启动 VOICEVOX 编辑器或 HTTP 引擎。

请按 [VOICEVOX CORE 官方指南](https://github.com/VOICEVOX/voicevox_core/blob/main/docs/guide/user/usage.md) 安装**匹配 Python、系统架构的 wheel**，并下载配套的 VOICEVOX ONNX Runtime、Open JTalk 词典和 `.vvm` 声音模型。仅下载 C API DLL 不代表已经安装 Python API。

下载资源例如放在：

```text
D:\VoicevoxCore\
├── onnxruntime\lib\...
├── dict\open_jtalk_dic_utf_8-1.11\...
└── models\vvms\0.vvm
```

PowerShell 中检查 Python API：

```powershell
python -c "from voicevox_core.blocking import Onnxruntime, OpenJtalk, Synthesizer, VoiceModelFile; print('VOICEVOX Python API OK')"
```

然后配置当前终端会话（目录请改成你实际安装的位置）：

```powershell
$env:WAIFU_TTS_BACKEND = 'voicevox'
$env:WAIFU_VOICEVOX_HOME = 'D:\VoicevoxCore'
$env:WAIFU_VOICEVOX_STYLE_ID = '0'

# 仅检查依赖和路径，不合成语音
python .\target\release\speak.py --check

# 测试发声
python .\target\release\speak.py 'こんにちは、ご主人様！'

# 原有命令无需改动
waifu ping 1.1.1.1 --miao
```

若资源不在默认位置，可使用 `WAIFU_VOICEVOX_MODEL` 指定 `.vvm` 文件路径。声音风格 ID 必须属于加载的模型；示例 `0` 不一定适用于你下载的每个模型。

第一次播放新台词时需加载模型并进行合成，速度可能较慢。后续相同台词与模型组合会直接播放 `%LOCALAPPDATA%\waifu-cli\voice-cache` 下的 WAV 缓存，无需再次初始化 CORE。可用 `WAIFU_VOICE_CACHE_DIR` 自定义缓存目录。当前音频播放使用 Windows 自带的 `winsound`。

`python` 必须指向安装了 `voicevox_core` wheel 的那个 Python 环境；如果你使用虚拟环境，请在激活后启动 CLI。脚本不会自动下载大型模型或把它们打包进 `waifu.exe`，发行时请遵守对应声音模型与角色的使用条款。

### 原 CeVIO 后端

默认仍是 `cevio`，以兼容已有安装：

```powershell
$env:WAIFU_TTS_BACKEND = 'cevio'
```

原 CeVIO 模式仍要求 Python、`pythonnet`、CeVIO AI 及 “さとうささら” 声音资源。调用时如未启动 Host，脚本会尝试启动。音频播放期间 CLI 命令会等待播放结束；语音失败不会改变原始 CLI 命令的执行结果。

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
