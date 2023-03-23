<script>
	import Voter from '$lib/components/Voter.svelte';
	import { hoursSince } from '$lib/util';

	export let thread;

	const hours_since_post = hoursSince(new Date(thread.created_at));
	const post_time =
		hours_since_post < 24
			? `${hours_since_post} hours ago`
			: `${Math.floor(hours_since_post / 24)} days ago`;
</script>

<div id="post-header">
	<Voter count={thread.vote_count} slug={thread.slug} />
	<div>
		<a href="/t/{thread.slug}"><h1>{thread.title}</h1></a>
		<small id="by-line">submitted {post_time} by {thread.username}</small>
	</div>
</div>

<style>
	#post-header {
		display: flex;
	}

	#by-line {
		display: block;
		margin-top: -0.4rem;
	}

	h1 {
		font-size: 1.1rem;
		margin: 0;
	}
</style>
