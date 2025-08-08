Remove-Item Alias:cd -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ls -Force -ErrorAction SilentlyContinue
Remove-Item Alias:cat -Force -ErrorAction SilentlyContinue
Remove-Item Alias:echo -Force -ErrorAction SilentlyContinue
Remove-Item Alias:clear -Force -ErrorAction SilentlyContinue
Remove-Item Alias:cls -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ps -Force -ErrorAction SilentlyContinue

function cd {
    $OutputEncoding = [System.Text.Encoding]::UTF8
    $argStr = ($args -join " ") -replace '"', ''
    # 直接 passthrough 调用 waifu，保持彩色 + 表情不变
    waifu cd "$argStr"
    # 从临时文件中读取路径
    $targetFile = "$env:TEMP\waifu_cd_path.txt"
    if (-not (Test-Path $targetFile)) {
        Write-Host "喵呜~ 没找到 waifu 留下的目标路径文件欸…是不是出错了？"
        return
    }

    $target = Get-Content $targetFile -Raw
    $target = $target.Trim()
    Remove-Item $targetFile -Force -ErrorAction SilentlyContinue

    if (Test-Path $target) {
        Set-Location $target
    } else {
        Write-Host "路径不存在喵：$target"
    }
}

# function ls {
#     $OutputEncoding = [System.Text.Encoding]::UTF8

#     # 先让 waifu 打个招呼
#     waifu ls

#     # 拿到 waifu 写的路径
#     $targetFile = "$env:TEMP\waifu_ls_path.txt"
#     if (-not (Test-Path $targetFile)) {
#         Write-Host "喵呜~ 找不到 waifu 留下的路径喵…不执行 ls 了！" -ForegroundColor Red
#         return
#     }

#     $target = Get-Content $targetFile -Raw
#     $target = $target.Trim()
#     Remove-Item $targetFile -Force -ErrorAction SilentlyContinue

#     if (Test-Path $target) {
#         # 真正列出内容（保留原始 ls 行为）
#         Get-ChildItem $target
#     } else {
#         Write-Host "路径不存在喵：$target"
#     }
# }
function ls {
    $OutputEncoding = [System.Text.Encoding]::UTF8
    if ($args) {
        waifu ls ($args -join " ") -replace '"', ''
    } else {
        waifu ls
    }

    $file = "$env:TEMP\waifu_ls_path.txt"
    if (Test-Path $file) {
        $target = Get-Content $file -Raw | ForEach-Object { $_.Trim() }
        Remove-Item $file -Force -ErrorAction SilentlyContinue

        if (Test-Path $target) {
            $entries = Get-ChildItem $target

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
        } else {
            Write-Host "路径不存在喵：$target"
        }
    } else {
        Write-Host "喵呜~ 找不到 waifu 留下的路径喵…不执行 ls 了！" -ForegroundColor Red
    }
}

function cat {
    $OutputEncoding = [System.Text.Encoding]::UTF8
    $argStr = ($args -join " ") -replace '"', ''
    waifu cat "$argStr"
}

function echo {
    $OutputEncoding = [System.Text.Encoding]::UTF8
    $argStr = ($args -join " ") -replace '"', ''
    waifu echo "$argStr"
}

function clear {
    waifu clear
}

function cls {
    waifu clear
}

# 🐾 公用渲染：从 waifu 的 TEMP 或参数来取 Top/Sort，然后用原生 Get-Process 获取数据并喵化输出
function Invoke-WaifuProcessList {
    param(
        [int]$Top = 10,
        [ValidateSet('cpu','mem','memory')]
        [string]$Sort = 'cpu',
        [switch]$FromWaifu  # 若为真，则优先读取 waifu 写入的 TEMP 参数
    )

    if ($FromWaifu) {
        $topFile  = "$env:TEMP\waifu_ps_top.txt"
        $sortFile = "$env:TEMP\waifu_ps_sort.txt"
        if (Test-Path $topFile)  { $Top  = [int](Get-Content $topFile -Raw).Trim() }
        if (Test-Path $sortFile) { $Sort = (Get-Content $sortFile -Raw).Trim().ToLower() }
        Remove-Item $topFile,$sortFile -Force -ErrorAction SilentlyContinue
    }

    # 用模块限定名，避免和我们自定义的 Get-Process 函数递归
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
# ps：沿用你的套路——先让 waifu 撒娇并写 TEMP，再渲染
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

    if ($PSBoundParameters.ContainsKey('Top') -or $PSBoundParameters.ContainsKey('Sort')) {
        # 用 -Top/-Sort 的情况：我们自己写临时参数（不依赖 clap 解析）
        Set-Content -Path "$env:TEMP\waifu_ps_top.txt"  -Value $Top
        Set-Content -Path "$env:TEMP\waifu_ps_sort.txt" -Value $Sort
        if ($Miao) {
            waifu ps --miao --top $Top --sort $Sort
        } else {
            waifu ps --top $Top --sort $Sort
        }
    } else {
        # 纯转发短参：-t / -s 交给 waifu（clap）解析
        if ($Miao) {
            waifu ps --miao @Args
        } else {
            waifu ps @Args
        }
    }

    Invoke-WaifuProcessList -FromWaifu
}

# =========================================================
# 覆盖 Get-Process：默认喵化（支持 -Top / -Sort），
# 若需要“原汁原味”，用 -Raw 或传原生参数（转发到真 cmdlet）
# 例：
#   Get-Process                 # 喵化
#   Get-Process -Top 15 -Sort mem
#   Get-Process -Raw notepad    # 原生通道
#   Get-Process -Raw -Id 1234
# =========================================================
function Get-Process {
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

    if ($PSBoundParameters.ContainsKey('Top') -or $PSBoundParameters.ContainsKey('Sort')) {
        # 用 -Top/-Sort 的情况：我们自己写临时参数（不依赖 clap 解析）
        Set-Content -Path "$env:TEMP\waifu_ps_top.txt"  -Value $Top
        Set-Content -Path "$env:TEMP\waifu_ps_sort.txt" -Value $Sort
        if ($Miao) {
            waifu ps --miao --top $Top --sort $Sort
        } else {
            waifu ps --top $Top --sort $Sort
        }
    } else {
        # 纯转发短参：-t / -s 交给 waifu（clap）解析
        if ($Miao) {
            waifu ps --miao @Args
        } else {
            waifu ps @Args
        }
    }

    Invoke-WaifuProcessList -FromWaifu

}
