function __fish_shuru_list_commands
    shuru --list-commands
end

function __fish_shuru_complete
    if test (commandline -t) = "-h" -o (commandline -t) = "--help"
        shuru --help
        return 0
    end

    __fish_shuru_list_commands
end

complete -c shuru -f -a "(__fish_shuru_complete)" -d "Shuru task runner"
