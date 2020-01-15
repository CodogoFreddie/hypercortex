#compdef task
# source _task.zsh && compdef _task task
# https://mads-hartmann.com/2017/08/06/writing-zsh-completion-scripts.html

function _task (){

	cargo run --bin _task_complete -- "$words" $CURRENT 

	#compadd -X "love from freddie" $CURRENT $PREFIX $SUFFIX

	#_arguments -C \
		#"-h[Show help information]" \
		#"--h[Show help information]" \
		#"1: :(quietly loudly)" \
		#"2: :(foo bar baz)" \
		#"3: :(foo bar baz)" \
		#"4: :(foo bar baz)" \
		#"5: :(foo bar baz)" \
		#"6: :(foo bar baz)" \
		#"7: :(foo bar baz)" \
		#"*::arg:->args"
}
