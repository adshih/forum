<script>
	import Comment from '$lib/components/Comment.svelte';
	import CommentTree from '../../../../lib/components/CommentTree.svelte';

	export let data;

	$: ({ parent, children, thread } = data);
</script>

<Comment comment={parent} {thread} focus={true} />
<CommentTree {thread} comments={children} focus={true} />
