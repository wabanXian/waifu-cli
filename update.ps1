[CmdletBinding()]
param(
    [string]$PackagePath,
    [string]$ChecksumPath,
    [int]$WaitForProcessId = 0
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$installDir = [System.IO.Path]::GetFullPath($PSScriptRoot)

if (-not (Test-Path -LiteralPath (Join-Path $installDir 'waifu.exe') -PathType Leaf)) {
    throw 'update.ps1 只能从已安装的 waifu.exe 同级目录运行；源码仓库请先 git pull 再编译。'
}

$work = Join-Path ([System.IO.Path]::GetTempPath()) ("waifu-update-" + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force -Path $work | Out-Null
try {
    $zip = Join-Path $work 'waifu-windows-x64.zip'
    $sums = Join-Path $work 'SHA256SUMS.txt'

    if ($PackagePath) {
        if (-not $ChecksumPath) {
            throw '离线更新必须同时传入 -PackagePath ZIP 和 -ChecksumPath SHA256SUMS.txt'
        }
        Copy-Item -LiteralPath $PackagePath -Destination $zip
        Copy-Item -LiteralPath $ChecksumPath -Destination $sums
    } else {
        $headers = @{ 'User-Agent' = 'waifu-cli-update' }
        $release = Invoke-RestMethod -Uri 'https://api.github.com/repos/wabanXian/waifu-cli/releases/latest' -Headers $headers
        $zipAsset = @($release.assets | Where-Object { $_.name -eq 'waifu-windows-x64.zip' }) | Select-Object -First 1
        $sumAsset = @($release.assets | Where-Object { $_.name -eq 'SHA256SUMS.txt' }) | Select-Object -First 1
        if (-not $zipAsset -or -not $sumAsset) {
            throw '最新 GitHub Release 缺少 ZIP 或 SHA256SUMS.txt；请等待正式版本发布。'
        }
        Write-Host "🐾 正在下载 $($release.tag_name) ..."
        Invoke-WebRequest -UseBasicParsing -Uri $zipAsset.browser_download_url -OutFile $zip
        Invoke-WebRequest -UseBasicParsing -Uri $sumAsset.browser_download_url -OutFile $sums
    }

    # Check archive integrity before extracting or overwriting local files.
    $manifest = Get-Content -LiteralPath $sums -Raw
    $match = [regex]::Match($manifest, '(?im)^([a-f0-9]{64})[ \t]+waifu-windows-x64\.zip[ \t]*$')
    if (-not $match.Success) {
        throw 'SHA256SUMS.txt 格式不合法或缺少 waifu-windows-x64.zip 校验值'
    }
    $expected = $match.Groups[1].Value
    $actual = (Get-FileHash -LiteralPath $zip -Algorithm SHA256).Hash
    if ($actual -ine $expected) {
        throw 'SHA256 校验失败，下载文件可能损坏；未修改已安装程序'
    }

    $unpack = Join-Path $work 'package'
    Expand-Archive -LiteralPath $zip -DestinationPath $unpack -Force
    $required = @('waifu.exe','waifu.json','waifu-shell.ps1','waifu-completion.ps1',
        'install.ps1','uninstall.ps1','update.ps1')
    foreach ($file in $required) {
        if (-not (Test-Path -LiteralPath (Join-Path $unpack $file) -PathType Leaf)) {
            throw "下载包缺少 $file；未修改已安装程序"
        }
    }

    if ($WaitForProcessId -gt 0) {
        for ($i = 0; $i -lt 30; $i++) {
            $process = Get-Process -Id $WaitForProcessId -ErrorAction SilentlyContinue
            if (-not $process) { break }
            Start-Sleep -Seconds 1
        }
        if (Get-Process -Id $WaitForProcessId -ErrorAction SilentlyContinue) {
            throw 'waifu.exe 仍在运行，无法安全替换；请关闭旧进程后手动执行 update.ps1'
        }
    }

    $previous = Join-Path $installDir 'waifu.exe.previous'
    Copy-Item -LiteralPath (Join-Path $installDir 'waifu.exe') -Destination $previous -Force
    try {
        foreach ($file in @('waifu.exe','waifu.json','speak.py','waifu-shell.ps1',
                'waifu-completion.ps1','install.ps1','uninstall.ps1','update.ps1','README.md')) {
            $from = Join-Path $unpack $file
            if (Test-Path -LiteralPath $from -PathType Leaf) {
                Copy-Item -LiteralPath $from -Destination (Join-Path $installDir $file) -Force
            }
        }
    } catch {
        Copy-Item -LiteralPath $previous -Destination (Join-Path $installDir 'waifu.exe') -Force
        throw
    }
    Remove-Item -LiteralPath $previous -Force
    Write-Host '🌸 waifu-cli 更新完成！重新打开 PowerShell 即可使用新版本。'
} finally {
    Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue
}
