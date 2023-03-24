<script>
	import Voter from './Voter.svelte';

	export let slug;
	export let comment;

	const { vote_count, is_voted } = comment;

	const path = slug ? `/t/${slug}` : '';
	// const action = is_voted ? `${path}?/unvote` : `${path}?/vote`;
	const action = `${path}?/vote_comment`;
</script>

<Voter {action} text={vote_count} />
