import { post } from './client';
import {
    RegisterResponseSchema,
    TokenResponseSchema,
    RegisterRequestSchema,
    LoginRequestSchema,
    RefreshRequestSchema,
    RefreshResponseSchema,
    type RegisterRequest,
    type LoginRequest,
    type RegisterResponse,
    type TokenResponse,
    type RefreshResponse,
} from '../schemas/auth';
import * as v from 'valibot';

export const authApi = {
    register: async (data: RegisterRequest): Promise<RegisterResponse> => {
        // リクエストデータのバリデーション（クライアントサイド）
        const validData = v.parse(RegisterRequestSchema, data);
        return post('/auth/register', validData, RegisterResponseSchema);
    },

    login: async (data: LoginRequest): Promise<TokenResponse> => {
        const validData = v.parse(LoginRequestSchema, data);
        return post('/auth/login', validData, TokenResponseSchema);
    },

    refresh: async (): Promise<RefreshResponse> => {
        // refresh_tokenはCookieから自動的に読み取られる
        const response = await fetch(`${import.meta.env.VITE_API_URL || 'http://localhost:8080/api'}/auth/refresh`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            credentials: 'include', // Cookieを送信
        });

        if (!response.ok) {
            throw new Error('Token refresh failed');
        }

        const data = await response.json();
        return v.parse(RefreshResponseSchema, data);
    },

    logout: async (): Promise<void> => {
        // Logout returns 204 No Content
        // refresh_tokenはCookieから自動的に読み取られるため、ボディは不要
        const response = await fetch(`${import.meta.env.VITE_API_URL || 'http://localhost:8080/api'}/auth/logout`, {
            method: 'POST',
            credentials: 'include', // Cookieを送信
        });

        if (!response.ok && response.status !== 204) {
            throw new Error('Logout failed');
        }
    },
};
