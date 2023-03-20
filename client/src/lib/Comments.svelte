<script>
	import { enhance } from '$app/forms';

	export let data;
</script>

<form method="POST">
	<textarea name="content" rows="4" />
	<button type="submit">Post</button>
</form>

{#each data as comment}
	<p>{comment.content}</p>
{/each}

<style>
	textarea {
		resize: vertical;
	}
</style>
