<script>
	import ListErrors from '$lib/components/ListErrors.svelte';
	import { enhance } from '$app/forms';
	export let form;
</script>

<div id="outer">
	<h1>Create an Account</h1>
	<div>
		<ListErrors errors={form?.errors} />
		<form method="POST" action="?/signup" use:enhance>
			<label>
				Username
				<input name="username" type="text" />
			</label>
			<label>
				Password
				<input name="password" type="password" />
			</label>
			<button type="submit">Submit</button>
			<p>
				Already have an account?
				<a href="/login">Log in</a>
			</p>
		</form>
	</div>
</div>

<style>
	#outer {
		display: flex;
		align-items: center;
		flex-direction: column;
	}

	form {
		width: 15rem;
	}
	h1 {
		margin: 0;
		margin-bottom: 1rem;
		font-size: 1rem;
	}
</style>
