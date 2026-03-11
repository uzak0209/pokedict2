<script lang="ts">
    import { onMount } from "svelte";
    import { getUserPokemon, deletePokemon } from "./api/pokemon";
    import type { PokemonResponseDto as PokemonResponse } from "./types/api";
    import TypeBadge from "./TypeBadge.svelte";
    import { TYPE_COLORS } from "../data/typeColors";
    import type { PokemonType } from "../types/pokemon";

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
        if (!confirm("Delete this Pokemon?")) {
            return;
        }

        try {
            await deletePokemon(pokemonId, accessToken);
            await loadPokemon();
        } catch (e: any) {
            alert("Delete failed: " + (e.message || "Unknown error"));
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

    function getMoveColor(type: string | null | undefined): string {
        if (!type) return "#333"; // Default dark gray
        const capitalizedType =
            type.charAt(0).toUpperCase() + type.slice(1).toLowerCase();
        return TYPE_COLORS[capitalizedType as PokemonType] || "#333";
    }

    function getIconUrl(formId: number): string {
        return `/icons/pokemon/${formId}.png`;
    }
</script>

<div class="pokemon-list">
    <div class="header">
        <h2 class="text-2xl font-bold text-white">Your Pokemon</h2>
        <button
            class="px-4 py-2 bg-accents-2 hovered text-white rounded-md transition-colors"
            on:click={() => (window.location.href = "#/pokemon/new")}
        >
            + New Pokemon
        </button>
    </div>

    {#if loading}
        <div class="loading text-accents-5">Loading...</div>
    {:else if error}
        <div class="error text-red-500">{error}</div>
    {:else if pokemon.length === 0}
        <div class="empty text-center py-12">
            <p class="text-accents-5 mb-4">No Pokemon registered yet.</p>
            <button
                class="px-4 py-2 bg-white text-black font-medium rounded-md hover:bg-gray-200 transition-colors"
                on:click={() => (window.location.href = "#/pokemon/new")}
            >
                Register your first Pokemon
            </button>
        </div>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each pokemon as poke (poke.pokemon_id)}
                <div
                    class="pokemon-card bg-black border border-accents-2 rounded-lg overflow-hidden shadow-sm hover:border-accents-5 transition-colors"
                >
                    <div
                        class="card-header p-4 bg-accents-1 border-b border-accents-2 flex justify-between items-center"
                    >
                        <div class="flex items-center gap-3">
                            <img
                                src={getIconUrl(poke.form_id)}
                                alt={poke.fullname}
                                class="w-10 h-10 pixelated"
                            />
                            <div>
                                <h3 class="text-lg font-bold text-white m-0">
                                    {poke.nickname ??
                                        poke.fullname_jp ??
                                        poke.fullname}
                                </h3>
                                {#if poke.nickname}
                                    <span
                                        class="text-xs text-accents-5 block mt-1"
                                        >({poke.fullname_jp ??
                                            poke.fullname})</span
                                    >
                                {/if}
                            </div>
                        </div>
                        <div class="flex gap-1 flex-col items-end">
                            <TypeBadge
                                type={poke.type1 as PokemonType}
                                size="sm"
                            />
                            {#if poke.type2}
                                <TypeBadge
                                    type={poke.type2 as PokemonType}
                                    size="sm"
                                />
                            {/if}
                        </div>
                    </div>

                    <div class="card-body p-4 space-y-3">
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="flex flex-col">
                                <span class="text-accents-5 text-xs"
                                    >Nature</span
                                >
                                <span class="text-white"
                                    >{poke.nature_jp ?? poke.nature}</span
                                >
                            </div>
                            <div class="flex flex-col">
                                <span class="text-accents-5 text-xs"
                                    >Ability</span
                                >
                                <span class="text-white"
                                    >{poke.ability_jp ?? poke.ability}</span
                                >
                            </div>
                            <div class="flex flex-col">
                                <span class="text-accents-5 text-xs">Item</span>
                                <span class="text-white"
                                    >{poke.held_item_jp ??
                                        poke.held_item ??
                                        "-"}</span
                                >
                            </div>
                            <div class="flex flex-col">
                                <span class="text-accents-5 text-xs"
                                    >Tera Type</span
                                >
                                <span class="text-white"
                                    >{poke.terastal_type_jp ??
                                        poke.terastal_type}</span
                                >
                            </div>
                        </div>

                        <div class="pt-3 border-t border-accents-2">
                            <div class="mb-2">
                                <span class="text-accents-5 text-xs block mb-1"
                                    >EVs</span
                                >
                                <span
                                    class="text-xs font-mono text-accents-4 bg-accents-1 px-2 py-1 rounded block w-full truncate"
                                    >{formatStats(poke, "ev")}</span
                                >
                            </div>
                        </div>

                        <div class="pt-2">
                            <span class="text-accents-5 text-xs block mb-2"
                                >Moves</span
                            >
                            <div class="flex flex-wrap gap-2">
                                {#each poke.moves as move, i}
                                    {#if move}
                                        <span
                                            class="px-2 py-1 rounded text-xs text-white font-medium shadow-sm"
                                            style="background-color: {getMoveColor(
                                                poke.moves_types?.[i],
                                            )}"
                                        >
                                            {poke.moves_jp?.[i] ?? move}
                                        </span>
                                    {/if}
                                {/each}
                            </div>
                        </div>
                    </div>

                    <div
                        class="card-actions p-3 bg-accents-1 border-t border-accents-2 flex gap-2 justify-end"
                    >
                        <button
                            class="px-3 py-1.5 text-xs text-white hover:bg-accents-2 rounded transition-colors"
                            on:click={() =>
                                (window.location.href = `#/pokemon/${poke.pokemon_id}`)}
                        >
                            Detail
                        </button>
                        <button
                            class="px-3 py-1.5 text-xs text-red-400 hover:bg-red-900/20 rounded transition-colors"
                            on:click={() => handleDelete(poke.pokemon_id)}
                        >
                            Delete
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .pokemon-list {
        padding: 1rem 0;
    }
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }
</style>
