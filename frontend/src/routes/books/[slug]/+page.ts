import { apiFetch } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load = async ({ params }: { params: { slug: string } }) => {
  const res = await apiFetch(`/books/${params.slug}`);
  if (res.status === 404) throw error(404, 'Book not found');
  if (!res.ok) throw error(500, 'Failed to load book');
  const data = await res.json();
  return { book: data.book };
};
