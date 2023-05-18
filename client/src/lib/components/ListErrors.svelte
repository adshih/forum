<script>
	export let errors;
</script>

{#if errors}
	{#each Object.keys(errors) as key}
		<p class="error">{key} {errors[key]}</p>
	{/each}
{/if}

<style>
	.error {
		color: rgb(255, 127, 127);
	}
</style>
