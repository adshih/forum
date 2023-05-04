<script>
	import PostHeader from '$lib/components/PostHeader.svelte';
	export let data;
	$: ({ threads } = data);
</script>

{#if data.threads.length === 0}
	<p>Looks like there is nothing here...</p>
	<p>Be the first to <a href="/submit">submit</a> a post!</p>
{/if}

<div id="posts">
	{#each threads as thread}
		<PostHeader {thread} />
	{/each}
</div>

<style>
	#posts {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	p {
		margin: 0;
		padding: 0;
	}
</style>
