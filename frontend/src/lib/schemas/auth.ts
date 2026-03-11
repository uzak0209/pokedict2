import * as v from 'valibot';
import { z } from 'zod';
import type {
    RegisterRequestDto,
    RegisterResponseDto,
    LoginRequestDto,
    TokenResponseDto,
    RefreshRequestDto,
    RefreshResponseDto
} from '../types/api';

// --- Valibot Schemas ---

export const RegisterRequestSchema = v.object({
    username: v.pipe(v.string(), v.minLength(3), v.maxLength(50)),
    email: v.pipe(v.string(), v.email()),
    password: v.pipe(v.string(), v.minLength(8)),
});

export type RegisterRequest = RegisterRequestDto;

export const RegisterResponseSchema = v.object({
    user_id: v.string(),
    username: v.string(),
    email: v.string(),
});

export type RegisterResponse = RegisterResponseDto;

export const LoginRequestSchema = v.object({
    email: v.pipe(v.string(), v.email()),
    password: v.pipe(v.string(), v.minLength(8)),
});

export type LoginRequest = LoginRequestDto;

export const TokenResponseSchema = v.object({
    user_id: v.string(),
    username: v.string(),
    email: v.string(),
    access_token: v.string(),
    token_type: v.string(),
    expires_in: v.number(),
});

export type TokenResponse = TokenResponseDto;

export const RefreshRequestSchema = v.object({
    refresh_token: v.string(),
});

export type RefreshRequest = RefreshRequestDto;

export const RefreshResponseSchema = v.object({
    access_token: v.string(),
    token_type: v.string(),
    expires_in: v.number(),
});

export type RefreshResponse = RefreshResponseDto;

// --- Zod Schemas (for reference/compatibility) ---

export const RegisterRequestZod = z.object({
    username: z.string().min(3).max(50),
    email: z.string().email(),
    password: z.string().min(8),
});

export const LoginRequestZod = z.object({
    email: z.string().email(),
    password: z.string().min(8),
});
