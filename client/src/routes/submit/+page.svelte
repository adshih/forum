<h1>Submit</h1>
<form method="POST">
	<label>
		Title
		<input name="title" type="text" />
	</label>
	<label>
		Content
		<textarea name="content" rows="5" />
	</label>
	<button type="submit">Submit</button>
</form>

<style>
	h1 {
		margin: 0;
		margin-bottom: 1rem;
		font-size: 1rem;
	}

	textarea {
		resize: vertical;
	}

	button {
		width: auto;
		padding: 0.25rem 0.5rem;
	}
</style>
