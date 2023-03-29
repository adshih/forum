<script>
	import PostHeader from '$lib/components/PostHeader.svelte';

	export let data;

	let { profile, threads } = data;
</script>

<h1>{profile.username}</h1>
<div id="posts">
	{#each data.threads as thread}
		<PostHeader {thread} />
	{/each}
</div>

<style>
	h1 {
		font-size: 1.1rem;
		margin-bottom: 1rem;
	}

	#posts {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
