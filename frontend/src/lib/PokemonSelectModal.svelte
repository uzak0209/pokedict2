<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import { getUserPokemon } from "./api/pokemon";
    import { authStore } from "../stores/auth";
    import type { PokemonResponseDto } from "./types/api";
    import TypeBadge from "./TypeBadge.svelte";

    const dispatch = createEventDispatcher<{
        select: PokemonResponseDto;
        close: void;
    }>();

    let pokemonList: PokemonResponseDto[] = [];
    let loading = true;
    let error: string | null = null;
    let accessToken = ""; // subscribe in onMount or reactive statement

    authStore.subscribe((state) => {
        accessToken = state.accessToken || "";
    });

    onMount(async () => {
        await loadPokemon();
    });

    async function loadPokemon() {
        loading = true;
        try {
            pokemonList = await getUserPokemon(accessToken);
        } catch (e: any) {
            error = e.message || "ポケモンの読み込みに失敗しました";
        } finally {
            loading = false;
        }
    }

    function handleSelect(pokemon: PokemonResponseDto) {
        dispatch("select", pokemon);
    }
</script>

<div class="modal-backdrop" on:click={() => dispatch("close")}>
    <div class="modal-content" on:click|stopPropagation>
        <div class="modal-header">
            <h3>ポケモンを選択</h3>
            <button class="close-btn" on:click={() => dispatch("close")}
                >×</button
            >
        </div>

        <div class="modal-body">
            {#if loading}
                <div class="loading">読み込み中...</div>
            {:else if error}
                <div class="error">{error}</div>
            {:else if pokemonList.length === 0}
                <div class="empty-state">
                    ポケモンが登録されていません。<br />
                    先に「ポケモン管理」タブでポケモンを登録してください。
                </div>
            {:else}
                <div class="pokemon-list">
                    {#each pokemonList as pokemon}
                        <button
                            class="pokemon-item"
                            on:click={() => handleSelect(pokemon)}
                        >
                            <div class="flex items-center gap-3">
                                <img
                                    src={`/icons/pokemon/${pokemon.form_id}.png`}
                                    alt={pokemon.fullname}
                                    class="w-10 h-10 pixelated"
                                />
                                <div class="pokemon-info">
                                    <span class="name"
                                        >{pokemon.nickname ||
                                            pokemon.fullname_jp}</span
                                    >
                                    <span class="species"
                                        >{pokemon.fullname_jp}</span
                                    >
                                </div>
                            </div>
                            <div class="pokemon-details">
                                <div class="flex gap-1 mb-1">
                                    <TypeBadge
                                        type={pokemon.type1 as any}
                                        size="sm"
                                    />
                                    {#if pokemon.type2}
                                        <TypeBadge
                                            type={pokemon.type2 as any}
                                            size="sm"
                                        />
                                    {/if}
                                </div>
                                <span class="text-xs text-accents-5"
                                    >Tera: <TypeBadge
                                        type={pokemon.terastal_type as any}
                                        size="sm"
                                    /></span
                                >
                                <span class="item"
                                    >{pokemon.held_item || "なし"}</span
                                >
                            </div>
                        </button>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .modal-content {
        background: white;
        border-radius: 8px;
        width: 90%;
        max-width: 600px;
        max-height: 80vh;
        display: flex;
        flex-direction: column;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .modal-header {
        padding: 1rem;
        border-bottom: 1px solid #eee;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .modal-header h3 {
        margin: 0;
        font-size: 1.25rem;
        color: #333;
    }

    .close-btn {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: #666;
    }

    .modal-body {
        padding: 1rem;
        overflow-y: auto;
    }

    .pokemon-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .pokemon-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem;
        border: 1px solid #eee;
        border-radius: 6px;
        background: white;
        cursor: pointer;
        transition: background-color 0.2s;
        text-align: left;
    }

    .pokemon-item:hover {
        background-color: #f5f5f5;
    }

    .pokemon-info {
        display: flex;
        flex-direction: column;
    }

    .name {
        font-weight: 600;
        color: #333;
    }

    .species {
        font-size: 0.85rem;
        color: #888;
    }

    .pokemon-details {
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        gap: 0.25rem;
    }

    .loading,
    .error,
    .empty-state {
        text-align: center;
        padding: 2rem;
        color: #666;
    }

    .error {
        color: #d32f2f;
    }
</style>
