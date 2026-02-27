<script lang="ts">
  import { apiFetch } from '$lib/api';
  import { goto } from '$app/navigation';

  async function logout() {
    await apiFetch('/logout', { method: 'POST' });
    goto('/admin/login');
  }
</script>

<svelte:head>
  <title>Admin Dashboard</title>
</svelte:head>

<div class="py-8">
  <div class="flex justify-between items-center mb-8">
    <h1 class="text-3xl font-bold">Admin Dashboard</h1>
    <button onclick={logout} class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600">Logout</button>
  </div>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <a href="/admin/articles" class="block p-6 border rounded hover:shadow-md transition-shadow">
      <h2 class="text-xl font-semibold mb-2">Articles</h2>
      <p class="text-gray-600">Manage your articles</p>
    </a>
    <a href="/admin/books" class="block p-6 border rounded hover:shadow-md transition-shadow">
      <h2 class="text-xl font-semibold mb-2">Books</h2>
      <p class="text-gray-600">Manage your book reviews</p>
    </a>
  </div>
</div>
