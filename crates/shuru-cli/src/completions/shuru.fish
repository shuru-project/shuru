function __fish_shuru_list_commands
    shuru --list-commands
end

function __fish_shuru_complete
    set -l options "-h" "--help" "-V" "--version" "--completions" "--list-commands" "--clear-cache" "--update-versions"

    echo $options
    __fish_shuru_list_commands
end

complete -c shuru -f -a "(__fish_shuru_complete)" -d "Shuru task runner"
