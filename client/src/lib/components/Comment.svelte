<script>
	import { timeSince } from '$lib/util.js';

	export let comment;
	export let thread;
	export let focus = false;

	const id = comment.id.toString(36);
</script>

<div id="outer">
	<div>
		<small>
			<a href={`/u/${comment.username}`}>{comment.username}</a>
			{timeSince(comment.created_at)} &#x2022; {comment.vote_count} points &#x2022;
			<form method="POST" action="/t/{thread.slug}/{id}?/vote">
				<input type="hidden" name="id" value={id} />
				{#if comment.is_voted}
					<button class="button-a" formaction="/t/{thread.slug}/{id}?/unvote"
						><small>unvote</small></button
					>
				{:else}
					<button class="button-a"><small>vote</small></button>
				{/if}
			</form>
		</small>
	</div>

	<div id="content">{comment.content}</div>

	{#if !focus}
		<a href={`/t/${thread.slug}/${comment.id}`}><small>reply</small></a>
	{:else}
		<div id="reply-form">
			<form method="POST" action="?/reply">
				<textarea name="content" rows="4" placeholder="Comment" />
				<button>reply</button>
			</form>
		</div>
	{/if}
</div>

<style>
	#reply-form {
		margin-top: 1rem;
	}

	#outer {
		display: flex;
		flex-direction: column;
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

	.button-a:hover {
		text-decoration: underline;
		color: var(--primary-hover);
	}

	.button-a:active * {
		background-color: var(--primary-focus);
		transition: background-color var(--transition);
		color: var(--transition);
		text-decoration: var(--transition);
	}
</style>
