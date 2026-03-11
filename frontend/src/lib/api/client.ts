import * as v from 'valibot';

const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

export class ApiError extends Error {
    constructor(public status: number, public message: string, public code?: string) {
        super(message);
        this.name = 'ApiError';
    }
}

// トークンリフレッシュ中フラグ
let isRefreshing = false;
let refreshPromise: Promise<string> | null = null;

// トークンをリフレッシュする関数 (Cookieから自動的にrefresh_tokenを読み取る)
async function refreshAccessToken(): Promise<string> {
    if (isRefreshing && refreshPromise) {
        return refreshPromise;
    }

    isRefreshing = true;
    refreshPromise = (async () => {
        try {
            // refresh_tokenはHTTPOnly Cookieで管理されているため、ボディは空
            const response = await fetch(`${BASE_URL}/auth/refresh`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include', // Cookieを送信するために必須
            });

            if (!response.ok) {
                throw new Error('Token refresh failed');
            }

            const data = await response.json();
            const newAccessToken = data.access_token;

            // LocalStorageを更新 (refresh_tokenはCookieで自動管理)
            localStorage.setItem('access_token', newAccessToken);

            // authストアも更新するためにカスタムイベントを発火
            if (typeof window !== 'undefined') {
                window.dispatchEvent(new CustomEvent('token-refreshed', {
                    detail: { access_token: newAccessToken }
                }));
            }

            return newAccessToken;
        } finally {
            isRefreshing = false;
            refreshPromise = null;
        }
    })();

    return refreshPromise;
}

// 汎用的なレスポンス処理
async function handleResponse<T extends v.GenericSchema>(
    response: Response,
    schema: T
): Promise<v.InferOutput<T>> {
    if (!response.ok) {
        let errorMessage = 'Unknown error';
        let errorCode = 'UNKNOWN';
        try {
            const errorData = await response.json();
            if (errorData && typeof errorData === 'object') {
                errorMessage = errorData.error || errorMessage;
                errorCode = errorData.error_code || errorCode;
            }
        } catch {
            // ignore json parse error
        }
        throw new ApiError(response.status, errorMessage, errorCode);
    }

    // ステータスコードが204 (No Content) の場合はnullを返す（スキーマがnull許容であることを期待するか、呼び出し元でハンドリング）
    if (response.status === 204) {
        return null as any;
    }

    const data = await response.json();
    // Valibotでバリデーション
    return v.parse(schema, data);
}

interface RequestOptions {
    headers?: Record<string, string>;
    token?: string;
}

// POST request
export async function post<T extends v.GenericSchema, B>(
    endpoint: string,
    body: B,
    responseSchema: T,
    options: RequestOptions = {},
    isRetry: boolean = false
): Promise<v.InferOutput<T>> {
    const headers: HeadersInit = {
        'Content-Type': 'application/json',
        ...(options.headers || {}),
    };

    if (options.token) {
        headers['Authorization'] = `Bearer ${options.token}`;
    }

    const response = await fetch(`${BASE_URL}${endpoint}`, {
        method: 'POST',
        headers,
        body: JSON.stringify(body),
        credentials: 'include', // Cookieを送信
    });

    // 401エラーで、リトライでない場合、トークンをリフレッシュしてリトライ
    if (response.status === 401 && !isRetry && !endpoint.includes('/auth/')) {
        try {
            const newToken = await refreshAccessToken();
            return post(endpoint, body, responseSchema, { ...options, token: newToken }, true);
        } catch (error) {
            // リフレッシュ失敗時はログアウト
            if (typeof window !== 'undefined') {
                window.dispatchEvent(new Event('auth-logout'));
                localStorage.clear();
            }
            throw error;
        }
    }

    return handleResponse(response, responseSchema);
}

// GET request (必要に応じて追加)
export async function get<T extends v.GenericSchema>(
    endpoint: string,
    responseSchema: T,
    options: RequestOptions = {},
    isRetry: boolean = false
): Promise<v.InferOutput<T>> {
    const headers: HeadersInit = {
        ...(options.headers || {}),
    };

    if (options.token) {
        headers['Authorization'] = `Bearer ${options.token}`;
    }

    const response = await fetch(`${BASE_URL}${endpoint}`, {
        method: 'GET',
        headers,
        credentials: 'include', // Cookieを送信
    });

    // 401エラーで、リトライでない場合、トークンをリフレッシュしてリトライ
    if (response.status === 401 && !isRetry && !endpoint.includes('/auth/')) {
        try {
            const newToken = await refreshAccessToken();
            return get(endpoint, responseSchema, { ...options, token: newToken }, true);
        } catch (error) {
            // リフレッシュ失敗時はログアウト
            if (typeof window !== 'undefined') {
                window.dispatchEvent(new Event('auth-logout'));
                localStorage.clear();
            }
            throw error;
        }
    }

    return handleResponse(response, responseSchema);
}

// PUT request
export async function put<T extends v.GenericSchema, B>(
    endpoint: string,
    body: B,
    responseSchema: T,
    options: RequestOptions = {},
    isRetry: boolean = false
): Promise<v.InferOutput<T>> {
    const headers: HeadersInit = {
        'Content-Type': 'application/json',
        ...(options.headers || {}),
    };

    if (options.token) {
        headers['Authorization'] = `Bearer ${options.token}`;
    }

    const response = await fetch(`${BASE_URL}${endpoint}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(body),
        credentials: 'include',
    });

    if (response.status === 401 && !isRetry && !endpoint.includes('/auth/')) {
        try {
            const newToken = await refreshAccessToken();
            return put(endpoint, body, responseSchema, { ...options, token: newToken }, true);
        } catch (error) {
            if (typeof window !== 'undefined') {
                window.dispatchEvent(new Event('auth-logout'));
                localStorage.clear();
            }
            throw error;
        }
    }

    return handleResponse(response, responseSchema);
}

// --- Team Suggestion Schemas ---

export const MatrixPokemonSchema = v.object({
    form_id: v.number(),
    name: v.string(),
    name_ja: v.nullable(v.string()),
    is_setup: v.boolean(),
});

export const SuggestedPokemonSchema = v.object({
    form_id: v.number(),
    name: v.string(),
    name_ja: v.nullable(v.string()),
    is_setup: v.boolean(),
    score: v.number(),
    covered_threats: v.array(MatrixPokemonSchema),
});

export const TeamSuggestionResponseSchema = v.object({
    all_threats: v.array(MatrixPokemonSchema),
    axis_pokemon: v.array(SuggestedPokemonSchema),
    suggestions: v.array(SuggestedPokemonSchema),
});

export const SuggestedPokemonWithReasoningSchema = v.object({
    form_id: v.number(),
    name: v.string(),
    name_ja: v.nullable(v.string()),
    is_setup: v.boolean(),
    score: v.number(),
    covered_threats: v.array(MatrixPokemonSchema),
    reasoning: v.nullable(v.string()),
});

export const TeamSuggestionWithReasoningResponseSchema = v.object({
    all_threats: v.array(MatrixPokemonSchema),
    axis_pokemon: v.array(SuggestedPokemonSchema),
    suggestions: v.array(SuggestedPokemonWithReasoningSchema),
});

export type MatrixPokemon = v.InferOutput<typeof MatrixPokemonSchema>;
export type SuggestedPokemon = v.InferOutput<typeof SuggestedPokemonSchema>;
export type TeamSuggestionResponse = v.InferOutput<typeof TeamSuggestionResponseSchema>;
export type SuggestedPokemonWithReasoning = v.InferOutput<typeof SuggestedPokemonWithReasoningSchema>;
export type TeamSuggestionWithReasoningResponse = v.InferOutput<typeof TeamSuggestionWithReasoningResponseSchema>;

// --- Matchup Correction API ---

export const SaveMatchupOverrideRequestSchema = v.object({
    form_id: v.number(),
    opponent_form_id: v.number(),
    judgment: v.number(), // 1: Win, -1: Lose, 0: Even
});

export async function saveMatchupOverride(token: string, form_id: number, opponent_form_id: number, judgment: number): Promise<void> {

    // 204 No Content response expected (or 200 OK empty)
    // post handles empty response if we pass a schema processing void?
    // Using v.any() or v.void() if supported. Or just v.unknown().

    await post(
        '/pokemon/master/matchups/override',
        { form_id, opponent_form_id, judgment },
        v.unknown(), // we don't care about response body for now
        { token }
    );
}

// --- Team API ---

export async function suggestTeamComplements(team_form_ids: number[], token?: string): Promise<TeamSuggestionResponse> {
    return post(
        '/pokemon/master/team/suggest',
        { team: team_form_ids },
        TeamSuggestionResponseSchema,
        token ? { token } : {}
    );
}

export async function suggestTeamComplementsWithReasoning(team_form_ids: number[], token?: string): Promise<TeamSuggestionWithReasoningResponse> {
    return post(
        '/pokemon/master/team/suggest-with-reasoning',
        { team: team_form_ids },
        TeamSuggestionWithReasoningResponseSchema,
        token ? { token } : {}
    );
}

// --- Matrix API ---

export const MatrixCellSchema = v.object({
    form_id: v.number(),
    opponent_form_id: v.number(),
    p: v.number(),
    n: v.number(),
});

export const MatchupMatrixDtoSchema = v.object({
    pokemon: v.array(MatrixPokemonSchema),
    cells: v.array(MatrixCellSchema),
});

export type MatchupMatrixDto = v.InferOutput<typeof MatchupMatrixDtoSchema>;

export async function getMatchupMatrix(limit: number = 30, token?: string): Promise<MatchupMatrixDto> {
    const params = new URLSearchParams({ limit: limit.toString() });
    return get(
        `/pokemon/master/matrix?${params.toString()}`,
        MatchupMatrixDtoSchema,
        token ? { token } : {}
    );
}

// DELETE request
export async function del(
    endpoint: string,
    options: RequestOptions = {},
    isRetry: boolean = false
): Promise<void> {
    const headers: HeadersInit = {
        ...(options.headers || {}),
    };

    if (options.token) {
        headers['Authorization'] = `Bearer ${options.token}`;
    }

    const response = await fetch(`${BASE_URL}${endpoint}`, {
        method: 'DELETE',
        headers,
        credentials: 'include',
    });

    if (response.status === 401 && !isRetry && !endpoint.includes('/auth/')) {
        try {
            const newToken = await refreshAccessToken();
            return del(endpoint, { ...options, token: newToken }, true);
        } catch (error) {
            if (typeof window !== 'undefined') {
                window.dispatchEvent(new Event('auth-logout'));
                localStorage.clear();
            }
            throw error;
        }
    }

    if (!response.ok && response.status !== 204) {
        throw new ApiError(response.status, "Delete failed");
    }
}
