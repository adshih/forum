<h1>Login</h1>
<form method="POST" action="?/login">
	<label>
		Username
		<input name="username" type="text" />
	</label>
	<label>
		Password
		<input name="password" type="password" />
	</label>
	<div id="buttons">
		<button type="submit">Log in</button>
		<button type="submit" formaction="?/register">Register</button>
	</div>
</form>

<style>
	#buttons {
		display: flex;
		gap: 1rem;
	}
</style>
