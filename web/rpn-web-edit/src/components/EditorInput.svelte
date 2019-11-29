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
		font-size: var(--size-font-03);
		padding: var(--size-space-03);
	}
</style>

<textarea bind:value on:input={onInput}/>
