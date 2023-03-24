<script>
	import PostHeader from '$lib/components/PostHeader.svelte';
	export let data;
</script>

<div id="posts">
	{#each data.threads as thread}
		<PostHeader {thread} />
	{/each}
</div>

<style>
	#posts {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
