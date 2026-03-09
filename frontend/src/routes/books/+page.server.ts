import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const apiBase = env.INTERNAL_API_URL || 'http://backend:3000';
	const res = await fetch(`${apiBase}/books`);
	if (!res.ok) return { books: [] };
	const data = await res.json();
	return { books: data.books || [] };
};
