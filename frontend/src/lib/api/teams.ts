import type {
    CreateTeamRequestDto,
    CreateTeamResponseDto,
    TeamResponseDto,
    UpdateTeamRequestDto,
    PokemonDataDto
} from "../types/api";

const API_BASE = "http://localhost:8080/api";

/**
 * チームを作成する
 */
export async function createTeam(
    request: Omit<CreateTeamRequestDto, "owner_id">, // owner_idはバックエンドで付与
    jwtToken: string
): Promise<CreateTeamResponseDto> {
    const response = await fetch(`${API_BASE}/teams`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${jwtToken}`,
        },
        body: JSON.stringify(request),
    });

    if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to create team");
    }

    return response.json();
}

/**
 * チームを取得する
 */
export async function getTeam(
    teamId: string,
    jwtToken: string
): Promise<TeamResponseDto> {
    const response = await fetch(`${API_BASE}/teams/${teamId}`, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${jwtToken}`,
        },
    });

    if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to get team");
    }

    return response.json();
}

/**
 * ユーザーのチーム一覧を取得する
 */
export async function getUserTeams(
    userId: string,
    jwtToken: string
): Promise<TeamResponseDto[]> {
    const response = await fetch(`${API_BASE}/users/${userId}/teams`, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${jwtToken}`,
        },
    });

    if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to get user teams");
    }

    return response.json();
}

/**
 * チームを更新する
 */
export async function updateTeam(
    teamId: string,
    request: UpdateTeamRequestDto,
    jwtToken: string
): Promise<TeamResponseDto> {
    const response = await fetch(`${API_BASE}/teams/${teamId}`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${jwtToken}`,
        },
        body: JSON.stringify(request),
    });

    if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to update team");
    }

    return response.json();
}

/**
 * チームを削除する
 */
export async function deleteTeam(
    teamId: string,
    jwtToken: string
): Promise<void> {
    const response = await fetch(`${API_BASE}/teams/${teamId}`, {
        method: "DELETE",
        headers: {
            Authorization: `Bearer ${jwtToken}`,
        },
    });

    if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || "Failed to delete team");
    }
}
