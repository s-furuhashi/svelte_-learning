import { apiFetch } from '$lib/api';
import { redirect } from '@sveltejs/kit';

export const load = async ({ url }: { url: URL }) => {
  if (url.pathname === '/admin/login') return {};
  const res = await apiFetch('/me');
  if (!res.ok) {
    throw redirect(302, '/admin/login');
  }
  const data = await res.json();
  return { user: data };
};
