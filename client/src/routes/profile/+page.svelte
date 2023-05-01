<script>
	import { timeSince } from '$lib/util';

	export let data;
	let { profile } = data;
</script>

<div id="stats">
	<p>User: <strong>{profile.username}</strong></p>
	<p>Score: {profile.score}</p>
	<p>Joined: {timeSince(new Date(profile.created_at))}</p>
</div>
<form method="POST">
	<button type="submit">log out</button>
</form>

<style>
	#stats {
		margin-bottom: 1rem;
	}

	p {
		margin: 0;
	}
</style>
