import { writable } from 'svelte/store';
import type { TokenResponse, RefreshResponse } from '$lib/schemas/auth';

interface AuthState {
    isAuthenticated: boolean;
    user: {
        userId: string;
        username: string;
        email: string;
    } | null;
    accessToken: string | null;
}

const createAuthStore = () => {
    const { subscribe, set, update } = writable<AuthState>({
        isAuthenticated: false,
        user: null,
        accessToken: null,
    });

    return {
        subscribe,
        login: (tokenResponse: TokenResponse) => {
            update(() => ({
                isAuthenticated: true,
                user: {
                    userId: tokenResponse.user_id,
                    username: tokenResponse.username,
                    email: tokenResponse.email,
                },
                accessToken: tokenResponse.access_token,
            }));

            // LocalStorageにaccess_tokenとuserのみを保存（refresh_tokenはCookieで管理）
            if (typeof window !== 'undefined') {
                localStorage.setItem('access_token', tokenResponse.access_token);
                localStorage.setItem('user', JSON.stringify({
                    userId: tokenResponse.user_id,
                    username: tokenResponse.username,
                    email: tokenResponse.email,
                }));
            }
        },
        updateTokens: (refreshResponse: RefreshResponse) => {
            update((state) => ({
                ...state,
                accessToken: refreshResponse.access_token,
            }));

            // LocalStorageのaccess_tokenを更新（refresh_tokenはCookieで管理）
            if (typeof window !== 'undefined') {
                localStorage.setItem('access_token', refreshResponse.access_token);
            }
        },
        logout: () => {
            set({
                isAuthenticated: false,
                user: null,
                accessToken: null,
            });

            // LocalStorageをクリア（refresh_token Cookieはlogout APIで削除）
            if (typeof window !== 'undefined') {
                localStorage.removeItem('access_token');
                localStorage.removeItem('user');
            }
        },
        init: () => {
            // ページ読み込み時にLocalStorageから復元
            if (typeof window !== 'undefined') {
                const accessToken = localStorage.getItem('access_token');
                const userJson = localStorage.getItem('user');

                if (accessToken && userJson) {
                    const user = JSON.parse(userJson);
                    set({
                        isAuthenticated: true,
                        user,
                        accessToken,
                    });
                }
            }
        },
    };
};

export const authStore = createAuthStore();
