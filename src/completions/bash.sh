# Bash completion for shuru
_shuru() {
  local cur prev words cword
  _init_completion -n : || return

  # Handle special arguments of options.
  case "$prev" in
    -h|--help)
      COMPREPLY=( $( compgen -W "$(shuru --help)" -- "$cur" ) )
      return 0
    ;;
  esac

  # Prepare task name completions from shuru
  local tasks=$(shuru --list-commands)  # Assuming this command lists tasks
  COMPREPLY=( $( compgen -W "${tasks}" -- "$cur" ) )
}

complete -F _shuru shuru
