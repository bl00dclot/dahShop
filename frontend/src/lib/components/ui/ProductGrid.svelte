<script>
	import Card from '../Card.svelte';

	import { onMount } from 'svelte';
	import { api } from '$lib/api/api.js';

	let products = [];
	let loading = true;
	let error = null;

	onMount(async () => {
		try {
			const response = await api.getProducts();
			if (response.success) {
				products = response.data;
			} else {
				error = 'Failed to load products';
			}
		} catch (err) {
			error = 'Network error: ' + err.message;
		} finally {
			loading = false;
		}
	});
</script>

<div class="container mx-auto px-4 py-8">
	<!-- Section Header -->
	<div class="text-center mb-12">
		<h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">Our Products</h2>
	</div>

	<!-- Product Grid -->
	{#if loading}
		<div class="loading">Loading products...</div>
	{:else if error}
		<div class="error">{error}</div>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
			<!-- Repeat for each product -->
			{#each products as product}
				<Card productObj={product} />
			{/each}
		</div>
	{/if}
</div>
