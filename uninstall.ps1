[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'Programs\waifu-cli'),
    [switch]$KeepFiles
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$InstallDir = [System.IO.Path]::GetFullPath($InstallDir)

# Remove only the exact marked block written by install.ps1.
$profilePath = $PROFILE.CurrentUserCurrentHost
if (Test-Path -LiteralPath $profilePath -PathType Leaf) {
    $current = Get-Content -LiteralPath $profilePath -Raw
    $expression = '(?ms)^# >>> waifu-cli managed >>>[ \t]*\r?\n.*?^# <<< waifu-cli managed <<<[ \t]*(?:\r?\n)?'
    $updated = [regex]::Replace($current, $expression, '')
    if ($updated -cne $current -and $PSCmdlet.ShouldProcess($profilePath, 'Remove managed waifu-cli Profile block')) {
        $backup = "$profilePath.bak-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
        Copy-Item -LiteralPath $profilePath -Destination $backup
        [System.IO.File]::WriteAllText($profilePath, $updated, [System.Text.UTF8Encoding]::new($true))
        Write-Host "🐾 Profile 已恢复（备份：$backup）"
    }
}

# Strip just the install folder; preserve every other user PATH entry.
$oldPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($oldPath) {
    $entries = @($oldPath -split ';' | Where-Object {
        $_ -and $_.TrimEnd('\') -ine $InstallDir.TrimEnd('\')
    })
    $newPath = $entries -join ';'
    if ($newPath -cne $oldPath -and $PSCmdlet.ShouldProcess('User PATH', "Remove $InstallDir")) {
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
    }
}

if (-not $KeepFiles -and (Test-Path -LiteralPath $InstallDir -PathType Container)) {
    # Do not delete the folder recursively: it may contain user models/configuration.
    foreach ($name in @('waifu.exe','waifu.json','speak.py','waifu-shell.ps1',
        'waifu-completion.ps1','install.ps1','uninstall.ps1','update.ps1','README.md','README_EN.md','LICENSE','CHANGELOG.md')) {
        $path = Join-Path $InstallDir $name
        if ((Test-Path -LiteralPath $path -PathType Leaf) -and
            $PSCmdlet.ShouldProcess($path, 'Delete installed waifu-cli file')) {
            Remove-Item -LiteralPath $path -Force
        }
    }
    if (@(Get-ChildItem -LiteralPath $InstallDir -Force).Count -eq 0) {
        if ($PSCmdlet.ShouldProcess($InstallDir, 'Remove empty install folder')) {
            Remove-Item -LiteralPath $InstallDir
        }
    }
}
Write-Host '🌸 已完成卸载操作；重新打开 PowerShell 后生效。'
Write-Host 'VOICEVOX 模型、WAV 缓存和其他用户文件不会被删除。'
