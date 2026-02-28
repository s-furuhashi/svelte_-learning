<script lang="ts">
  import { apiFetch } from '$lib/api';
  import { onMount } from 'svelte';

  type Article = {
    id: string;
    title: string;
    slug: string;
    markdown: string;
    published: boolean;
    created_at: unknown;
  };

  // 一覧
  let articles = $state<Article[]>([]);
  let listError = $state('');

  // フォーム共通
  let message = $state('');
  let formError = $state('');
  let loading = $state(false);

  // 新規作成フォーム
  let showForm = $state(false);
  let newTitle = $state('');
  let newMarkdown = $state('');
  let newPublished = $state(false);

  // 編集モーダル
  let editingArticle = $state<Article | null>(null);
  let editTitle = $state('');
  let editMarkdown = $state('');
  let editPublished = $state(false);

  async function loadArticles() {
    listError = '';
    try {
      const res = await apiFetch('/admin/articles');
      if (res.ok) {
        articles = await res.json();
      } else {
        listError = 'Failed to load articles';
      }
    } catch {
      listError = 'Network error';
    }
  }

  onMount(loadArticles);

  async function handleCreate(e: Event) {
    e.preventDefault();
    loading = true;
    formError = '';
    message = '';
    try {
      const res = await apiFetch('/admin/articles', {
        method: 'POST',
        body: JSON.stringify({ title: newTitle, markdown: newMarkdown, published: newPublished }),
      });
      if (res.ok) {
        const data = await res.json();
        message = `作成しました！ slug: ${data.slug}`;
        newTitle = '';
        newMarkdown = '';
        newPublished = false;
        showForm = false;
        await loadArticles();
      } else {
        const data = await res.json();
        formError = data.error ?? 'Failed to create article';
      }
    } catch {
      formError = 'Network error';
    } finally {
      loading = false;
    }
  }

  function startEdit(article: Article) {
    editingArticle = article;
    editTitle = article.title;
    editMarkdown = article.markdown;
    editPublished = article.published;
    message = '';
    formError = '';
  }

  async function handleUpdate(e: Event) {
    e.preventDefault();
    if (!editingArticle) return;
    loading = true;
    formError = '';
    message = '';
    try {
      const res = await apiFetch(`/admin/articles/${editingArticle.id}`, {
        method: 'PUT',
        body: JSON.stringify({ title: editTitle, markdown: editMarkdown, published: editPublished }),
      });
      if (res.ok) {
        message = '更新しました！';
        editingArticle = null;
        await loadArticles();
      } else {
        const data = await res.json();
        formError = data.error ?? 'Failed to update article';
      }
    } catch {
      formError = 'Network error';
    } finally {
      loading = false;
    }
  }

  async function handleDelete(article: Article) {
    if (!confirm(`「${article.title}」を削除しますか？`)) return;
    message = '';
    try {
      const res = await apiFetch(`/admin/articles/${article.id}`, { method: 'DELETE' });
      if (res.ok || res.status === 204) {
        message = '削除しました';
        await loadArticles();
      } else {
        listError = 'Failed to delete article';
      }
    } catch {
      listError = 'Network error';
    }
  }
</script>

<svelte:head>
  <title>Admin - Articles</title>
</svelte:head>

<div class="py-8">
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-2xl font-bold">Articles</h1>
    <button
      onclick={() => { showForm = !showForm; formError = ''; message = ''; }}
      class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
    >
      {showForm ? 'キャンセル' : '+ 新規作成'}
    </button>
  </div>

  {#if message}
    <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">{message}</div>
  {/if}

  {#if showForm}
    <div class="border rounded p-6 mb-8 bg-gray-50">
      <h2 class="text-lg font-semibold mb-4">新規記事を作成</h2>
      {#if formError}
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{formError}</div>
      {/if}
      <form onsubmit={handleCreate} class="space-y-4 max-w-2xl">
        <div>
          <label for="new-title" class="block text-sm font-medium mb-1">Title</label>
          <input id="new-title" type="text" bind:value={newTitle} required class="w-full border rounded px-3 py-2" />
        </div>
        <div>
          <label for="new-markdown" class="block text-sm font-medium mb-1">Content (Markdown)</label>
          <textarea id="new-markdown" bind:value={newMarkdown} required rows="10" class="w-full border rounded px-3 py-2 font-mono text-sm"></textarea>
        </div>
        <div class="flex items-center gap-2">
          <input id="new-published" type="checkbox" bind:checked={newPublished} class="rounded" />
          <label for="new-published" class="text-sm font-medium">公開する</label>
        </div>
        <button type="submit" disabled={loading} class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 disabled:opacity-50">
          {loading ? '作成中...' : '作成'}
        </button>
      </form>
    </div>
  {/if}

  {#if editingArticle}
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-y-auto p-6">
        <h2 class="text-lg font-semibold mb-4">記事を編集</h2>
        {#if formError}
          <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{formError}</div>
        {/if}
        <form onsubmit={handleUpdate} class="space-y-4">
          <div>
            <label for="edit-title" class="block text-sm font-medium mb-1">Title</label>
            <input id="edit-title" type="text" bind:value={editTitle} required class="w-full border rounded px-3 py-2" />
          </div>
          <div>
            <label for="edit-markdown" class="block text-sm font-medium mb-1">Content (Markdown)</label>
            <textarea id="edit-markdown" bind:value={editMarkdown} required rows="12" class="w-full border rounded px-3 py-2 font-mono text-sm"></textarea>
          </div>
          <div class="flex items-center gap-2">
            <input id="edit-published" type="checkbox" bind:checked={editPublished} class="rounded" />
            <label for="edit-published" class="text-sm font-medium">公開する</label>
          </div>
          <div class="flex gap-3">
            <button type="submit" disabled={loading} class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 disabled:opacity-50">
              {loading ? '保存中...' : '保存'}
            </button>
            <button type="button" onclick={() => editingArticle = null} class="bg-gray-200 text-gray-700 px-6 py-2 rounded hover:bg-gray-300">
              キャンセル
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if listError}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{listError}</div>
  {/if}

  {#if articles.length === 0}
    <div class="text-center py-16 text-gray-400 border rounded">
      <p class="text-lg">記事がありません</p>
      <p class="text-sm mt-1">「+ 新規作成」ボタンから作成してください</p>
    </div>
  {:else}
    <div class="border rounded overflow-hidden">
      <table class="w-full text-sm">
        <thead class="bg-gray-50 border-b">
          <tr>
            <th class="text-left px-4 py-3 font-medium text-gray-600">タイトル</th>
            <th class="text-left px-4 py-3 font-medium text-gray-600 hidden md:table-cell">Slug</th>
            <th class="text-center px-4 py-3 font-medium text-gray-600">状態</th>
            <th class="text-right px-4 py-3 font-medium text-gray-600">操作</th>
          </tr>
        </thead>
        <tbody>
          {#each articles as article}
            <tr class="border-b last:border-0 hover:bg-gray-50">
              <td class="px-4 py-3 font-medium">{article.title}</td>
              <td class="px-4 py-3 text-gray-500 font-mono text-xs hidden md:table-cell">{article.slug}</td>
              <td class="px-4 py-3 text-center">
                {#if article.published}
                  <span class="inline-block bg-green-100 text-green-700 text-xs px-2 py-1 rounded-full font-medium">公開中</span>
                {:else}
                  <span class="inline-block bg-gray-100 text-gray-500 text-xs px-2 py-1 rounded-full font-medium">下書き</span>
                {/if}
              </td>
              <td class="px-4 py-3 text-right">
                <div class="flex justify-end gap-2">
                  <button
                    onclick={() => startEdit(article)}
                    class="text-blue-600 hover:underline text-sm px-2 py-1"
                  >編集</button>
                  <button
                    onclick={() => handleDelete(article)}
                    class="text-red-500 hover:underline text-sm px-2 py-1"
                  >削除</button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
