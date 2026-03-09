const API_BASE = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

export { API_BASE };

export async function apiFetch(
  path: string,
  options: RequestInit = {},
  fetchFn: typeof fetch = fetch,
) {
  const res = await fetchFn(`${API_BASE}${path}`, {
    ...options,
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });
  return res;
}
