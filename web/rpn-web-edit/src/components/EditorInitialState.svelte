<script>
   export let initialStackState = "";
</script>

<style>
   input { 
       flex: 1;
       min-height: var(--size-space-05);
       height: var(--size-space-05);
       max-height: var(--size-space-05);
       font-size: var(--size-font-03);
   }
</style>

<input bind:value={initialStackState} />
