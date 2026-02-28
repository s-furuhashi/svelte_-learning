<script lang="ts">
  import '../app.css';
  import type { Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();
</script>

<nav class="bg-gray-800 text-white p-4">
  <div class="container mx-auto flex gap-6">
    <a href="/" class="font-bold text-xl">My HP</a>
    <a href="/articles" class="hover:text-gray-300">Articles</a>
    <a href="/books" class="hover:text-gray-300">Books</a>
    <a href="/about" class="hover:text-gray-300">About</a>
  </div>
</nav>

<main class="container mx-auto p-4 min-h-[calc(100vh-8rem)]">
  {@render children()}
</main>

<footer class="bg-gray-100 text-gray-500 text-sm text-center p-4 mt-8">
  <p>© {new Date().getFullYear()} My HP. All rights reserved.</p>
</footer>
