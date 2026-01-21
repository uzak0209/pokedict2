<script lang="ts">
    import { onMount } from "svelte";
    import { getUserPokemon, deletePokemon } from "./api/pokemon";
    import type { PokemonResponseDto as PokemonResponse } from "./types/api";

    export let accessToken: string = "";

    let pokemon: PokemonResponse[] = [];
    let loading = true;
    let error: string | null = null;

    onMount(async () => {
        await loadPokemon();
    });

    async function loadPokemon() {
        loading = true;
        error = null;
        try {
            pokemon = await getUserPokemon(accessToken);
        } catch (e: any) {
            error = e.message || "ポケモンの読み込みに失敗しました";
        } finally {
            loading = false;
        }
    }

    async function handleDelete(pokemonId: string) {
        if (!confirm("このポケモンを削除してもよろしいですか？")) {
            return;
        }

        try {
            await deletePokemon(pokemonId, accessToken);
            await loadPokemon();
        } catch (e: any) {
            alert("削除に失敗しました: " + (e.message || "不明なエラー"));
        }
    }

    function formatStats(
        pokemon: PokemonResponse,
        statType: "ev" | "iv",
    ): string {
        if (statType === "ev") {
            return `H${pokemon.ev_hp} A${pokemon.ev_attack} B${pokemon.ev_defense} C${pokemon.ev_special_attack} D${pokemon.ev_special_defense} S${pokemon.ev_speed}`;
        } else {
            return `H${pokemon.iv_hp} A${pokemon.iv_attack} B${pokemon.iv_defense} C${pokemon.iv_special_attack} D${pokemon.iv_special_defense} S${pokemon.iv_speed}`;
        }
    }
</script>

<div class="pokemon-list">
    <div class="header">
        <h2>保存したポケモン</h2>
        <button
            class="btn-primary"
            on:click={() => (window.location.href = "#/pokemon/new")}
        >
            + 新規登録
        </button>
    </div>

    {#if loading}
        <div class="loading">読み込み中...</div>
    {:else if error}
        <div class="error">{error}</div>
    {:else if pokemon.length === 0}
        <div class="empty">
            <p>まだポケモンが登録されていません</p>
            <button
                class="btn-primary"
                on:click={() => (window.location.href = "#/pokemon/new")}
            >
                最初のポケモンを登録する
            </button>
        </div>
    {:else}
        <div class="pokemon-grid">
            {#each pokemon as poke (poke.pokemon_id)}
                <div class="pokemon-card">
                    <div class="card-header">
                        <h3>
                            {poke.nickname ?? poke.fullname_jp ?? poke.fullname}
                        </h3>
                        {#if poke.nickname}
                            <span class="species-name"
                                >({poke.fullname_jp ?? poke.fullname})</span
                            >
                        {/if}
                    </div>

                    <div class="card-body">
                        <div class="info-row">
                            <span class="label">性格:</span>
                            <span>{poke.nature}</span>
                        </div>
                        <div class="info-row">
                            <span class="label">特性:</span>
                            <span>{poke.ability}</span>
                        </div>
                        {#if poke.held_item}
                            <div class="info-row">
                                <span class="label">持ち物:</span>
                                <span>{poke.held_item}</span>
                            </div>
                        {/if}
                        <div class="info-row">
                            <span class="label">テラス:</span>
                            <span>{poke.terastal_type}</span>
                        </div>

                        <div class="stats-section">
                            <div class="info-row">
                                <span class="label">努力値:</span>
                                <span class="stats"
                                    >{formatStats(poke, "ev")}</span
                                >
                            </div>
                            <div class="info-row">
                                <span class="label">個体値:</span>
                                <span class="stats"
                                    >{formatStats(poke, "iv")}</span
                                >
                            </div>
                        </div>

                        <div class="moves-section">
                            <span class="label">技:</span>
                            <div class="moves">
                                {#each poke.moves as move}
                                    <span class="move-badge">{move}</span>
                                {/each}
                            </div>
                        </div>
                    </div>

                    <div class="card-actions">
                        <button
                            class="btn-secondary"
                            on:click={() =>
                                (window.location.href = `#/pokemon/${poke.pokemon_id}`)}
                        >
                            詳細
                        </button>
                        <button
                            class="btn-danger"
                            on:click={() => handleDelete(poke.pokemon_id)}
                        >
                            削除
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .pokemon-list {
        padding: 2rem;
        max-width: 1200px;
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

    .loading,
    .error {
        text-align: center;
        padding: 2rem;
        font-size: 1.2rem;
    }

    .error {
        color: #d32f2f;
    }

    .empty {
        text-align: center;
        padding: 4rem 2rem;
    }

    .empty p {
        font-size: 1.2rem;
        color: #666;
        margin-bottom: 2rem;
    }

    .pokemon-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
        gap: 1.5rem;
    }

    .pokemon-card {
        background: white;
        border: 1px solid #e0e0e0;
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        transition: box-shadow 0.2s;
    }

    .pokemon-card:hover {
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
    }

    .card-header {
        padding: 1rem;
        background: #f5f5f5;
        border-bottom: 1px solid #e0e0e0;
    }

    .card-header h3 {
        margin: 0;
        color: #333;
        font-size: 1.25rem;
    }

    .species-name {
        color: #666;
        font-size: 0.9rem;
        margin-left: 0.5rem;
    }

    .card-body {
        padding: 1rem;
    }

    .info-row {
        display: flex;
        margin-bottom: 0.5rem;
    }

    .label {
        font-weight: 600;
        color: #555;
        min-width: 70px;
        margin-right: 0.5rem;
    }

    .stats-section {
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid #e0e0e0;
    }

    .stats {
        font-family: "Courier New", monospace;
        font-size: 0.9rem;
    }

    .moves-section {
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid #e0e0e0;
    }

    .moves {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        margin-top: 0.5rem;
    }

    .move-badge {
        background: #e3f2fd;
        color: #1976d2;
        padding: 0.25rem 0.75rem;
        border-radius: 16px;
        font-size: 0.85rem;
    }

    .card-actions {
        display: flex;
        gap: 0.5rem;
        padding: 1rem;
        background: #f9f9f9;
        border-top: 1px solid #e0e0e0;
    }

    .btn-primary,
    .btn-secondary,
    .btn-danger {
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        font-size: 0.9rem;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .btn-primary {
        background-color: #4caf50;
        color: white;
    }

    .btn-primary:hover {
        background-color: #45a049;
    }

    .btn-secondary {
        background-color: #f0f0f0;
        color: #333;
        flex: 1;
    }

    .btn-secondary:hover {
        background-color: #e0e0e0;
    }

    .btn-danger {
        background-color: #f44336;
        color: white;
    }

    .btn-danger:hover {
        background-color: #d32f2f;
    }
</style>
