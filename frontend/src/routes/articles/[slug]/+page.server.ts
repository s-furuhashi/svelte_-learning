import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const apiBase = env.INTERNAL_API_URL || 'http://backend:3000';
	const res = await fetch(`${apiBase}/articles/${params.slug}`);
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
