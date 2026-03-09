import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const apiBase = env.INTERNAL_API_URL || 'http://backend:3000';
	const res = await fetch(`${apiBase}/books/${params.slug}`);
	if (res.status === 404) throw error(404, 'Book not found');
	if (!res.ok) throw error(500, 'Failed to load book');
	const data = await res.json();
	return { book: data.book };
};
