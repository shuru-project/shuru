#compdef shuru

_shuru() {
    local commands
    commands=($(shuru --list-commands))

    _describe -t commands 'shuru commands' commands "$@"
}

compdef _shuru shuru
