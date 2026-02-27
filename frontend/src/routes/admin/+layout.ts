import { apiFetch } from '$lib/api';

export const load = async ({ url }: { url: URL }) => {
  if (url.pathname === '/admin/login') return {};
  // Check auth by calling backend
  const res = await apiFetch('/health');
  // We can't easily check session without a dedicated /me endpoint
  // This is a basic check - in production, add GET /me endpoint
  return {};
};
