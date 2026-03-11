<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import type { PokemonMasterDto } from "./types/api";
    import TypeBadge from "./TypeBadge.svelte";
    import type { PokemonType } from "../types/pokemon";

    const dispatch = createEventDispatcher<{
        select: PokemonMasterDto;
        close: void;
    }>();

    export let pokemonList: PokemonMasterDto[] = [];
    export let title: string = "ポケモンを選択";

    let searchQuery = "";
    let filteredList: PokemonMasterDto[] = [];

    $: {
        if (searchQuery.trim() === "") {
            filteredList = pokemonList;
        } else {
            const query = searchQuery.toLowerCase();
            filteredList = pokemonList.filter(
                (p) =>
                    p.fullname.toLowerCase().includes(query) ||
                    p.fullname_ja?.toLowerCase().includes(query),
            );
        }
    }

    function handleSelect(pokemon: PokemonMasterDto) {
        dispatch("select", pokemon);
    }
</script>

<div class="modal-backdrop" on:click={() => dispatch("close")}>
    <div class="modal-content" on:click|stopPropagation>
        <div class="modal-header">
            <h3>{title}</h3>
            <button class="close-btn" on:click={() => dispatch("close")}
                >×</button
            >
        </div>

        <div class="search-bar">
            <input
                type="text"
                bind:value={searchQuery}
                placeholder="ポケモン名で検索..."
                class="search-input"
            />
        </div>

        <div class="modal-body">
            {#if filteredList.length === 0}
                <div class="empty-state">
                    {#if searchQuery}
                        検索結果が見つかりませんでした
                    {:else}
                        ポケモンが登録されていません
                    {/if}
                </div>
            {:else}
                <div class="pokemon-list">
                    {#each filteredList as pokemon}
                        <button
                            class="pokemon-item"
                            on:click={() => handleSelect(pokemon)}
                        >
                            <div class="pokemon-info">
                                <span class="name"
                                    >{pokemon.fullname_ja ||
                                        pokemon.fullname}</span
                                >
                                <span class="species">{pokemon.fullname}</span>
                            </div>
                            <div class="pokemon-details">
                                <TypeBadge
                                    type={pokemon.type1 as PokemonType}
                                    size="sm"
                                />
                                {#if pokemon.type2}
                                    <TypeBadge
                                        type={pokemon.type2 as PokemonType}
                                        size="sm"
                                    />
                                {/if}
                                {#if pokemon.usage}
                                    <span class="usage"
                                        >{(pokemon.usage * 100).toFixed(
                                            1,
                                        )}%</span
                                    >
                                {/if}
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
        max-width: 700px;
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

    .search-bar {
        padding: 1rem;
        border-bottom: 1px solid #eee;
    }

    .search-input {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 1rem;
    }

    .modal-body {
        padding: 1rem;
        overflow-y: auto;
        flex: 1;
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
        width: 100%;
    }

    .pokemon-item:hover {
        background-color: #f0f7ff;
        border-color: #4a90e2;
    }

    .pokemon-info {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .name {
        font-weight: 600;
        color: #333;
        font-size: 1rem;
    }

    .species {
        font-size: 0.85rem;
        color: #888;
    }

    .pokemon-details {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .usage {
        font-size: 0.85rem;
        color: #28a745;
        font-weight: 600;
    }

    .empty-state {
        text-align: center;
        padding: 3rem 2rem;
        color: #999;
        font-size: 1rem;
    }
</style>
