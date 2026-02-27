<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>{data.book.title}</title>
</svelte:head>

<article class="py-8 max-w-3xl">
  {#if data.book.image_url}
    <img src={data.book.image_url} alt={data.book.title} class="w-full max-h-80 object-cover rounded mb-6" />
  {/if}
  <h1 class="text-4xl font-bold mb-8">{data.book.title}</h1>
  <div class="prose max-w-none">
    {@html data.book.html}
  </div>
</article>
