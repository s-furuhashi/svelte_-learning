import { apiFetch } from '$lib/api';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
  const res = await apiFetch(`/articles/${params.slug}`, {}, fetch);
  if (res.status === 404) throw error(404, 'Article not found');
  if (!res.ok) throw error(500, 'Failed to load article');
  const data = await res.json();
  const article = data.article;

  // Strip HTML tags for description meta, truncate at word boundary (max 150 chars)
  const stripped = article.html ? article.html.replace(/<[^>]+>/g, '').trim() : '';
  const description =
    stripped.length <= 150
      ? stripped
      : stripped.slice(0, 150).replace(/\s\S*$/, '') + '...';

  return { article, description };
};
