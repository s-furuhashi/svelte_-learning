import { apiFetch } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
  const res = await apiFetch('/articles', {}, fetch);
  if (!res.ok) return { latestArticles: [] };
  const data = await res.json();
  const articles = data.articles || [];
  return { latestArticles: articles.slice(0, 5) };
};
