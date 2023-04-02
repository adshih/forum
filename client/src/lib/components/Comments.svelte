<script>
	import { enhance } from '$app/forms';
	import { timeSince } from '$lib/util';

	export let thread;
	export let comments;
	console.log(comments);
</script>

<form method="POST" action="?/comment">
	<textarea name="content" rows="4" placeholder="Comment" />
	<button>Post</button>
</form>

<div id="outer">
	{#each comments as comment}
		<div id="comment">
			<div id="inner">
				<div>
					<small>
						<a href={`/u/${comment.username}`}>{comment.username}</a>
						{timeSince(comment.created_at)} | {comment.vote_count} points
						<!-- <a>vote</a>
						|
						<a>reply</a> -->
					</small>
				</div>
				<form method="POST" action="/t/{thread.slug}?/vote_comment">
					<input type="hidden" name="id" value={comment.id.toString(36)} />
					<button>vote</button>
				</form>
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
</style>
