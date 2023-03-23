<script>
	import Footer from '$lib/components/Footer.svelte';
	import Nav from '$lib/components/Nav.svelte';
	export let data;
</script>

<div class="container">
	<Nav {data} />
	<main>
		<slot />
	</main>
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	:root {
		--border-radius: 0;
	}

	[data-theme='light'],
	:root:not([data-theme='dark']) {
		--primary: #757575;
		--primary-hover: #616161;
		--primary-focus: rgba(117, 117, 117, 0.125);
		--primary-inverse: #fff;
	}

	@media only screen and (prefers-color-scheme: dark) {
		:root:not([data-theme]) {
			--primary: #757575;
			--primary-hover: #9e9e9e;
			--primary-focus: rgba(117, 117, 117, 0.25);
			--primary-inverse: #fff;
		}
	}

	[data-theme='dark'] {
		--primary: #757575;
		--primary-hover: #9e9e9e;
		--primary-focus: rgba(117, 117, 117, 0.25);
		--primary-inverse: #fff;
	}

	:root {
		--form-element-active-border-color: var(--primary);
		--form-element-focus-color: var(--primary-focus);
		--switch-color: var(--primary-inverse);
		--switch-checked-background-color: var(--primary);
	}

	:global(button) {
		width: auto;
		padding: 0.03125rem 0.5rem;
	}

	@media (min-width: 576px) {
		.container {
			max-width: 510px;
			padding-right: 0;
			padding-left: 0;
		}
	}
	@media (min-width: 768px) {
		.container {
			max-width: 700px;
		}
	}
	@media (min-width: 992px) {
		.container {
			max-width: 920px;
		}
	}
</style>
