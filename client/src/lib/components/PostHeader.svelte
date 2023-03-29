<script>
	import ThreadVoter from '$lib/components/voters/ThreadVoter.svelte';
	import { timeSince } from '$lib/util';

	export let thread;

	const post_time = timeSince(thread.created_at);
</script>

<div id="post-header">
	<ThreadVoter {thread} />
	<div>
		<a href="/t/{thread.slug}"><h1>{thread.title}</h1></a>
		<small id="by-line"
			>submitted {post_time} by <a href={`/u/${thread.username}`}>{thread.username}</a></small
		>
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
