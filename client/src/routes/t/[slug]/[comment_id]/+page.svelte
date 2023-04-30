<script>
	import { timeSince } from '$lib/util.js';
	export let data;
	let { parent, children, thread } = data;
</script>

<div id="comment">
	<div id="inner">
		<div>
			<small>
				<a href={`/u/${parent.username}`}>{parent.username}</a>
				{timeSince(parent.created_at)} &#x2022; {parent.vote_count} points &#x2022;
				<form method="POST" action="/t/{thread.slug}?/vote_comment">
					<input type="hidden" name="id" value={parent.id.toString(36)} />
					{#if parent.is_voted}
						<button class="button-a" formaction="/t/{thread.slug}?/unvote_comment"
							><small>unvote</small></button
						>
					{:else}
						<button class="button-a"><small>vote</small></button>
					{/if}
				</form>
			</small>
		</div>

		<div id="content">{parent.content}</div>
		<!-- <a href={`/t/test/${parent.id}`}><small>reply</small></a> -->
		<div>
			<form method="POST" action="?/comment">
				<textarea name="content" rows="4" placeholder="Comment" />
				<button>reply</button>
			</form>
		</div>
	</div>
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
		gap: 1rem;
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
