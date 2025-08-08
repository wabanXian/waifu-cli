Remove-Item Alias:cd -Force -ErrorAction SilentlyContinue
Remove-Item Alias:ls -Force -ErrorAction SilentlyContinue

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

