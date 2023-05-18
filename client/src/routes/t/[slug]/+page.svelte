<script>
	import CommentTree from '$lib/components/CommentTree.svelte';
	import PostHeader from '$lib/components/PostHeader.svelte';

	export let data;

	$: ({ thread, comments } = data);
	$: paragraphs = thread.content.split(/\r?\n/).filter((par) => par != '');
</script>

<div id="outer">
	<PostHeader {thread} />
	<div class="indent">
		<div id="content">
			{#each paragraphs as paragraph}
				<p>{paragraph}</p>
			{/each}
		</div>
		<div>
			<CommentTree {thread} {comments} />
		</div>
	</div>
</div>

<style>
	.indent {
		margin: 0 2.25rem;
	}

	#content {
		margin: 1rem 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	#outer {
		display: flex;
		flex-direction: column;
	}

	p {
		margin-bottom: 0;
	}
</style>
