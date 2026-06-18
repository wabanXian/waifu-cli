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

`--miao` 会调用 `speak.py` 播放日语台词。该功能是可选的，需要：

- Python，并可通过 `python` 命令启动
- `pythonnet`（提供 `clr` 模块）
- CeVIO AI，以及默认安装位置中的 `CeVIO.Talk.RemoteService2.dll`
- CeVIO 角色“さとうささら”

语音依赖缺失或播放失败时只会报告错误，不会使主要命令失败。未安装这些依赖时不要传递 `--miao` 或 PowerShell 的 `-Miao` 即可。

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
