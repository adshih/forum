<script>
	import { timeSince } from '$lib/util';

	export let thread;

	const post_time = timeSince(thread.created_at);
</script>

<div id="post-header">
	<form method="POST" action='/t/{thread.slug}?/vote'>
		<button id='vote-button' class="secondary">
			<span>{thread.vote_count}</span>
			<!-- <div id='vote-icon'>
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" d="M4.5 15.75l7.5-7.5 7.5 7.5" />
				</svg>
			</div> -->
		</button>
	</form>
	<div>
		<a href="/t/{thread.slug}"><h1>{thread.title}</h1></a>
		<div id="by-line">
			<small>submitted {post_time} by <a href={`/u/${thread.username}`}>{thread.username}</a></small>
		</div>
	</div>
</div>

<style>
	svg {
		width: 1.5rem;
		height: 1.5rem;
	}

	#post-header {
		display: flex;
	}

	#by-line {
		line-height: 1rem;
	}

	/* #vote-button:hover span {
		display:none;
	}

	#vote-button:hover #vote-icon {
		display: block;
	}

	#vote-icon {
		display: none;
	}

	#vote-button {
		display: flex;
		justify-content: center;
		align-items: center;
	} */

	h1 {
		font-size: 1rem;
		margin: 0;
		line-height: 1rem;
	}

	form {
		margin: 0;
	}

	button {
		height: 1.8rem;
		width: 1.8rem;
		margin: 0.1rem 0.5rem 0.1rem 0;
	}
</style>
