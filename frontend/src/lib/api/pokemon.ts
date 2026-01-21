import * as v from 'valibot';
import { post, get } from './client';
import type {
    CreatePokemonRequestDto,
    CreatePokemonResponseDto,
    PokemonResponseDto,
    UpdatePokemonRequestDto,
    PokemonErrorResponseDto,
} from '../types/api';

// Valibotスキーマ定義
const CreatePokemonResponseSchema = v.object({
    pokemon_id: v.string(),
    user_id: v.string(),
    nickname: v.nullable(v.string()),
    form_id: v.number(),
    species_id: v.number(),
    fullname: v.string(),
    fullname_jp: v.string(),
});

const PokemonResponseSchema = v.object({
    pokemon_id: v.string(),
    user_id: v.string(),
    nickname: v.nullable(v.string()),
    form_id: v.number(),
    species_id: v.number(),
    fullname: v.string(),
    fullname_jp: v.string(),
    terastal_type: v.string(),
    ev_hp: v.number(),
    ev_attack: v.number(),
    ev_defense: v.number(),
    ev_special_attack: v.number(),
    ev_special_defense: v.number(),
    ev_speed: v.number(),
    iv_hp: v.number(),
    iv_attack: v.number(),
    iv_defense: v.number(),
    iv_special_attack: v.number(),
    iv_special_defense: v.number(),
    iv_speed: v.number(),
    nature: v.string(),
    ability: v.string(),
    held_item: v.nullable(v.string()),
    moves: v.array(v.string()),
});

const PokemonListResponseSchema = v.array(PokemonResponseSchema);

/**
 * ポケモンを登録
 */
export async function createPokemon(
    request: CreatePokemonRequestDto,
    token?: string
): Promise<CreatePokemonResponseDto> {
    return post('/pokemon', request, CreatePokemonResponseSchema, { token }) as unknown as Promise<CreatePokemonResponseDto>;
}

/**
 * ポケモンを取得
 */
export async function getPokemon(
    pokemonId: string,
    token?: string
): Promise<PokemonResponseDto> {
    return get(`/pokemon/${pokemonId}`, PokemonResponseSchema, { token }) as unknown as Promise<PokemonResponseDto>;
}

/**
 * ユーザーのポケモン一覧を取得
 * JWTトークンから自動的にユーザーIDを取得
 */
export async function getUserPokemon(
    token?: string
): Promise<PokemonResponseDto[]> {
    return get('/pokemon', PokemonListResponseSchema, { token }) as unknown as Promise<PokemonResponseDto[]>;
}

/**
 * ポケモンを更新
 * JWTトークンから自動的にユーザーIDを取得
 */
export async function updatePokemon(
    pokemonId: string,
    request: UpdatePokemonRequestDto,
    token?: string
): Promise<PokemonResponseDto> {
    return post(
        `/pokemon/${pokemonId}`,
        request,
        PokemonResponseSchema,
        { token }
    ) as unknown as Promise<PokemonResponseDto>;
}

/**
 * ポケモンを削除
 * JWTトークンから自動的にユーザーIDを取得
 */
export async function deletePokemon(
    pokemonId: string,
    token?: string,
    isRetry: boolean = false
): Promise<void> {
    const response = await fetch(
        `${import.meta.env.VITE_API_URL || 'http://localhost:8080/api'}/pokemon/${pokemonId}`,
        {
            method: 'DELETE',
            headers: {
                ...(token ? { Authorization: `Bearer ${token}` } : {}),
            },
            credentials: 'include', // Cookieを送信
        }
    );

    // 401エラーで、リトライでない場合、トークンをリフレッシュしてリトライ
    if (response.status === 401 && !isRetry) {
        try {
            const refreshResponse = await fetch(
                `${import.meta.env.VITE_API_URL || 'http://localhost:8080/api'}/auth/refresh`,
                {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    credentials: 'include', // Cookieを送信
                }
            );

            if (refreshResponse.ok) {
                const data = await refreshResponse.json();
                const newAccessToken = data.access_token;

                // LocalStorageを更新
                localStorage.setItem('access_token', newAccessToken);

                // イベントを発火
                if (typeof window !== 'undefined') {
                    window.dispatchEvent(new CustomEvent('token-refreshed', {
                        detail: { access_token: newAccessToken }
                    }));
                }

                // リトライ
                return deletePokemon(pokemonId, newAccessToken, true);
            }
        } catch (error) {
            // リフレッシュ失敗
        }

        // リフレッシュ失敗時はログアウト
        if (typeof window !== 'undefined') {
            window.dispatchEvent(new Event('auth-logout'));
            localStorage.clear();
        }
    }

    if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to delete pokemon');
    }
}
