<script lang="ts">
    import { onMount } from "svelte";
    import { getTopPokemonMaster } from "./api/pokemonMaster";
    import { getMatchups } from "./api/pokemon";
    import type { PokemonMasterDto, MatchupsDto } from "./types/api";
    import Card from "./components/ui/Card.svelte";

    let topPokemon: PokemonMasterDto[] = [];
    let loading = true;
    let selectedPokemon: PokemonMasterDto | null = null;
    let matchups: MatchupsDto | null = null;
    let loadingMatchups = false;

    onMount(async () => {
        try {
            const response = await getTopPokemonMaster(30);
            topPokemon = response.pokemon;
        } catch (e) {
            console.error("Failed to load top pokemon:", e);
        } finally {
            loading = false;
        }
    });

    async function selectPokemon(pokemon: PokemonMasterDto) {
        selectedPokemon = pokemon;
        loadingMatchups = true;
        matchups = null;
        try {
            matchups = await getMatchups(pokemon.form_id);
        } catch (e) {
            console.error("Failed to load matchups:", e);
        } finally {
            loadingMatchups = false;
        }
    }

    function formatPercent(p: number): string {
        return (p * 100).toFixed(1) + "%";
    }
</script>

<div class="mt-6 space-y-6">
    <div class="space-y-2">
        <h2 class="text-3xl font-bold text-white">Matchup Analysis</h2>
        <p class="text-accents-5">
            Usage rates and matchup data for top Pokemon.
        </p>
    </div>

    <div
        class="grid grid-cols-1 md:grid-cols-3 gap-6 h-[calc(100vh-200px)] min-h-[600px]"
    >
        <!-- Left: Ranking List -->
        <Card class="md:col-span-1 flex flex-col p-4 h-full overflow-hidden">
            <h3 class="text-lg font-bold text-white mb-4">Top 30 Usage</h3>
            {#if loading}
                <div class="text-center py-8 text-accents-5">Loading...</div>
            {:else}
                <div
                    class="overflow-y-auto flex-1 pr-2 space-y-1 custom-scrollbar"
                >
                    {#each topPokemon as pokemon, i}
                        <button
                            class="w-full flex items-center p-3 rounded-lg text-left transition-colors duration-200
                            {selectedPokemon?.form_id === pokemon.form_id
                                ? 'bg-accents-2 border-l-2 border-white'
                                : 'hover:bg-accents-1 border-l-2 border-transparent'}"
                            onclick={() => selectPokemon(pokemon)}
                        >
                            <span class="w-8 font-mono font-bold text-accents-5"
                                >#{i + 1}</span
                            >
                            <div class="flex-1 min-w-0">
                                <div class="font-medium text-white truncate">
                                    {pokemon.fullname_ja || pokemon.fullname}
                                </div>
                            </div>
                            <span class="text-sm font-mono text-accents-4">
                                {(pokemon as any).usage_rate?.toFixed(1) ||
                                    "0"}%
                            </span>
                        </button>
                    {/each}
                </div>
            {/if}
        </Card>

        <!-- Right: Detail -->
        <Card
            class="md:col-span-2 flex flex-col p-6 h-full overflow-hidden text-center justify-center items-center"
        >
            {#if !selectedPokemon}
                <div class="text-accents-5">
                    Select a Pokemon from the list to view matchups
                </div>
            {:else}
                <div class="flex flex-col h-full w-full">
                    <h3 class="text-2xl font-bold text-white mb-6 text-left">
                        {selectedPokemon.fullname_ja ||
                            selectedPokemon.fullname}
                    </h3>

                    {#if loadingMatchups}
                        <div
                            class="flex-1 flex items-center justify-center text-accents-5"
                        >
                            Loading matchups...
                        </div>
                    {:else if matchups}
                        <div
                            class="grid grid-cols-1 lg:grid-cols-2 gap-6 flex-1 overflow-y-auto"
                        >
                            <!-- Favorable -->
                            <div class="flex flex-col">
                                <div
                                    class="bg-green-900/20 text-green-400 p-3 rounded-t-lg font-bold border border-green-900/50"
                                >
                                    Advantage (Win Rate &gt; 50%)
                                </div>
                                <div
                                    class="border border-accents-2 border-t-0 rounded-b-lg flex-1 overflow-hidden bg-black/30"
                                >
                                    {#if matchups.favorable.length === 0}
                                        <div
                                            class="p-8 text-accents-5 text-center"
                                        >
                                            No data
                                        </div>
                                    {:else}
                                        <div
                                            class="overflow-y-auto max-h-[400px]"
                                        >
                                            <table
                                                class="w-full text-sm text-left"
                                            >
                                                <thead
                                                    class="bg-accents-1 text-accents-5 sticky top-0"
                                                >
                                                    <tr>
                                                        <th
                                                            class="p-3 font-normal"
                                                            >Opponent</th
                                                        >
                                                        <th
                                                            class="p-3 font-normal text-right"
                                                            >Win Rate</th
                                                        >
                                                        <th
                                                            class="p-3 font-normal text-right"
                                                            >Games</th
                                                        >
                                                    </tr>
                                                </thead>
                                                <tbody
                                                    class="divide-y divide-accents-2"
                                                >
                                                    {#each matchups.favorable as m}
                                                        <tr
                                                            class="hover:bg-accents-1/50"
                                                        >
                                                            <td
                                                                class="p-3 text-white"
                                                                >{m.opponent_name_ja ||
                                                                    m.opponent_name}</td
                                                            >
                                                            <td
                                                                class="p-3 text-right text-green-400 font-bold"
                                                                >{formatPercent(
                                                                    m.p,
                                                                )}</td
                                                            >
                                                            <td
                                                                class="p-3 text-right text-accents-5"
                                                                >{Math.round(
                                                                    m.n,
                                                                )}</td
                                                            >
                                                        </tr>
                                                    {/each}
                                                </tbody>
                                            </table>
                                        </div>
                                    {/if}
                                </div>
                            </div>

                            <!-- Unfavorable -->
                            <div class="flex flex-col">
                                <div
                                    class="bg-red-900/20 text-red-400 p-3 rounded-t-lg font-bold border border-red-900/50"
                                >
                                    Disadvantage (Win Rate &lt; 50%)
                                </div>
                                <div
                                    class="border border-accents-2 border-t-0 rounded-b-lg flex-1 overflow-hidden bg-black/30"
                                >
                                    {#if matchups.unfavorable.length === 0}
                                        <div
                                            class="p-8 text-accents-5 text-center"
                                        >
                                            No data
                                        </div>
                                    {:else}
                                        <div
                                            class="overflow-y-auto max-h-[400px]"
                                        >
                                            <table
                                                class="w-full text-sm text-left"
                                            >
                                                <thead
                                                    class="bg-accents-1 text-accents-5 sticky top-0"
                                                >
                                                    <tr>
                                                        <th
                                                            class="p-3 font-normal"
                                                            >Opponent</th
                                                        >
                                                        <th
                                                            class="p-3 font-normal text-right"
                                                            >Win Rate</th
                                                        >
                                                        <th
                                                            class="p-3 font-normal text-right"
                                                            >Games</th
                                                        >
                                                    </tr>
                                                </thead>
                                                <tbody
                                                    class="divide-y divide-accents-2"
                                                >
                                                    {#each matchups.unfavorable as m}
                                                        <tr
                                                            class="hover:bg-accents-1/50"
                                                        >
                                                            <td
                                                                class="p-3 text-white"
                                                                >{m.opponent_name_ja ||
                                                                    m.opponent_name}</td
                                                            >
                                                            <td
                                                                class="p-3 text-right text-red-400 font-bold"
                                                                >{formatPercent(
                                                                    m.p,
                                                                )}</td
                                                            >
                                                            <td
                                                                class="p-3 text-right text-accents-5"
                                                                >{Math.round(
                                                                    m.n,
                                                                )}</td
                                                            >
                                                        </tr>
                                                    {/each}
                                                </tbody>
                                            </table>
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    {:else}
                        <div class="text-accents-5">
                            No matchup data available
                        </div>
                    {/if}
                </div>
            {/if}
        </Card>
    </div>
</div>

<style>
    /* Custom scrollbar for webkit */
    .custom-scrollbar::-webkit-scrollbar {
        width: 6px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background-color: #333;
        border-radius: 3px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background-color: #555;
    }
</style>
