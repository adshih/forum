<script>
	import { enhance } from '$app/forms';
	import { timeSince } from '$lib/util';

	export let thread;
	export let comments;
	console.log(comments);
</script>

<div>
	<form method="POST" action="?/comment">
		<textarea name="content" rows="4" placeholder="Comment" />
		<button>Post</button>
	</form>
</div>

<div id="outer">
	{#each comments as comment}
		<div id="comment">
			<div id="inner">
				<div>
					<small>
						<a href={`/u/${comment.username}`}>{comment.username}</a>
						{timeSince(comment.created_at)} &#x2022; {comment.vote_count} points &#x2022;
						<form method="POST" action="/t/{thread.slug}?/vote_comment">
							<input type="hidden" name="id" value={comment.id.toString(36)} />
							<button class="button-a"><small>vote</small></button>
						</form>
					</small>
				</div>

				<div id="content">{comment.content}</div>
				<button class="button-a"><small>reply</small></button>
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

	textarea {
		resize: vertical;
		margin: 0;
	}

	form {
		display: inline;
		margin: 0;
	}

	.button-a {
		display: inline;
		margin: 0;
		padding: 0;
		background-color: transparent;
		border: none;
		box-shadow: none;
		color: var(--primary);
		border-radius: 0;
		text-align: start;
	}

	button:hover {
		text-decoration: underline;
		color: var(--primary-hover);
	}

	button:active * {
		background-color: var(--primary-focus);
		transition: background-color var(--transition), color var(--transition),
			text-decoration var(--transition);
	}
</style>
