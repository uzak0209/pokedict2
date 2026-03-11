<script lang="ts">
    import { onMount } from "svelte";
    import { getPokemon, updatePokemon } from "./api/pokemon";
    import PokemonForm from "./PokemonForm.svelte";
    import Button from "./components/ui/Button.svelte";
    import Card from "./components/ui/Card.svelte";
    import TypeBadge from "./TypeBadge.svelte";
    import { TYPE_COLORS } from "../data/typeColors";
    import type { PokemonType } from "../types/pokemon";
    import type {
        PokemonResponseDto as PokemonResponse,
        UpdatePokemonRequestDto,
    } from "./types/api";

    export let pokemonId: string;
    export let accessToken: string = "";

    let pokemon: PokemonResponse | null = null;
    let loading = true;
    let error: string | null = null;
    let editMode = false;

    onMount(async () => {
        await loadPokemon();
    });

    async function loadPokemon() {
        loading = true;
        error = null;
        try {
            pokemon = await getPokemon(pokemonId, accessToken);
        } catch (e: any) {
            error = e.message || "Failed to load Pokemon";
        } finally {
            loading = false;
        }
    }

    async function handleUpdate(event: CustomEvent<UpdatePokemonRequestDto>) {
        if (!pokemon) return;

        const request = event.detail;
        try {
            pokemon = await updatePokemon(pokemonId, request, accessToken);
            editMode = false;
            alert("Updated successfully");
        } catch (e: any) {
            alert("Update failed: " + (e.message || "Unknown error"));
        }
    }

    function getMoveColor(type: string | null | undefined): string {
        if (!type) return "#333";
        const capitalizedType =
            type.charAt(0).toUpperCase() + type.slice(1).toLowerCase();
        return TYPE_COLORS[capitalizedType as PokemonType] || "#333";
    }
</script>

<div class="max-w-4xl mx-auto py-6">
    {#if loading}
        <div class="text-center py-12 text-accents-5">Loading...</div>
    {:else if error}
        <div class="text-center py-12 text-red-500 mb-4">{error}</div>
        <div class="text-center">
            <Button
                variant="secondary"
                onclick={() => (window.location.href = "#/pokemon")}
            >
                Back to List
            </Button>
        </div>
    {:else if pokemon && !editMode}
        <div class="space-y-6">
            <div
                class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4"
            >
                <div class="flex items-center gap-4">
                    <img
                        src={`/icons/pokemon/${pokemon.form_id}.png`}
                        alt={pokemon.fullname}
                        class="w-16 h-16 pixelated"
                    />
                    <div>
                        <h2 class="text-3xl font-bold text-white">
                            {pokemon.nickname || pokemon.fullname_jp}
                        </h2>
                        {#if pokemon.nickname}
                            <div class="text-accents-5 mt-1">
                                Species: {pokemon.fullname_jp}
                            </div>
                        {/if}
                    </div>
                </div>
                <div class="flex gap-2">
                    <Button
                        variant="secondary"
                        onclick={() => (editMode = true)}>Edit</Button
                    >
                    <Button
                        variant="ghost"
                        onclick={() => (window.location.href = "#/pokemon")}
                        >Back</Button
                    >
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <!-- Basic Info -->
                <Card>
                    <h3 class="text-xl font-bold text-white mb-4">
                        Basic Info
                    </h3>
                    <div class="space-y-3 text-sm">
                        <div
                            class="flex justify-between border-b border-accents-2 pb-2"
                        >
                            <span class="text-accents-5">Type</span>
                            <div class="flex gap-2">
                                <TypeBadge
                                    type={pokemon.type1 as PokemonType}
                                />
                                {#if pokemon.type2}
                                    <TypeBadge
                                        type={pokemon.type2 as PokemonType}
                                    />
                                {:else if pokemon.type2_jp}
                                    <!-- Fallback if type2 (EN) missing but type2_jp exists -->
                                    <span
                                        class="px-3 py-1 rounded-md font-bold text-white shadow-sm bg-gray-500"
                                        >{pokemon.type2_jp}</span
                                    >
                                {/if}
                            </div>
                        </div>
                        <div
                            class="flex justify-between border-b border-accents-2 pb-2"
                        >
                            <span class="text-accents-5">Nature</span>
                            <span class="text-white"
                                >{pokemon.nature_jp ?? pokemon.nature}</span
                            >
                        </div>
                        <div
                            class="flex justify-between border-b border-accents-2 pb-2"
                        >
                            <span class="text-accents-5">Ability</span>
                            <span class="text-white"
                                >{pokemon.ability_jp ?? pokemon.ability}</span
                            >
                        </div>
                        <div
                            class="flex justify-between border-b border-accents-2 pb-2"
                        >
                            <span class="text-accents-5">Tera Type</span>
                            <span class="text-white"
                                >{pokemon.terastal_type_jp ??
                                    pokemon.terastal_type}</span
                            >
                        </div>
                        {#if pokemon.held_item}
                            <div
                                class="flex justify-between border-b border-accents-2 pb-2"
                            >
                                <span class="text-accents-5">Item</span>
                                <span class="text-white"
                                    >{pokemon.held_item_jp ??
                                        pokemon.held_item}</span
                                >
                            </div>
                        {/if}
                    </div>
                </Card>

                <!-- Moves -->
                <Card>
                    <h3 class="text-xl font-bold text-white mb-4">Moves</h3>
                    <div class="text-xs text-accents-5 mb-2">
                        DEBUG: {JSON.stringify(pokemon.moves_types)}
                    </div>
                    <div class="space-y-2">
                        {#each pokemon.moves as move, i}
                            <div
                                class="flex items-center gap-3 p-2 rounded bg-accents-1 border border-accents-2"
                            >
                                <span
                                    class="text-accents-5 font-mono text-xs w-4"
                                    >{i + 1}.</span
                                >
                                <span
                                    class="text-white font-medium px-2 py-0.5 rounded text-sm shadow-sm"
                                    style="background-color: {getMoveColor(
                                        pokemon.moves_types?.[i],
                                    )}">{pokemon.moves_jp?.[i] ?? move}</span
                                >
                            </div>
                        {/each}
                    </div>
                </Card>
            </div>

            <!-- EVs -->
            <Card>
                <h3 class="text-xl font-bold text-white mb-4">
                    EVs (Effort Values)
                </h3>
                <div class="space-y-4">
                    <!-- HP -->
                    <div class="flex items-center gap-4 text-sm">
                        <span class="w-16 font-bold text-accents-5">HP</span>
                        <div
                            class="flex-1 h-2 bg-accents-2 rounded-full overflow-hidden"
                        >
                            <div
                                class="h-full bg-green-500 rounded-full"
                                style="width: {(pokemon.ev_hp / 252) * 100}%"
                            ></div>
                        </div>
                        <span class="w-8 text-right font-mono text-white"
                            >{pokemon.ev_hp}</span
                        >
                    </div>
                    <!-- Attack -->
                    <div class="flex items-center gap-4 text-sm">
                        <span class="w-16 font-bold text-accents-5">Attack</span
                        >
                        <div
                            class="flex-1 h-2 bg-accents-2 rounded-full overflow-hidden"
                        >
                            <div
                                class="h-full bg-green-500 rounded-full"
                                style="width: {(pokemon.ev_attack / 252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="w-8 text-right font-mono text-white"
                            >{pokemon.ev_attack}</span
                        >
                    </div>
                    <!-- Defense -->
                    <div class="flex items-center gap-4 text-sm">
                        <span class="w-16 font-bold text-accents-5"
                            >Defense</span
                        >
                        <div
                            class="flex-1 h-2 bg-accents-2 rounded-full overflow-hidden"
                        >
                            <div
                                class="h-full bg-green-500 rounded-full"
                                style="width: {(pokemon.ev_defense / 252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="w-8 text-right font-mono text-white"
                            >{pokemon.ev_defense}</span
                        >
                    </div>
                    <!-- Sp. Atk -->
                    <div class="flex items-center gap-4 text-sm">
                        <span class="w-16 font-bold text-accents-5"
                            >Sp. Atk</span
                        >
                        <div
                            class="flex-1 h-2 bg-accents-2 rounded-full overflow-hidden"
                        >
                            <div
                                class="h-full bg-green-500 rounded-full"
                                style="width: {(pokemon.ev_special_attack /
                                    252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="w-8 text-right font-mono text-white"
                            >{pokemon.ev_special_attack}</span
                        >
                    </div>
                    <!-- Sp. Def -->
                    <div class="flex items-center gap-4 text-sm">
                        <span class="w-16 font-bold text-accents-5"
                            >Sp. Def</span
                        >
                        <div
                            class="flex-1 h-2 bg-accents-2 rounded-full overflow-hidden"
                        >
                            <div
                                class="h-full bg-green-500 rounded-full"
                                style="width: {(pokemon.ev_special_defense /
                                    252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="w-8 text-right font-mono text-white"
                            >{pokemon.ev_special_defense}</span
                        >
                    </div>
                    <!-- Speed -->
                    <div class="flex items-center gap-4 text-sm">
                        <span class="w-16 font-bold text-accents-5">Speed</span>
                        <div
                            class="flex-1 h-2 bg-accents-2 rounded-full overflow-hidden"
                        >
                            <div
                                class="h-full bg-blue-500 rounded-full"
                                style="width: {(pokemon.ev_speed / 252) * 100}%"
                            ></div>
                        </div>
                        <span class="w-8 text-right font-mono text-white"
                            >{pokemon.ev_speed}</span
                        >
                    </div>
                </div>
            </Card>

            <!-- IVs (Simplified display) -->
            <Card>
                <h3 class="text-xl font-bold text-white mb-4">
                    IVs (Individual Values)
                </h3>
                <div class="grid grid-cols-3 md:grid-cols-6 gap-4 text-center">
                    <div
                        class="bg-accents-1 p-2 rounded border border-accents-2"
                    >
                        <div class="text-accents-5 text-xs mb-1">HP</div>
                        <div class="text-white font-mono font-bold">
                            {pokemon.iv_hp}
                        </div>
                    </div>
                    <div
                        class="bg-accents-1 p-2 rounded border border-accents-2"
                    >
                        <div class="text-accents-5 text-xs mb-1">Atk</div>
                        <div class="text-white font-mono font-bold">
                            {pokemon.iv_attack}
                        </div>
                    </div>
                    <div
                        class="bg-accents-1 p-2 rounded border border-accents-2"
                    >
                        <div class="text-accents-5 text-xs mb-1">Def</div>
                        <div class="text-white font-mono font-bold">
                            {pokemon.iv_defense}
                        </div>
                    </div>
                    <div
                        class="bg-accents-1 p-2 rounded border border-accents-2"
                    >
                        <div class="text-accents-5 text-xs mb-1">SpA</div>
                        <div class="text-white font-mono font-bold">
                            {pokemon.iv_special_attack}
                        </div>
                    </div>
                    <div
                        class="bg-accents-1 p-2 rounded border border-accents-2"
                    >
                        <div class="text-accents-5 text-xs mb-1">SpD</div>
                        <div class="text-white font-mono font-bold">
                            {pokemon.iv_special_defense}
                        </div>
                    </div>
                    <div
                        class="bg-accents-1 p-2 rounded border border-accents-2"
                    >
                        <div class="text-accents-5 text-xs mb-1">Spe</div>
                        <div class="text-white font-mono font-bold">
                            {pokemon.iv_speed}
                        </div>
                    </div>
                </div>
            </Card>
        </div>
    {:else if pokemon && editMode}
        <PokemonForm
            editMode={true}
            initialData={pokemon}
            on:submit={handleUpdate}
            on:cancel={() => (editMode = false)}
        />
    {/if}
</div>
