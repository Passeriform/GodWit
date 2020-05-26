
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'godwit' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'godwit'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'godwit' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Setup Godwit working directory')
            [CompletionResult]::new('switch', 'switch', [CompletionResultType]::ParameterValue, 'Switch to target glyph, organization or project')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add projects under Godwit')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove projects under Godwit')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Display Godwit''s status')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'godwit;init' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--headless', 'headless', [CompletionResultType]::ParameterName, 'Headless setup')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'Purge existing settings before setup')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit;switch' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--default', 'default', [CompletionResultType]::ParameterName, 'Use as default project')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit;add' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Add existing project (Doesn''t trigger weaver)')
            [CompletionResult]::new('--existing', 'existing', [CompletionResultType]::ParameterName, 'Add existing project (Doesn''t trigger weaver)')
            [CompletionResult]::new('--active', 'active', [CompletionResultType]::ParameterName, 'Add as active project')
            [CompletionResult]::new('--default', 'default', [CompletionResultType]::ParameterName, 'Add as default project')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit;remove' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit;status' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
        'godwit;help' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--org', 'org', [CompletionResultType]::ParameterName, 'Organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('--project', 'project', [CompletionResultType]::ParameterName, 'Project in organization (for all operations) (Overrides glyph)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Silence all output')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Debug mode')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Debug mode')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
