<script>
	import { enhance } from '$app/forms';
	import Voter from '$lib/components/Voter.svelte';

	export let data;
</script>

<form method="POST" action="?/comment">
	<textarea name="content" rows="4" placeholder="Comment" />
	<button>Post</button>
</form>

{#each data as comment}
	<div id="outer">
		<Voter />
		<p>{comment.content}</p>
	</div>
{/each}

<style>
	#outer {
		display: flex;
	}

	textarea {
		resize: vertical;
		margin: 0;
	}

	p {
		margin: 0;
	}

	form {
		margin: 0;
	}
</style>
