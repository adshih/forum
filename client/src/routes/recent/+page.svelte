<script>
	export let data;
</script>

{#each data.threads as thread}
	<a href="/t/{thread.slug}">{thread.title}</a>
{/each}

<style>
	a {
		display: block;
	}
</style>
