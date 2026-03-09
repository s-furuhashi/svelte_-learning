import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const apiBase = env.INTERNAL_API_URL || 'http://backend:3000';
	const res = await fetch(`${apiBase}/articles`);
	if (!res.ok) return { latestArticles: [] };
	const data = await res.json();
	const articles = data.articles || [];
	return { latestArticles: articles.slice(0, 5) };
};
