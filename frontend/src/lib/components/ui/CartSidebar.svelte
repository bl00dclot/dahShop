<!-- components/CartSidebar.svelte -->
<script>
  import { Sidebar, Button, Divider } from 'flowbite-svelte';
  import { cart, removeFromCart, updateQuantity } from '$lib/stores/cart';
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';
  
  export let open = false;
  export let closeCart = () => {};

  // Calculate totals
  $: totalItems = $cart.reduce((sum, item) => sum + item.quantity, 0);
  $: subtotal = $cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
  $: tax = subtotal * 0.1; // 10% tax example
  $: total = subtotal + tax;

  async function proceedToCheckout() {
    closeCart();
    await goto('/checkout');
  }
</script>

<Sidebar bind:open {closeCart} placement="right" class="w-full max-w-md">
  <div class="h-full flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b">
      <h2 class="text-xl font-bold">Your Cart ({totalItems})</h2>
    </div>

    <!-- Cart Items -->
    <div class="flex-1 overflow-y-auto p-4">
      {#if $cart.length === 0}
        <div class="text-center py-8">
          <p class="text-gray-500">Your cart is empty</p>
        </div>
      {:else}
        <div class="space-y-4">
          {#each $cart as item (item.id)}
            <div transition:fly="{{ y: 20, duration: 300 }}" class="flex gap-4 border-b pb-4">
              <img 
                src={item.image} 
                alt={item.name}
                class="w-16 h-16 object-cover rounded"
              />
              
              <div class="flex-1">
                <h3 class="font-semibold">{item.name}</h3>
                <p class="text-gray-600">${item.price}</p>
                
                <div class="flex items-center gap-2 mt-2">
                  <label class="text-sm">Qty:</label>
                  <select 
                    value={item.quantity}
                    on:change={(e) => updateQuantity(item.id, parseInt(e.target.value))}
                    class="border rounded px-2 py-1 text-sm"
                  >
                    {#each Array.from({ length: 10 }, (_, i) => i + 1) as num}
                      <option value={num}>{num}</option>
                    {/each}
                  </select>
                  
                  <button 
                    on:click={() => removeFromCart(item.id)}
                    class="text-red-500 hover:text-red-700 ml-auto text-sm"
                  >
                    Remove
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Footer with Totals and Checkout Button -->
    {#if $cart.length > 0}
      <div class="border-t p-4 space-y-3">
        <div class="flex justify-between">
          <span>Subtotal:</span>
          <span>${subtotal.toFixed(2)}</span>
        </div>
        <div class="flex justify-between">
          <span>Tax:</span>
          <span>${tax.toFixed(2)}</span>
        </div>
        <Divider />
        <div class="flex justify-between text-lg font-bold">
          <span>Total:</span>
          <span>${total.toFixed(2)}</span>
        </div>
        
        <Button on:click={proceedToCheckout} class="w-full !bg-green-600 hover:!bg-green-700">
          Proceed to Checkout
        </Button>
      </div>
    {/if}
  </div>
</Sidebar>