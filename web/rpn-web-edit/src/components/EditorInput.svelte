<script> 
	export let program;
	export let value = program.split("").reverse().join("");

	function onInput(){
		console.log(...arguments);
		program = value.split("").reverse().join("");
	}
</script>

<style>
	textarea {
		flex: 1;
		font-size: var(--size-font-03);
		padding: var(--size-space-03);
		line-height: var(--size-space-06);
	}
</style>

<textarea bind:value on:input={onInput}/>
