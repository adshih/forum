<script>
	import CommentTree from '$lib/components/CommentTree.svelte';
	import PostHeader from '$lib/components/PostHeader.svelte';

	export let data;

	const paragraphs = data.thread.content.split(/\r?\n/).filter((par) => par != '');
</script>

<div id="outer">
	<PostHeader thread={data.thread} />
	<div class="indent">
		<div>
			{#each paragraphs as paragraph}
				<p>{paragraph}</p>
			{/each}
		</div>
		<div>
			<CommentTree thread={data.thread} comments={data.comments} />
		</div>
	</div>
</div>

<style>
	.indent {
		margin: 0 2.25rem;
	}

	#outer {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	p {
		margin-bottom: 0.5rem;
	}
</style>
