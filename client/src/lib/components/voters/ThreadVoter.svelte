<script>
	import Voter from './Voter.svelte';
	export let thread;

	const { vote_count, is_voted, slug } = thread;
	const path = slug ? `/t/${slug}` : '';
	// const action = is_voted ? `${path}?/unvote` : `${path}?/vote`;
	const action = `${path}?/vote`;
</script>

<Voter {action} text={vote_count} />
