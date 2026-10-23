$waifuScriptPath = if ($PSCommandPath) {
    $PSCommandPath
} else {
    $MyInvocation.MyCommand.Path
}
$Script:WaifuProjectRoot = if ($waifuScriptPath) {
    Split-Path -Parent $waifuScriptPath
} else {
    $null
}

function Resolve-WaifuExecutable {
    if ($Script:WaifuProjectRoot) {
        $candidates = @(
            (Join-Path $Script:WaifuProjectRoot 'target\release\waifu.exe'),
            (Join-Path $Script:WaifuProjectRoot 'target\debug\waifu.exe')
        )
        foreach ($candidate in $candidates) {
            if (Test-Path -LiteralPath $candidate -PathType Leaf) {
                return $candidate
            }
        }
    }

    $command = Get-Command waifu -CommandType Application -ErrorAction SilentlyContinue |
        Select-Object -First 1
    if ($command) {
        return $command.Source
    }

    return $null
}

function Get-WaifuExecutable {
    if (-not $Script:WaifuExe -or
        -not (Test-Path -LiteralPath $Script:WaifuExe -PathType Leaf)) {
        $Script:WaifuExe = Resolve-WaifuExecutable
    }

    if (-not $Script:WaifuExe) {
        Write-Error 'waifu.exe not found. Run cargo build --release or check project path.'
        return $null
    }

    return $Script:WaifuExe
}

$Script:WaifuExe = Resolve-WaifuExecutable

function waifu {
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    & $waifuExe @args
}

Remove-Item Alias:cd -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ls -Force -ErrorAction SilentlyContinue
Remove-Item Alias:cat -Force -ErrorAction SilentlyContinue
Remove-Item Alias:echo -Force -ErrorAction SilentlyContinue
Remove-Item Alias:clear -Force -ErrorAction SilentlyContinue
Remove-Item Alias:cls -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ps -Force -ErrorAction SilentlyContinue

function Invoke-WaifuCd {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true, Position = 0)]
        [string]$Path,
        [switch]$Miao
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    $waifuArgs = @('cd')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += @('--', $Path)

    $output = @(& $waifuExe @waifuArgs)
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        return
    }

    if ($output.Count -ne 1 -or [string]::IsNullOrWhiteSpace([string]$output[0])) {
        Write-Error "waifu cd 没有返回唯一有效的目标路径"
        return
    }

    $target = [string]$output[0]
    if (-not (Test-Path -LiteralPath $target -PathType Container)) {
        Write-Error "waifu cd 返回的目标目录无效：$target"
        return
    }

    Set-Location -LiteralPath $target
}

function Invoke-WaifuLs {
    [CmdletBinding()]
    param(
        [Parameter(Position = 0)]
        [string]$Path = '.',
        [switch]$Miao
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    $waifuArgs = @('ls')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += @('--', $Path)

    $output = @(& $waifuExe @waifuArgs)
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        return
    }
    if ($output.Count -ne 1 -or [string]::IsNullOrWhiteSpace([string]$output[0])) {
        Write-Error "waifu ls 没有返回唯一有效的目标路径"
        return
    }

    $target = [string]$output[0]
    if (-not (Test-Path -LiteralPath $target -PathType Container)) {
        Write-Error "waifu ls 返回的目标目录无效：$target"
        return
    }

    $entries = @(Get-ChildItem -LiteralPath $target)

    # 🐱 喵喵标题头部
    Write-Host ("{0,-6} {1,-6} {2,-30} {3,-16} {4,10}" -f "喵权", "喵类", "名字喵", "最后摸摸", "胖胖程度") -ForegroundColor Magenta

    foreach ($entry in $entries) {
        # 🐾 权限翻译为喵语
        $perm = if ($entry.PSIsContainer) {
            "ฅ喵~"
        } elseif ($entry.Attributes.ToString().Contains("ReadOnly")) {
            "喵……"
        } else {
            "喵呜~"
        }

        # 📁 or 📄 类型标识
        $type = if ($entry.PSIsContainer) { "📁" } else { "📄" }

        $name = $entry.Name
        $time = $entry.LastWriteTime.ToString("yyyy-MM-dd HH:mm")
        $size = if ($entry.PSIsContainer) { "" } else { "{0,8} B" -f $entry.Length }

        Write-Host ("{0,-6} {1,-6} {2,-30} {3,-16} {4,10}" -f $perm, $type, $name, $time, $size)
    }

    Write-Host "`n(=^･ω･^=)ﾉ喵~ 总共有 $($entries.Count) 个文件宝宝在等你~" -ForegroundColor Cyan
}

function Invoke-WaifuCat {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true, Position = 0)]
        [string]$Path,
        [switch]$Miao
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    $waifuArgs = @('cat')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += @('--', $Path)
    & $waifuExe @waifuArgs
}

function Invoke-WaifuEcho {
    [CmdletBinding()]
    param(
        [switch]$Miao,
        [Parameter(Position = 0, ValueFromRemainingArguments = $true)]
        [string[]]$Message
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    $waifuArgs = @('echo')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += '--'
    $waifuArgs += $Message
    & $waifuExe @waifuArgs
}

function Invoke-WaifuClear {
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    & $waifuExe clear
}

function Invoke-WaifuCls {
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }
    & $waifuExe clear
}

# 🐾 公用渲染：根据 Top/Sort，用原生 Get-Process 获取数据并喵化输出
function Invoke-WaifuProcessList {
    param(
        [int]$Top = 10,
        [ValidateSet('cpu','mem','memory')]
        [string]$Sort = 'cpu'
    )

    # 显式调用 PowerShell 原生 Get-Process 获取进程对象
    $procs = Microsoft.PowerShell.Management\Get-Process |
             Select-Object Id, ProcessName, CPU, WorkingSet

    if (-not $procs) {
        Write-Host "喵呜……居然没拿到任何进程……" -ForegroundColor Red
        return
    }

    switch ($Sort) {
        'mem'    { $procs = $procs | Sort-Object -Property WorkingSet -Descending }
        'memory' { $procs = $procs | Sort-Object -Property WorkingSet -Descending }
        default  { $procs = $procs | Sort-Object -Property CPU        -Descending }
    }

    $procs = $procs | Select-Object -First $Top

    # 表头（注意：PowerShell 的 -f 不支持 '>' 对齐符，用正数宽度即右对齐）
    Write-Host ("{0,-8} {1,-30} {2,8} {3,10}" -f "喵PID","名字喵","CPU秒","内存MiB") -ForegroundColor Cyan

    foreach ($p in $procs) {
        $memMiB = [math]::Round(($p.WorkingSet / 1MB), 1)
        $cpuSec = if ($p.CPU) { "{0:N1}" -f $p.CPU } else { "0.0" }
        $icon   = if ($p.ProcessName -match '\.exe$') { "📄" } else { "🐾" }

        Write-Host ("{0,-8} {1,-30} {2,8} {3,10}" -f $p.Id,
            "$icon $($p.ProcessName)", $cpuSec, $memMiB)
    }
}

# =========================================================
# ps：先让 waifu 撒娇并返回参数，再渲染
# waifu 化进程展示使用 ps；原生进程命令直接使用 Get-Process
# 例如：waifu ps -t 20 -s mem --miao
# =========================================================
function Invoke-WaifuPs {
    [CmdletBinding()]
    param(
        [int]$Top = 10,
        [ValidateSet('cpu','mem','memory')]
        [string]$Sort = 'cpu',
        [switch]$Miao,

        # 让你也可以直接传原生短参：-t 20 -s mem
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Args
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuExe = Get-WaifuExecutable
    if (-not $waifuExe) { return }

    $waifuArgs = @('ps')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    if ($PSBoundParameters.ContainsKey('Top') -or $PSBoundParameters.ContainsKey('Sort')) {
        $waifuArgs += @('--top', [string]$Top, '--sort', $Sort)
    } else {
        $waifuArgs += $Args
    }

    $output = @(& $waifuExe @waifuArgs)
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        return
    }
    if ($output.Count -ne 1 -or [string]::IsNullOrWhiteSpace([string]$output[0])) {
        Write-Error "waifu ps 没有返回唯一有效的参数结果"
        return
    }

    try {
        $result = [string]$output[0] | ConvertFrom-Json -ErrorAction Stop
        $resultTop = [int]$result.top
        $resultSort = [string]$result.sort
    } catch {
        Write-Error "无法解析 waifu ps 返回的参数：$_"
        return
    }

    Invoke-WaifuProcessList -Top $resultTop -Sort $resultSort
}

# Use -Raw (or -raw) to forward arguments to the original PowerShell command.
# These public wrappers intentionally omit [CmdletBinding()] so that @args
# retains native named parameters such as -Force, -Tail, and -Name.
# Without -Raw, the existing waifu implementations stay unchanged.

function cd {
    param([switch]$Raw)

    if ($Raw) {
        Microsoft.PowerShell.Management\Set-Location @args
    } else {
        Invoke-WaifuCd @args
    }
}

function ls {
    param([switch]$Raw)

    if ($Raw) {
        Microsoft.PowerShell.Management\Get-ChildItem @args
    } else {
        Invoke-WaifuLs @args
    }
}

function cat {
    param([switch]$Raw)

    if ($Raw) {
        Microsoft.PowerShell.Management\Get-Content @args
    } else {
        Invoke-WaifuCat @args
    }
}

function echo {
    param([switch]$Raw)

    if ($Raw) {
        Microsoft.PowerShell.Utility\Write-Output @args
    } else {
        Invoke-WaifuEcho @args
    }
}

function clear {
    param([switch]$Raw)

    if ($Raw) {
        Clear-Host @args
    } else {
        Invoke-WaifuClear @args
    }
}

function cls {
    param([switch]$Raw)

    if ($Raw) {
        Clear-Host @args
    } else {
        Invoke-WaifuCls @args
    }
}

function ps {
    param([switch]$Raw)

    if ($Raw) {
        Microsoft.PowerShell.Management\Get-Process @args
    } else {
        Invoke-WaifuPs @args
    }
}
