<script>
	import { timeSince } from '$lib/util';
	import PostHeader from '$lib/components/PostHeader.svelte';

	export let data;
	let { profile } = data;
</script>

<div id="outer">
	<div id="stats">
		<p>User: <strong>{profile.username}</strong></p>
		<p>Score: {profile.score}</p>
		<p>Joined: {timeSince(new Date(profile.created_at))}</p>
	</div>
	<form method="POST">
		<button type="submit">log out</button>
	</form>
</div>

<div id="posts">
	{#each data.threads as thread}
		<PostHeader {thread} />
	{/each}
</div>

<style>
	#outer {
		display: flex;
		justify-content: space-between;
	}

	#stats {
		margin-bottom: 1rem;
	}

	p {
		margin: 0;
	}
</style>
