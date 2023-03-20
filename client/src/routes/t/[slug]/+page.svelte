<script>
	import Comments from '$lib/Comments.svelte';

	export let data;

	const { thread, comments } = data;
	const paragraphs = thread.content.split(/\r?\n/).filter((par) => par != '');
	console.log(paragraphs);
</script>

<a href="/recent">&larr; Back</a>

<h1>{thread.title}</h1>
<div id="content">
	{#each paragraphs as paragraph}
		<p>{paragraph}</p>
	{/each}
</div>
<Comments data={comments} />

<style>
	h1 {
		margin: 0;
	}

	p {
		margin-bottom: 1rem;
	}

	/* article {
		box-shadow: none;
	} */

	#content {
		margin: 1rem 0;
	}
</style>
