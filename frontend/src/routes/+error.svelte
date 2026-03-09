<script lang="ts">
  import { page } from '$app/stores';
</script>

<svelte:head>
  <title>{$page.status} – {$page.error?.message ?? 'Error'}</title>
</svelte:head>

<section class="py-16 text-center">
  <h1 class="text-6xl font-bold text-gray-300 mb-4">{$page.status}</h1>
  <p class="text-xl text-gray-600 mb-8">
    {#if $page.status === 404}
      The page you are looking for does not exist.
    {:else}
      Something went wrong. Please try again later.
    {/if}
  </p>
  <a href="/" class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700">Go Home</a>
</section>
