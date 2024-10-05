# Bash completion for shuru
_shuru() {
  local cur prev words cword
  _init_completion -n : || return

  case "$prev" in
    -h|--help)
      COMPREPLY=( $( compgen -W "$(shuru --help)" -- "$cur" ) )
      return 0
    ;;
  esac

  local tasks=$(shuru --list-commands)
  COMPREPLY=( $( compgen -W "${tasks}" -- "$cur" ) )
}

complete -F _shuru shuru
