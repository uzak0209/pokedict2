<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { getUserTeams, deleteTeam } from "./api/teams";
    import type { TeamResponseDto } from "./types/api";
    import { authStore } from "../stores/auth";

    export let userId: string;

    const dispatch = createEventDispatcher();

    let teams: TeamResponseDto[] = [];
    let loading = true;
    let error: string | null = null;
    let accessToken: string = "";

    authStore.subscribe((state) => {
        accessToken = state.accessToken || "";
    });

    onMount(async () => {
        await loadTeams();
    });

    async function loadTeams() {
        loading = true;
        error = null;
        try {
            teams = await getUserTeams(userId, accessToken);
        } catch (e: any) {
            error = e.message;
        } finally {
            loading = false;
        }
    }

    function handleCreate() {
        // PokemonManagerで "/teams/new" への遷移を処理してもらうため、
        // 単純に親コンポーネントの責務ではなくハッシュ変更をここでやるか、
        // あるいは親にイベントを送るか。
        // PokemonManager.svelte では、TeamListからイベントを受け取るようにはなっていなかったが、
        // ハッシュ変更で制御しているので、直接ハッシュを変えるのが手っ取り早い。
        window.location.hash = "/teams/new";
    }

    function handleEdit(teamId: string) {
        dispatch("edit", teamId);
    }

    async function handleDelete(teamId: string) {
        if (!confirm("本当にこのチームを削除しますか？")) return;

        try {
            await deleteTeam(teamId, accessToken);
            teams = teams.filter((t) => t.team_id !== teamId);
        } catch (e: any) {
            alert("削除に失敗しました: " + e.message);
        }
    }
</script>

<div class="team-list-container">
    <div class="header">
        <h2>マイチーム一覧</h2>
        <button class="btn-create" on:click={handleCreate}>
            + 新しいチームを作成
        </button>
    </div>

    {#if loading}
        <div class="loading">読み込み中...</div>
    {:else if error}
        <div class="error">{error}</div>
    {:else if teams.length === 0}
        <div class="empty-state">
            <p>まだチームがありません。</p>
            <button class="btn-primary" on:click={handleCreate}>
                最初のチームを作成する
            </button>
        </div>
    {:else}
        <div class="team-grid">
            {#each teams as team}
                <div class="team-card">
                    <div class="team-header">
                        <h3>{team.team_name}</h3>
                        <div class="actions">
                            <button
                                class="btn-icon edit"
                                on:click={() => handleEdit(team.team_id)}
                                title="編集"
                            >
                                ✎
                            </button>
                            <button
                                class="btn-icon delete"
                                on:click={() => handleDelete(team.team_id)}
                                title="削除"
                            >
                                🗑️
                            </button>
                        </div>
                    </div>
                    <div class="pokemon-icons">
                        {#each team.pokemon as p}
                            <div class="pokemon-icon" title={p.fullname_jp}>
                                <!-- アイコン画像があればここに表示。なければ名前の先頭文字など -->
                                <div class="placeholder-icon">
                                    {p.fullname_jp.charAt(0)}
                                </div>
                                <span class="pokemon-name">{p.fullname_jp}</span
                                >
                            </div>
                        {/each}
                        {#each Array(6 - team.pokemon.length) as _}
                            <div class="pokemon-icon empty">
                                <span class="empty-slot">-</span>
                            </div>
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .team-list-container {
        max-width: 1000px;
        margin: 0 auto;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .header h2 {
        margin: 0;
        color: #333;
    }

    .btn-create {
        background-color: #4caf50;
        color: white;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 4px;
        font-weight: 600;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-create:hover {
        background-color: #43a047;
    }

    .team-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1.5rem;
    }

    .team-card {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        padding: 1.5rem;
        transition:
            transform 0.2s,
            box-shadow 0.2s;
    }

    .team-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
    }

    .team-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1rem;
    }

    .team-header h3 {
        margin: 0;
        font-size: 1.25rem;
        color: #333;
    }

    .actions {
        display: flex;
        gap: 0.5rem;
    }

    .btn-icon {
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.2rem;
        padding: 0.25rem;
        border-radius: 4px;
        transition: background 0.2s;
    }

    .btn-icon:hover {
        background-color: #f0f0f0;
    }

    .btn-icon.delete:hover {
        color: #d32f2f;
        background-color: #ffebee;
    }

    .pokemon-icons {
        display: flex;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .pokemon-icon {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 40px;
    }

    .placeholder-icon {
        width: 32px;
        height: 32px;
        background-color: #e0e0e0;
        border-radius: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 0.8rem;
        color: #666;
        margin-bottom: 0.25rem;
    }

    .pokemon-icon.empty .empty-slot {
        color: #ccc;
        font-size: 1.5rem;
    }

    .pokemon-name {
        font-size: 0.6rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 100%;
    }

    .loading,
    .error,
    .empty-state {
        text-align: center;
        padding: 3rem;
        color: #666;
    }

    .error {
        color: #d32f2f;
    }

    .btn-primary {
        background-color: #2196f3;
        color: white;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 4px;
        cursor: pointer;
        margin-top: 1rem;
    }
</style>
