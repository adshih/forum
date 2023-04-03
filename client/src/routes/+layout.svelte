<script>
	import Footer from '$lib/components/Footer.svelte';
	import Nav from '$lib/components/Nav.svelte';
	import '$lib/global.css';

	export let data;
</script>

<div class="container">
	<Nav {data} />
	<main>
		<slot />
	</main>
</div>

<style>
	main {
		margin-bottom: 2rem;
	}

	.container {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}
</style>
