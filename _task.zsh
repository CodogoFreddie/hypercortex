#compdef task
# source _task.zsh && compdef _task task
# https://mads-hartmann.com/2017/08/06/writing-zsh-completion-scripts.html

function _task (){
	compadd -X "love from freddie" $(cargo run --quiet --bin _task_complete -- "$words" $CURRENT)
}
