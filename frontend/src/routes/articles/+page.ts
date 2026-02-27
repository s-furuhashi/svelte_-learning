import { apiFetch } from '$lib/api';

export const load = async () => {
  const res = await apiFetch('/articles');
  if (!res.ok) return { articles: [] };
  const data = await res.json();
  return { articles: data.articles || [] };
};
