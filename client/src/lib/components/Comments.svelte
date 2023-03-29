<script>
	import { enhance } from '$app/forms';
	import { timeSince } from '$lib/util';
	import CommentVoter from '$lib/components/voters/CommentVoter.svelte';

	export let thread;
	export let comments;
</script>

<form method="POST" action="?/comment">
	<textarea name="content" rows="4" placeholder="Comment" />
	<button>Post</button>
</form>

<div id="outer">
	{#each comments as comment}
		<div id="comment">
			<div id="inner">
				<small>
					<a href={`/u/${comment.username}`}>{comment.username}</a>
					{timeSince(comment.created_at)} | {comment.vote_count} points |
					<a>vote</a>
				</small>
				<div id="content">{comment.content}</div>
			</div>
		</div>
	{/each}
</div>

<style>
	#outer {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	#inner {
		display: flex;
		flex-direction: column;
	}

	#comment {
		display: flex;
	}

	#content {
		margin-top: -0.25rem;
	}

	textarea {
		resize: vertical;
		margin: 0;
	}

	form {
		margin: 0;
	}
</style>
