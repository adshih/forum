<script>
	import { timeSince } from '$lib/util';
	export let thread;
	const post_time = timeSince(thread.created_at);
</script>

<div id="post-header">
	<form method="POST" action="/t/{thread.slug}?/vote">
		{#if thread.is_voted}
			<button
				id="vote-button"
				class:outline={!thread.is_voted}
				formaction="/t/{thread.slug}?/unvote"
			>
				<span>{thread.vote_count}</span>
			</button>
		{:else}
			<button id="vote-button" class:outline={!thread.is_voted}>
				<span>{thread.vote_count}</span>
			</button>
		{/if}
	</form>
	<div>
		<a href="/t/{thread.slug}"><h1>{thread.title}</h1></a>
		<div id="by-line">
			<small>submitted {post_time} by <a href={`/u/${thread.username}`}>{thread.username}</a></small
			>
		</div>
	</div>
</div>

<style>
	#post-header {
		display: flex;
	}

	#by-line {
		line-height: 1rem;
	}

	h1 {
		font-size: 1rem;
		margin: 0;
		line-height: 1rem;
	}

	form {
		margin: 0;
	}

	button {
		height: 1.8rem;
		width: 1.8rem;
		margin: 0.1rem 0.5rem 0.1rem 0;
	}
</style>
