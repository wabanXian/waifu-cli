[CmdletBinding()]
param(
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'Programs\waifu-cli'),
    [switch]$NoProfile,
    [switch]$NoPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$source = $PSScriptRoot
$binary = Join-Path $source 'waifu.exe'
if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    $binary = Join-Path $source 'target\release\waifu.exe'
}
if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    throw '找不到 waifu.exe：请解压 Windows Release ZIP，或先 cargo build --release --locked'
}
$json = if (Test-Path (Join-Path $source 'waifu.json')) {
    Join-Path $source 'waifu.json'
} else {
    Join-Path $source 'target\release\waifu.json'
}
if (-not (Test-Path -LiteralPath $json -PathType Leaf)) {
    throw '找不到 waifu.json，请确认构建产物完整'
}

$required = @('waifu-shell.ps1','waifu-completion.ps1','update.ps1','uninstall.ps1')
foreach ($file in $required) {
    if (-not (Test-Path -LiteralPath (Join-Path $source $file) -PathType Leaf)) {
        throw "安装资源缺失：$file"
    }
}

$InstallDir = [System.IO.Path]::GetFullPath($InstallDir)
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$files = @(
    @{Source=$binary; Name='waifu.exe'},
    @{Source=$json; Name='waifu.json'}
)
foreach ($name in @('waifu-shell.ps1','waifu-completion.ps1','update.ps1','uninstall.ps1','install.ps1','README.md','speak.py')) {
    $from = Join-Path $source $name
    if (Test-Path -LiteralPath $from -PathType Leaf) {
        $files += @{Source=$from; Name=$name}
    }
}
foreach ($file in $files) {
    $destination = Join-Path $InstallDir $file.Name
    if ([System.IO.Path]::GetFullPath($file.Source) -ine [System.IO.Path]::GetFullPath($destination)) {
        Copy-Item -LiteralPath $file.Source -Destination $destination -Force
    }
}

if (-not $NoPath) {
    $oldPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $entries = @($oldPath -split ';' | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
    $isInstalled = @($entries | Where-Object { $_.TrimEnd('\') -ieq $InstallDir.TrimEnd('\') }).Count -gt 0
    if (-not $isInstalled) {
        $newPath = (@($entries) + $InstallDir) -join ';'
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
    }
    $localEntries = @($env:Path -split ';')
    if (-not @($localEntries | Where-Object { $_.TrimEnd('\') -ieq $InstallDir.TrimEnd('\') }).Count) {
        $env:Path = "$InstallDir;$env:Path"
    }
}

if (-not $NoProfile) {
    $profilePath = $PROFILE.CurrentUserCurrentHost
    $profileDir = Split-Path -Parent $profilePath
    if (-not (Test-Path -LiteralPath $profileDir -PathType Container)) {
        New-Item -ItemType Directory -Force -Path $profileDir | Out-Null
    }
    $contents = if (Test-Path -LiteralPath $profilePath) {
        Get-Content -LiteralPath $profilePath -Raw
    } else { '' }

    $begin = '# >>> waifu-cli managed >>>'
    $end = '# <<< waifu-cli managed <<<'
    $quotedScript = (Join-Path $InstallDir 'waifu-shell.ps1').Replace("'", "''")
    $block = "$begin" + [Environment]::NewLine +
        ". '$quotedScript'" + [Environment]::NewLine +
        "$end"
    $expression = '(?ms)^# >>> waifu-cli managed >>>[ \t]*\r?\n.*?^# <<< waifu-cli managed <<<[ \t]*(?:\r?\n)?'
    if ($contents -match $expression) {
        $updated = [regex]::Replace($contents, $expression, $block + [Environment]::NewLine)
    } else {
        $separator = if ($contents.Length -gt 0 -and -not $contents.EndsWith([string][char]10)) { [Environment]::NewLine } else { '' }
        $updated = $contents + $separator + $block + [Environment]::NewLine
    }
    if ($updated -cne $contents) {
        if (Test-Path -LiteralPath $profilePath) {
            $backup = "$profilePath.bak-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
            Copy-Item -LiteralPath $profilePath -Destination $backup
            Write-Host "🐾 已备份 PowerShell Profile：$backup"
        }
        [System.IO.File]::WriteAllText($profilePath, $updated, [System.Text.UTF8Encoding]::new($true))
    }
}
Write-Host "🌸 waifu-cli 安装完成：$InstallDir"
Write-Host "下次打开 PowerShell 将自动加载 waifu-shell.ps1；当前窗口可执行："
Write-Host ". '$((Join-Path $InstallDir 'waifu-shell.ps1').Replace("'", "''"))'"
Write-Host "语音模型不包含在发行包内；若模型留在原项目根目录，请设置 WAIFU_PROJECT_ROOT。"
