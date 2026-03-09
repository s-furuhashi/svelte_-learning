<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>My HP</title>
</svelte:head>

<section class="py-12">
  <h1 class="text-4xl font-bold mb-4">Welcome</h1>
  <p class="text-gray-600 text-lg mb-8">A personal website and blog platform.</p>
  <div class="flex gap-4 mb-12">
    <a href="/articles" class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700">Articles</a>
    <a href="/books" class="bg-green-600 text-white px-6 py-2 rounded hover:bg-green-700">Books</a>
  </div>

  {#if data.latestArticles.length > 0}
    <div>
      <h2 class="text-2xl font-semibold mb-4">Latest Articles</h2>
      <div class="space-y-3">
        {#each data.latestArticles as article}
          <article class="border rounded p-4 hover:shadow-md transition-shadow">
            <h3 class="text-lg font-semibold">
              <a href="/articles/{article.slug}" class="text-blue-600 hover:underline">{article.title}</a>
            </h3>
            <p class="text-gray-500 text-sm mt-1">{new Date(article.created_at).toLocaleDateString()}</p>
          </article>
        {/each}
      </div>
      <a href="/articles" class="inline-block mt-6 text-blue-600 hover:underline">View all articles →</a>
    </div>
  {/if}
</section>
