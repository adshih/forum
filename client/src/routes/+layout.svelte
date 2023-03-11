<script>
	export let data;
</script>

<div class="container">
	<nav>
		<ul>
			<li>
				<a href="/">Home</a>
			</li>
			<li>
				<a href="/recent">Recent</a>
			</li>
		</ul>
		<ul>
			<li>
				{#if !data.user}
					<a href="/login">Login</a>
				{:else}
					<a href="/profile">{data.user.username}</a>
				{/if}
			</li>
		</ul>
	</nav>
	<main>
		<slot />
	</main>
</div>
