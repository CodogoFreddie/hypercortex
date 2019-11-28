<script>
   export let testTask = {};
   export let testTaskStringified = "{}";
   export let testTaskStringifiedError = false;

   $: {
      try {
         const parsed = JSON.parse(testTaskStringified);
         testTask = parsed;
         testTaskStringifiedError = false;
      } catch(e){

         testTaskStringifiedError = true;
      }
   }

</script>

<style>
   textarea {
      flex: 2;
   }

   .invalid {
      background-color: var(--color-chromatic-red);
      color: var(--color-chromatic-white);
   }
</style>

<textarea
    class:invalid={testTaskStringifiedError} 
    bind:value={testTaskStringified}
/>
