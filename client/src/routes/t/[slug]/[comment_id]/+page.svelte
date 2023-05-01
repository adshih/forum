<script>
	import Comment from '$lib/components/Comment.svelte';
	import { timeSince } from '$lib/util.js';

	export let data;

	let { parent, children, thread } = data;
</script>

<Comment comment={parent} {thread} focus={true} />
