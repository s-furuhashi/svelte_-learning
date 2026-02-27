<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>Articles</title>
</svelte:head>

<section class="py-8">
  <h1 class="text-3xl font-bold mb-6">Articles</h1>
  {#if data.articles.length === 0}
    <p class="text-gray-500">No articles published yet.</p>
  {:else}
    <div class="space-y-4">
      {#each data.articles as article}
        <article class="border rounded p-4 hover:shadow-md transition-shadow">
          <h2 class="text-xl font-semibold">
            <a href="/articles/{article.slug}" class="text-blue-600 hover:underline">{article.title}</a>
          </h2>
          <p class="text-gray-500 text-sm mt-1">{new Date(article.created_at).toLocaleDateString()}</p>
        </article>
      {/each}
    </div>
  {/if}
</section>
