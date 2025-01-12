_shuru() {
    local cur prev words cword
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    local options="-h --help -V --version --completions --list-commands --update-versions --clear-cache"

    if [[ "$prev" == -* ]]; then
        COMPREPLY=( $( compgen -W "$options" -- "$cur" ) )
        return 0
    fi

    local tasks
    tasks=$(shuru --list-commands)
    COMPREPLY=( $( compgen -W "$tasks" -- "$cur" ) )
}

complete -F _shuru shuru
