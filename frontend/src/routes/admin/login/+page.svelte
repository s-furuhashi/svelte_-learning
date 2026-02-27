<script lang="ts">
  import { apiFetch } from '$lib/api';
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    try {
      const res = await apiFetch('/login', {
        method: 'POST',
        body: JSON.stringify({ email, password }),
      });
      if (res.ok) {
        goto('/admin/dashboard');
      } else {
        error = 'Invalid email or password';
      }
    } catch {
      error = 'Network error';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Admin Login</title>
</svelte:head>

<div class="max-w-md mx-auto mt-16">
  <h1 class="text-2xl font-bold mb-6">Admin Login</h1>
  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{error}</div>
  {/if}
  <form onsubmit={handleSubmit} class="space-y-4">
    <div>
      <label for="email" class="block text-sm font-medium mb-1">Email</label>
      <input
        id="email"
        type="email"
        bind:value={email}
        required
        class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    <div>
      <label for="password" class="block text-sm font-medium mb-1">Password</label>
      <input
        id="password"
        type="password"
        bind:value={password}
        required
        class="w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
    <button
      type="submit"
      disabled={loading}
      class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 disabled:opacity-50"
    >
      {loading ? 'Logging in...' : 'Login'}
    </button>
  </form>
</div>
