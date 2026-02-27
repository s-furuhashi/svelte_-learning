<script lang="ts">
  import { apiFetch } from '$lib/api';

  let title = $state('');
  let slug = $state('');
  let markdown = $state('');
  let published = $state(false);
  let message = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    message = '';
    try {
      const res = await apiFetch('/admin/articles', {
        method: 'POST',
        body: JSON.stringify({ title, slug, markdown, published }),
      });
      if (res.ok) {
        message = 'Article created successfully!';
        title = '';
        slug = '';
        markdown = '';
        published = false;
      } else {
        error = 'Failed to create article';
      }
    } catch {
      error = 'Network error';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Admin - Articles</title>
</svelte:head>

<div class="py-8">
  <h1 class="text-2xl font-bold mb-6">Create Article</h1>
  {#if message}
    <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">{message}</div>
  {/if}
  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{error}</div>
  {/if}
  <form onsubmit={handleSubmit} class="space-y-4 max-w-2xl">
    <div>
      <label for="title" class="block text-sm font-medium mb-1">Title</label>
      <input id="title" type="text" bind:value={title} required class="w-full border rounded px-3 py-2" />
    </div>
    <div>
      <label for="slug" class="block text-sm font-medium mb-1">Slug</label>
      <input id="slug" type="text" bind:value={slug} required class="w-full border rounded px-3 py-2" />
    </div>
    <div>
      <label for="markdown" class="block text-sm font-medium mb-1">Content (Markdown)</label>
      <textarea id="markdown" bind:value={markdown} required rows="15" class="w-full border rounded px-3 py-2 font-mono"></textarea>
    </div>
    <div class="flex items-center gap-2">
      <input id="published" type="checkbox" bind:checked={published} class="rounded" />
      <label for="published" class="text-sm font-medium">Published</label>
    </div>
    <button type="submit" disabled={loading} class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 disabled:opacity-50">
      {loading ? 'Creating...' : 'Create Article'}
    </button>
  </form>
</div>
