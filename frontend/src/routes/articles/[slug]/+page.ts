import { apiFetch } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load = async ({ params }: { params: { slug: string } }) => {
  const res = await apiFetch(`/articles/${params.slug}`);
  if (res.status === 404) throw error(404, 'Article not found');
  if (!res.ok) throw error(500, 'Failed to load article');
  const data = await res.json();
  return { article: data.article };
};
