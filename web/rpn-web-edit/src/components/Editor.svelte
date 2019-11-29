<script>
	import qs from "qs";

	import EditorInitialState from "./EditorInitialState.svelte";
	import EditorTestTask from "./EditorTestTask.svelte";
	import EditorInput from "./EditorInput.svelte";
	import EditorStackViewer from "./EditorStackViewer.svelte";
	import EditorOutput from "./EditorOutput.svelte";

	export let initialStackState = "";
	export let stackTrace = [];
	export let testTask = {};
	export let program = "";

</script>

<style>
	#editor-container {
		display: flex;
		flex-direction: column;
	}

	#initialiseation-container {
		display: flex;
		flex-direction: row;
	}

	#stack-container {
		display: flex;
		flex-direction: row;
	}
</style>

<div id="editor-container">
	<div id="initialiseation-container">
		<EditorInitialState bind:initialStackState={initialStackState} />
		<EditorTestTask bind:testTask={testTask} />
	</div>
	<code>input: {initialStackState} {JSON.stringify(testTask, null, 2)}</code>
	<div id="stack-container">
		<EditorInput bind:program={program} />
		<EditorStackViewer bind:stackTrace={stackTrace}/>
	</div>
	<EditorOutput {program} />
</div>
