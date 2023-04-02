<script>
	import { timeSince } from '$lib/util';

	export let data;
	let { profile } = data;
</script>

<div id='stats'>
	<h1>{profile.username}</h1>
	<p>Score: {profile.score}</p>
	<p>Joined: {timeSince(new Date(profile.created_at))}</p>
</div>
<form method="POST">
	<button type="submit">Log out</button>
</form>

<style>
	#stats {
		margin-bottom: 1rem;
	}

	h1 {
		font-size: 1rem;
		margin: 0;
	}

	p {
		margin: 0;
	}
</style>