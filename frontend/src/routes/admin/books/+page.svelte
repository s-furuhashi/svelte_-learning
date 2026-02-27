<script lang="ts">
  import { apiFetch } from '$lib/api';

  let title = $state('');
  let slug = $state('');
  let markdown = $state('');
  let imageUrl = $state('');
  let published = $state(false);
  let imageFile: File | null = null;
  let message = $state('');
  let error = $state('');
  let loading = $state(false);

  function handleFileChange(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      imageFile = input.files[0];
    }
  }

  async function uploadImage(): Promise<string | null> {
    if (!imageFile) return null;
    const formData = new FormData();
    formData.append('file', imageFile);
    const API_BASE = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';
    const res = await fetch(`${API_BASE}/admin/upload-image`, {
      method: 'POST',
      credentials: 'include',
      body: formData,
    });
    if (!res.ok) return null;
    const data = await res.json();
    return data.url;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    message = '';
    try {
      let finalImageUrl = imageUrl;
      if (imageFile) {
        const uploaded = await uploadImage();
        if (uploaded) finalImageUrl = uploaded;
      }
      const res = await apiFetch('/admin/books', {
        method: 'POST',
        body: JSON.stringify({
          title,
          slug,
          markdown,
          image_url: finalImageUrl || null,
          published,
        }),
      });
      if (res.ok) {
        message = 'Book created successfully!';
        title = '';
        slug = '';
        markdown = '';
        imageUrl = '';
        published = false;
        imageFile = null;
      } else {
        error = 'Failed to create book';
      }
    } catch {
      error = 'Network error';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Admin - Books</title>
</svelte:head>

<div class="py-8">
  <h1 class="text-2xl font-bold mb-6">Create Book</h1>
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
      <textarea id="markdown" bind:value={markdown} required rows="10" class="w-full border rounded px-3 py-2 font-mono"></textarea>
    </div>
    <div>
      <label for="image" class="block text-sm font-medium mb-1">Cover Image (WebP)</label>
      <input id="image" type="file" accept="image/webp" onchange={handleFileChange} class="w-full border rounded px-3 py-2" />
    </div>
    <div class="flex items-center gap-2">
      <input id="published" type="checkbox" bind:checked={published} class="rounded" />
      <label for="published" class="text-sm font-medium">Published</label>
    </div>
    <button type="submit" disabled={loading} class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 disabled:opacity-50">
      {loading ? 'Creating...' : 'Create Book'}
    </button>
  </form>
</div>
