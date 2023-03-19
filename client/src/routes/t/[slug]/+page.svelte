<script>
	import Comments from '$lib/Comments.svelte';
	export let data;
</script>

<a href="/recent">&larr; Back</a>

<h1>{data.title}</h1>
<p>{data.content}</p>

<Comments />
