# Changelog

## v0.1.0 — 首次 Windows x64 正式发布

### 功能

- `waifu ping/cd/ls/cat/echo/clear/ps`：彩色、猫娘风格的 CLI 与 PowerShell 包装。
- `--miao`：在 Windows 上通过本地 VOICEVOX CORE C API 合成日语台词（可选资源）；CeVIO 兼容模式。
- VOICEVOX 模型 / ONNX Runtime 自动发现、WAV 缓存、可选 CUDA GPU 加速。
- `waifu doctor`：运行环境与依赖检查；`--voice-test` 执行真实的 CPU/GPU 合成验证。
- `waifu git status` / `waifu git log`：只读 Git 猫娘助手。
- PowerShell Tab 补全以及带 `-Raw` 原生命令转发的 Shell 包装。
- `install.ps1` / `uninstall.ps1` / `waifu update`：每用户安装、升级与卸载，附 SHA-256 校验。
- GitHub Actions Windows CI、Windows Release ZIP 和 `SHA256SUMS.txt`。

### 发布说明

- 平台：Windows x64。
- 下载 `waifu-windows-x64.zip` 后解压，执行 `.\waifu.exe doctor` 和 `.\install.ps1`。
- 语音模型、词典、ONNX Runtime 和 CUDA/cuDNN DLL **不包含**在发行 ZIP 中，需自行安装并遵守原始许可。
- 后续声线与人格系统见 [Issue #7](https://github.com/wabanXian/waifu-cli/issues/7)。
- 二进制发行物由 Windows GitHub Actions 从版本提交构建并附带 SHA-256 校验文件。
