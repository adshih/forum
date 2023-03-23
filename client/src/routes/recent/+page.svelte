<script>
	import PostHeader from '$lib/components/PostHeader.svelte';
	export let data;
</script>

{#each data.threads as thread}
	<PostHeader {thread} />
{/each}
