import { redirect } from '@sveltejs/kit';
import type { ServerLoad } from '@sveltejs/kit';

// サーバー内部通信用URL（Dockerネットワーク内）、なければ外部URL
const API_BASE = process.env.INTERNAL_API_URL || 'http://localhost:3000';

export const load: ServerLoad = async ({ url, cookies }) => {
  // ログインページ自体はチェック不要
  if (url.pathname === '/admin/login') return {};

  const sessionId = cookies.get('session_id');
  if (!sessionId) {
    throw redirect(302, '/admin/login');
  }

  // SSRでバックエンドに直接リクエスト（Cookieを明示的に転送）
  const res = await fetch(`${API_BASE}/me`, {
    headers: {
      'Content-Type': 'application/json',
      'Cookie': `session_id=${sessionId}`,
    },
  });

  if (!res.ok) {
    throw redirect(302, '/admin/login');
  }

  const data = await res.json();
  return { user: data };
};
