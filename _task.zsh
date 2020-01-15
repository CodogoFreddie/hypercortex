#compdef _task task
#
# https://mads-hartmann.com/2017/08/06/writing-zsh-completion-scripts.html
# _expand
    #This completer function does not really perform completion, but instead checks if the word on the command line is eligible for expansion and, if it is, gives detailed control over how this expansion is done. For this to happen, the completion system needs to be invoked with complete-word, not expand-or-complete (the default binding for TAB), as otherwise the string will be expanded by the shell’s internal mechanism before the completion system is started. Note also this completer should be called before the _complete completer function.

    #The tags used when generating expansions are all-expansions for the string containing all possible expansions, expansions when adding the possible expansions as single matches and original when adding the original string from the line. The order in which these strings are generated, if at all, can be controlled by the group-order and tag-order styles, as usual.

    #The format string for all-expansions and for expansions may contain the sequence ‘%o’ which will be replaced by the original string from the line.

    #The kind of expansion to be tried is controlled by the substitute, glob and subst-globs-only styles.

    #It is also possible to call _expand as a function, in which case the different modes may be selected with options: -s for substitute, -g for glob and -o for subst-globs-only.

function _task (){
	_arguments -C \
		"-h[Show help information]" \
		"--h[Show help information]" \
		"1: :(quietly loudly)" \
		"2: :(foo bar baz)" \
		"*::arg:->args"
}
