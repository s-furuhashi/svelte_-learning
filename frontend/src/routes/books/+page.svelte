<script lang="ts">
  let { data } = $props();
</script>

<svelte:head>
  <title>Books</title>
</svelte:head>

<section class="py-8">
  <h1 class="text-3xl font-bold mb-6">Books</h1>
  {#if data.books.length === 0}
    <p class="text-gray-500">No books published yet.</p>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each data.books as book}
        <div class="border rounded overflow-hidden hover:shadow-md transition-shadow">
          {#if book.image_url}
            <img src={book.image_url} alt={book.title} class="w-full h-48 object-cover" />
          {/if}
          <div class="p-4">
            <h2 class="text-lg font-semibold">
              <a href="/books/{book.slug}" class="text-blue-600 hover:underline">{book.title}</a>
            </h2>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</section>
