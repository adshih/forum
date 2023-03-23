<script>
	import Comments from '$lib/components/Comments.svelte';
	import PostHeader from '$lib/components/PostHeader.svelte';
	import { hoursSince } from '$lib/util';

	export let data;

	const { thread, comments, votes } = data;
	const paragraphs = thread.content.split(/\r?\n/).filter((par) => par != '');
</script>

<div id="outer">
	<PostHeader {thread} />
	<div class="indent">
		<div>
			{#each paragraphs as paragraph}
				<p>{paragraph}</p>
			{/each}
		</div>
		<!-- <div>
			<Comments data={comments} />
		</div> -->
	</div>
</div>

<style>
	.indent {
		margin-left: 2.25rem;
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
