<script>
    export let testTask = {};
    export let testTaskStringified = "{}";
    export let testTaskStringifiedError = false;

    $: {
        try {
            const parsed = JSON.parse(testTaskStringified);
            testTask = parsed;
            testTaskStringified  = JSON.stringify(parsed, null, 2);
            testTaskStringifiedError = false;
        } catch(e){
            testTaskStringifiedError = true;
        }
    }

</script>

<style>
    textarea {
        flex: 2;
        min-height: var(--size-space-11);
        height: var(--size-space-11);
        font-size: var(--size-font-03);
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
