<div id="outer">
	<h1>Log in</h1>
	<form method="POST" action="?/login">
		<label>
			Username
			<input name="username" type="text" />
		</label>
		<label>
			Password
			<input name="password" type="password" />
		</label>
		<button type="submit">Log in</button>
		<p>
			Don't have an account?
			<a href="/signup">Sign up</a>
		</p>
	</form>
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
