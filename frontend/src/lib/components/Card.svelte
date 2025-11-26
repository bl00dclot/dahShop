<script lang="ts">
  import { Card, Button, Rating, Badge } from "flowbite-svelte";
  export let productObj = {
    "name": "Default name",
    "description": "Default description",
    "price_xmr": 321,
    "image_ipfs_hash": "/",
    "in_stock": false,
  };
</script>

<Card class="p-0">
  <a href="/">
    <img class="rounded-t-lg p-8" src={productObj.image_ipfs_hash} alt="Product" />
  </a>
  <div class="px-5 pb-5">
    <a href="/">
      <h5 class="text-xl font-semibold tracking-tight text-gray-900 dark:text-white">{productObj.name}</h5>
    </a>
    <Rating rating={4} size={24} class="mt-2.5 mb-5">
      {#snippet text()}
        <Badge class="ms-3">4</Badge>
      {/snippet}
    </Rating>
    <div class="flex items-center justify-between">
      <span class="text-3xl font-bold text-gray-900 dark:text-white">{productObj.price_xmr}</span>
      <Button color="red" href="/">Buy now</Button>
    </div>
  </div>
</Card>