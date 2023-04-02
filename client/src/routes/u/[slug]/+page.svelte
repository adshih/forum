<script>
	import PostHeader from '$lib/components/PostHeader.svelte';
	import { timeSince } from '$lib/util';

	export let data;

	let { profile, threads } = data;
</script>

<div id='stats'>
	<h1>{profile.username}</h1>
	<p>Score: {profile.score}</p>
	<p>Joined: {timeSince(new Date(profile.created_at))}</p>
</div>

<div id="posts">
	{#each data.threads as thread}
		<PostHeader {thread} />
	{/each}
</div>

<style>
	#stats {
		margin-bottom: 1rem;
	}

	h1 {
		font-size: 1rem;
		margin: 0;
	}

	#posts {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	p {
		margin: 0;
	}
</style>
