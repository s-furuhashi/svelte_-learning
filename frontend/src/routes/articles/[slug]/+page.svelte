<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>{data.article.title}</title>
</svelte:head>

<article class="py-8 max-w-3xl">
  <h1 class="text-4xl font-bold mb-2">{data.article.title}</h1>
  <p class="text-gray-500 text-sm mb-8">{new Date(data.article.created_at).toLocaleDateString()}</p>
  <div class="prose max-w-none">
    {@html data.article.html}
  </div>
</article>
