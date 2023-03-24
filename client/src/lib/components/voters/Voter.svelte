<script>
	export let text = '0';
	export let action;
</script>

<div id="voter">
	<form method="POST" {action}>
		<button class="secondary">{text}</button>
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
