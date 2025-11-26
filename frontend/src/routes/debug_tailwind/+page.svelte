<!-- src/routes/debug-tailwind/+page.svelte -->
<script>
	import { Button, Card, Alert } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	let cssStatus = {};

	onMount(() => {
		// Check which CSS files are loaded
		Array.from(document.styleSheets).forEach((sheet) => {
			try {
				const rules = sheet.cssRules || sheet.rules;
				cssStatus[sheet.href] = {
					loaded: true,
					rules: rules?.length || 0
				};
			} catch (e) {
				cssStatus[sheet.href] = {
					loaded: false,
					error: e.message
				};
			}
		});
		console.log('CSS Status:', cssStatus);
	});
</script>

<div class="p-8 space-y-4">
	<h1 class="text-3xl font-bold text-gray-900 dark:text-white">Tailwind v4 + Flowbite Test</h1>

	<!-- Test basic Tailwind classes -->
	<div class="p-4 border-2 border-dashed">
		<p class="text-red-500 bg-red-100 p-2">Red text test</p>
		<p class="text-blue-500 bg-blue-100 p-2">Blue text test</p>
		<p class="text-green-500 bg-green-100 p-2">Green text test</p>
		<button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
			Raw Tailwind Button
		</button>
	</div>

	<!-- Test Flowbite components -->
	<div class="p-4 border-2 border-dashed border-blue-500">
		<h2 class="text-xl font-semibold mb-4">Flowbite Components:</h2>
		<Button>Flowbite Button</Button>
		<Button color="red">Red Button</Button>
		<Button color="green">Green Button</Button>
	</div>
</div>

<div class="p-8 space-y-4">
	<h2 class="text-2xl font-bold">CSS Debug</h2>

	<div class="grid grid-cols-2 gap-4">
		<!-- Test buttons -->
		<div class="border p-4">
			<h3 class="font-semibold mb-2">No color prop:</h3>
			<Button>Default Button</Button>
		</div>

		<div class="border p-4">
			<h3 class="font-semibold mb-2">With color="blue":</h3>
			<Button color="blue">Blue Button</Button>
		</div>
	</div>

	<div class="mt-8 p-4 bg-gray-100">
		<h3 class="font-semibold">Loaded Stylesheets:</h3>
		<pre>{JSON.stringify(cssStatus, null, 2)}</pre>
	</div>
</div>
