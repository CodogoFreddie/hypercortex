<script>
	export let program;

	export let shortProgram = program;

	$: shortProgram = program.replace(/\s+/gm, " ");
</script>

<style> 
	output {
		
	}
</style>

<output>{shortProgram}</output>
