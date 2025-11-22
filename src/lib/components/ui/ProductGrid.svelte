<script>
    import Card from "../Card.svelte";
</script>

<div class="container mx-auto px-4 py-8">
  <!-- Section Header -->
  <div class="text-center mb-12">
    <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">Our Products</h2>
  </div>

  <!-- Product Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
    <!-- Repeat for each product -->
    {#each ["Product 1", "Product 2", "Product 3", "Product 4"] as product}
      <Card productName={product} />
    {/each}
  </div>
</div>