# PowerShell argument completion for waifu and waifu.exe.
# Does not change native cmdlets or the existing -Raw behavior.

function Get-WaifuCompletionCandidates {
    param(
        [string]$Word,
        [System.Management.Automation.Language.CommandAst]$Ast
    )

    $tokens = @($Ast.CommandElements | ForEach-Object { $_.Extent.Text.Trim([char]39, [char]34) })
    $arguments = @($tokens | Select-Object -Skip 1)
    $root = @('ping','cd','ls','cat','echo','clear','ps','doctor','git','update','--help','--version')

    if ($arguments.Count -eq 0 -or
        ($arguments.Count -eq 1 -and $Word -and $arguments[0] -eq $Word)) {
        $options = $root
    } else {
        $cmd = $arguments[0]
        switch ($cmd) {
            'ping'   { $options = @('--count','-c','--miao','--help') }
            'cd'     { $options = @('--miao','--help') }
            'ls'     { $options = @('--miao','--help') }
            'cat'    { $options = @('--miao','--help') }
            'echo'   { $options = @('--miao','--help') }
            'clear'  { $options = @('--help') }
            'ps'     { $options = @('--top','-t','--sort','-s','--miao','--help') }
            'doctor' { $options = @('--voice-test','--help') }
            'git' {
                $gitSub = if ($arguments.Count -gt 1) { $arguments[1] } else { '' }
                if (-not $gitSub -or ($arguments.Count -eq 2 -and $Word -and $gitSub -eq $Word)) {
                    $options = @('status','log','--help')
                } elseif ($gitSub -eq 'log') {
                    $options = @('--count','-n','--help')
                } else {
                    $options = @('--help')
                }
            }
            'update' { $options = @('--help') }
            default  { $options = @() }
        }

        if ($arguments.Count -ge 2) {
            $previous = $arguments[$arguments.Count - 2]
            if ($previous -in @('--sort','-s')) { $options = @('cpu','mem','memory') }
            if ($previous -in @('--count','-n','-c','--top','-t')) { $options = @('5','10','20') }
        }

        if ($cmd -in @('cd','ls','cat') -and -not $Word.StartsWith('-')) {
            # Path completion uses file listings only; never evaluates user input.
            $parent = Split-Path -Path $Word -Parent
            if (-not $parent) { $parent = '.' }
            if (Test-Path -LiteralPath $parent -PathType Container) {
                foreach ($entry in @(Get-ChildItem -LiteralPath $parent -ErrorAction SilentlyContinue)) {
                    if ($cmd -ne 'cat' -and -not $entry.PSIsContainer) { continue }
                    $path = Join-Path $parent $entry.Name
                    if ($entry.PSIsContainer) { $path += '\' }
                    $options += $path
                }
            }
        }
    }
    foreach ($value in @($options | Where-Object { $_ -like "$([System.Management.Automation.WildcardPattern]::Escape($Word))*" } | Select-Object -Unique)) {
        $replacement = if ($value.Contains(' ')) { "'" + $value.Replace("'", "''") + "'" } else { $value }
        [System.Management.Automation.CompletionResult]::new(
            $replacement, $value, 'ParameterValue', $value
        )
    }
}

Register-ArgumentCompleter -CommandName 'waifu' -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)
    Get-WaifuCompletionCandidates -Word $wordToComplete -Ast $commandAst
}

Register-ArgumentCompleter -Native -CommandName 'waifu.exe' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    Get-WaifuCompletionCandidates -Word $wordToComplete -Ast $commandAst
}
