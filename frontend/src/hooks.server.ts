import type { HandleFetch } from '@sveltejs/kit';

// SSR時、コンテナ内から localhost:3000 ではなく backend:3000 を使うようリライト
export const handleFetch: HandleFetch = ({ request, fetch }) => {
	const url = new URL(request.url);
	const internalApiUrl = process.env.INTERNAL_API_URL;

	if (internalApiUrl && (url.hostname === 'localhost' || url.hostname === '127.0.0.1')) {
		const internal = new URL(internalApiUrl);
		if (url.port === internal.port || url.port === '3000') {
			const newUrl = new URL(request.url);
			newUrl.hostname = internal.hostname;
			newUrl.port = internal.port;
			newUrl.protocol = internal.protocol;
			return fetch(new Request(newUrl.toString(), request));
		}
	}

	return fetch(request);
};
