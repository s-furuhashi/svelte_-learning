import { apiFetch } from '$lib/api';

export const load = async () => {
  const res = await apiFetch('/books');
  if (!res.ok) return { books: [] };
  const data = await res.json();
  return { books: data.books || [] };
};
