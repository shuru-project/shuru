#compdef shuru
_shuru_complete() {
    local commands=($(shuru --list-commands))
    _describe -t commands 'shuru commands' commands
}

compdef _shuru_complete shuru
