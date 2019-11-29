<script>
	export let stackTrace;

</script>

<style>
	#stack-history {
		flex: 0 0 auto;
		display: flex;
		flex-direction: column;
		font-variant-numeric: tabular-nums;
		list-style: none;
		padding: var(--size-space-03);
		line-height: var(--size-space-06);
	}

	.stack-snapshot {
	}

	.stack-items {
		display: flex;
		flex-direction: row;

	}

	.stack-item {
		display: block;
		margin: var(--size-space-01);
		padding: var(--size-space-01);
		width: var(--size-space-08);
		font-size: var(--size-font-03);
	}
</style>

<ol id="stack-history">
	{#each stackTrace as stackSnapshot}
		<li class="stack-snapshot">
			<ol class="stack-items">
				{#each stackSnapshot as stackItem }
					<li class="stack-item">
						{stackItem}
					</li>
				{/each}
			</ol>
		</li>
	{/each}
</ol>
