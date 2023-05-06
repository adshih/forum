<script>
	import { enhance } from '$app/forms';
	import { timeSince } from '$lib/util';
	import Comment from '$lib/components/Comment.svelte';

	export let thread;
	export let comments;
	export let focus = false;
</script>

{#if !focus}
	<div>
		<form method="POST" action="?/comment" use:enhance>
			<textarea name="content" rows="4" placeholder="Comment" />
			<button>submit</button>
		</form>
	</div>
{/if}

<div id="outer">
	{#each comments as comment}
		<Comment {comment} {thread} />
	{/each}
</div>

<style>
	#outer {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	textarea {
		resize: vertical;
		margin: 0;
	}

	form {
		display: inline;
		margin: 0;
	}
</style>
