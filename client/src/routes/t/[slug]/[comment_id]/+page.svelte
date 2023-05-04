<script>
	import Comment from '$lib/components/Comment.svelte';

	export let data;

	$: ({ parent, children, thread } = data);
</script>

<Comment comment={parent} {thread} focus={true} />
