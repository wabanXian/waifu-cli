Remove-Item Alias:cd -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ls -Force -ErrorAction SilentlyContinue
Remove-Item Alias:cat -Force -ErrorAction SilentlyContinue
Remove-Item Alias:echo -Force -ErrorAction SilentlyContinue
Remove-Item Alias:clear -Force -ErrorAction SilentlyContinue
Remove-Item Alias:cls -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ps -Force -ErrorAction SilentlyContinue

function cd {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true, Position = 0)]
        [string]$Path,
        [switch]$Miao
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuArgs = @('cd')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += @('--', $Path)

    $output = @(& waifu @waifuArgs)
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

function ls {
    [CmdletBinding()]
    param(
        [Parameter(Position = 0)]
        [string]$Path = '.',
        [switch]$Miao
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuArgs = @('ls')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += @('--', $Path)

    $output = @(& waifu @waifuArgs)
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

function cat {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true, Position = 0)]
        [string]$Path,
        [switch]$Miao
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuArgs = @('cat')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += @('--', $Path)
    & waifu @waifuArgs
}

function echo {
    [CmdletBinding()]
    param(
        [switch]$Miao,
        [Parameter(Position = 0, ValueFromRemainingArguments = $true)]
        [string[]]$Message
    )

    $OutputEncoding = [System.Text.Encoding]::UTF8
    $waifuArgs = @('echo')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    $waifuArgs += '--'
    $waifuArgs += $Message
    & waifu @waifuArgs
}

function clear {
    waifu clear
}

function cls {
    waifu clear
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
function ps {
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

    $waifuArgs = @('ps')
    if ($Miao) {
        $waifuArgs += '--miao'
    }
    if ($PSBoundParameters.ContainsKey('Top') -or $PSBoundParameters.ContainsKey('Sort')) {
        $waifuArgs += @('--top', [string]$Top, '--sort', $Sort)
    } else {
        $waifuArgs += $Args
    }

    $output = @(& waifu @waifuArgs)
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
