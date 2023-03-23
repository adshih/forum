<script>
	export let count = 0;
	export let slug = '';

	const path = slug ? `/t/${slug}` : '';
</script>

<div id="voter">
	<form method="POST" action="{path}?/vote">
		<button>{count}</button>
	</form>
</div>

<style>
	#voter {
		display: flex;
		margin-right: 0.5rem;
		justify-content: center;
		align-items: center;
	}

	button {
		margin: 0;
		border-radius: 0.5rem;
	}

	form {
		margin: 0;
	}
</style>
