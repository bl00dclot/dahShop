// stores/cart.js
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// Initialize from localStorage
const initialCart = browser ? JSON.parse(localStorage.getItem('cart')) || [] : [];
export const cart = writable(initialCart);

// Persistent cart
if (browser) {
  cart.subscribe(value => {
    localStorage.setItem('cart', JSON.stringify(value));
  });
}

// Helper functions
export function addToCart(product) {
  cart.update(items => {
    const existing = items.find(item => item.id === product.id);
    if (existing) {
      return items.map(item =>
        item.id === product.id 
          ? { ...item, quantity: item.quantity + 1 }
          : item
      );
    }
    return [...items, { ...product, quantity: 1 }];
  });
}

export function removeFromCart(productId) {
  cart.update(items => items.filter(item => item.id !== productId));
}

export function updateQuantity(productId, quantity) {
  if (quantity <= 0) {
    removeFromCart(productId);
    return;
  }
  cart.update(items =>
    items.map(item =>
      item.id === productId ? { ...item, quantity } : item
    )
  );
}

export function clearCart() {
  cart.set([]);
}