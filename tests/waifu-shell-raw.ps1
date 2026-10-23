# Run with: pwsh -NoProfile -File .\tests\waifu-shell-raw.ps1
$ErrorActionPreference = 'Stop'

. (Join-Path (Split-Path $PSScriptRoot -Parent) 'waifu-shell.ps1')

# -Raw and -raw should both work; native parameters must be forwarded.
$files = @(ls -raw -LiteralPath $PSScriptRoot -Force)
if (-not ($files | Where-Object { $_.Name -eq 'waifu-shell-raw.ps1' })) {
    throw 'ls -raw did not return native FileInfo objects.'
}

$line = @(cat -Raw -LiteralPath $PSCommandPath -TotalCount 1)
if ($line.Count -ne 1 -or $line[0] -notlike '# Run with:*') {
    throw 'cat -Raw did not forward -TotalCount and -LiteralPath.'
}

$words = @(echo -Raw -InputObject @('hello', 'world'))
if ($words.Count -ne 2 -or $words[0] -ne 'hello' -or $words[1] -ne 'world') {
    throw 'echo -Raw did not preserve native Write-Output behavior.'
}

$proc = ps -Raw -Id $PID
if ($null -eq $proc -or $proc.Id -ne $PID) {
    throw 'ps -Raw did not forward the -Id parameter.'
}

$before = (Get-Location).Path
try {
    cd -Raw -LiteralPath $PSScriptRoot
    if ((Get-Location).Path -ne $PSScriptRoot) {
        throw 'cd -Raw did not invoke Set-Location.'
    }
}
finally {
    Microsoft.PowerShell.Management\Set-Location -LiteralPath $before
}

# Clear-Host is interactive and intentionally not invoked by this test.
Write-Host 'Raw mode smoke checks passed.' -ForegroundColor Green
