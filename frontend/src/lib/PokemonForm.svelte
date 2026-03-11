<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { createEventDispatcher as svelteCreateEventDispatcher } from "svelte";
    import type {
        CreatePokemonRequestDto,
        PokemonResponseDto as PokemonResponse,
        PokemonMasterDto,
        LearnableMoveDto,
    } from "./types/api";
    import {
        getAllPokemonMaster,
        getPokemonUsage,
        type PokemonUsageStatsDto,
    } from "./api/pokemonMaster";
    import { getLearnableMoves } from "./api/pokemon";
    import PokemonAutocomplete from "./PokemonAutocomplete.svelte";
    import MasterDataAutocomplete from "./MasterDataAutocomplete.svelte";
    import Button from "./components/ui/Button.svelte";
    import Card from "./components/ui/Card.svelte";

    export let editMode: boolean = false;
    export let initialData: PokemonResponse | null = null;

    const dispatch = createEventDispatcher<{
        submit: CreatePokemonRequestDto;
        cancel: void;
    }>();

    // Pokemon Master Data
    let pokemonMasterData: PokemonMasterDto[] = [];
    let loadingMasterData = true;
    let selectedPokemon: PokemonMasterDto | null = null;

    // Usage Stats Data
    let usageStats: PokemonUsageStatsDto | null = null;
    let loadingUsageStats = false;

    // Learnable Moves List
    let learnableMoves: LearnableMoveDto[] = [];
    let loadingMoves = false;

    // Types List (Not used directly but kept for reference)
    const types = [
        "Normal",
        "Fire",
        "Water",
        "Electric",
        "Grass",
        "Ice",
        "Fighting",
        "Poison",
        "Ground",
        "Flying",
        "Psychic",
        "Bug",
        "Rock",
        "Ghost",
        "Dragon",
        "Dark",
        "Steel",
        "Fairy",
    ];

    // Form Initial Values
    let pokemonName = initialData?.fullname || "";
    let pokemonNameJp = initialData?.fullname_jp || "";
    let nickname = initialData?.nickname || "";
    let terastalType = initialData?.terastal_type || "Normal";
    let nature = initialData?.nature || "Hardy";
    let ability = initialData?.ability || "";
    let heldItem = initialData?.held_item || "";

    let evHp = initialData?.ev_hp || 0;
    let evAttack = initialData?.ev_attack || 0;
    let evDefense = initialData?.ev_defense || 0;
    let evSpAttack = initialData?.ev_special_attack || 0;
    let evSpDefense = initialData?.ev_special_defense || 0;
    let evSpeed = initialData?.ev_speed || 0;

    let ivHp = initialData?.iv_hp ?? 31;
    let ivAttack = initialData?.iv_attack ?? 31;
    let ivDefense = initialData?.iv_defense ?? 31;
    let ivSpAttack = initialData?.iv_special_attack ?? 31;
    let ivSpDefense = initialData?.iv_special_defense ?? 31;
    let ivSpeed = initialData?.iv_speed ?? 31;

    let moves = initialData?.moves || ["", "", "", ""];
    if (moves.length < 4) {
        moves = [...moves, ...Array(4 - moves.length).fill("")];
    }

    $: totalEv =
        evHp + evAttack + evDefense + evSpAttack + evSpDefense + evSpeed;
    $: evValid =
        totalEv <= 508 &&
        evHp <= 252 &&
        evAttack <= 252 &&
        evDefense <= 252 &&
        evSpAttack <= 252 &&
        evSpDefense <= 252 &&
        evSpeed <= 252;

    // Update name and load stats when pokemon is selected
    $: if (selectedPokemon) {
        pokemonName = selectedPokemon.fullname;
        pokemonNameJp = selectedPokemon.fullname_ja || selectedPokemon.fullname;
        loadUsageStats(selectedPokemon.form_id);
        loadLearnableMoves(selectedPokemon.form_id);
    }

    async function loadLearnableMoves(formId: number) {
        loadingMoves = true;
        learnableMoves = [];
        try {
            learnableMoves = await getLearnableMoves(formId);
        } catch (e) {
            console.error("[PokemonForm] Failed to load learnable moves:", e);
        } finally {
            loadingMoves = false;
        }
    }

    async function loadUsageStats(formId: number) {
        loadingUsageStats = true;
        usageStats = null;
        try {
            usageStats = await getPokemonUsage(formId);
        } catch (e) {
            console.error("[PokemonForm] Failed to load usage stats:", e);
        } finally {
            loadingUsageStats = false;
        }
    }

    onMount(async () => {
        try {
            const response = await getAllPokemonMaster();
            pokemonMasterData = response.pokemon;

            if (editMode && initialData) {
                selectedPokemon =
                    pokemonMasterData.find(
                        (p) =>
                            p.fullname === initialData!.fullname ||
                            p.fullname_ja === initialData!.fullname_jp,
                    ) || null;
            }
        } catch (e) {
            console.error("Failed to load pokemon master data:", e);
        } finally {
            loadingMasterData = false;
        }
    });

    function handleSubmit() {
        if (!pokemonName || !ability || moves.filter((m) => m).length === 0) {
            alert("Name, Ability, and at least one Move are required.");
            return;
        }

        if (!evValid) {
            alert("Invalid EVs (Total <= 508, Each <= 252)");
            return;
        }

        const request: CreatePokemonRequestDto = {
            pokemon_name: pokemonName,
            pokemon_name_jp: pokemonNameJp,
            nickname: nickname || undefined,
            terastal_type: terastalType,
            ev_hp: evHp,
            ev_attack: evAttack,
            ev_defense: evDefense,
            ev_special_attack: evSpAttack,
            ev_special_defense: evSpDefense,
            ev_speed: evSpeed,
            iv_hp: ivHp,
            iv_attack: ivAttack,
            iv_defense: ivDefense,
            iv_special_attack: ivSpAttack,
            iv_special_defense: ivSpDefense,
            iv_speed: ivSpeed,
            nature: nature,
            ability: ability,
            held_item: heldItem || undefined,
            moves: moves.filter((m) => m !== ""),
        };

        dispatch("submit", request);
    }
</script>

<div class="max-w-4xl mx-auto py-6">
    <h2 class="text-3xl font-bold text-white mb-6">
        {editMode ? "Edit Pokemon" : "New Pokemon"}
    </h2>

    <form on:submit|preventDefault={handleSubmit} class="space-y-6 relative">
        <!-- Basic Info -->
        <Card class="relative z-20">
            <h3
                class="text-xl font-bold text-white mb-4 border-b border-accents-2 pb-2"
            >
                Basic Info
            </h3>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="space-y-1">
                    <label
                        for="pokemon-name"
                        class="block text-sm font-medium text-accents-5"
                        >Pokemon *</label
                    >
                    {#if loadingMasterData}
                        <input
                            type="text"
                            placeholder="Loading..."
                            disabled
                            class="w-full bg-black/50 border border-accents-2 rounded p-2 text-accents-5"
                        />
                    {:else}
                        <PokemonAutocomplete
                            bind:value={selectedPokemon}
                            pokemonList={pokemonMasterData}
                            placeholder="Search Pokemon..."
                            required={true}
                        />
                    {/if}
                </div>

                <div class="space-y-1">
                    <label
                        for="nickname"
                        class="block text-sm font-medium text-accents-5"
                        >Nickname</label
                    >
                    <input
                        id="nickname"
                        type="text"
                        bind:value={nickname}
                        maxlength="100"
                        class="w-full bg-black border border-accents-2 rounded p-2 text-white focus:border-white outline-none transition-colors"
                    />
                </div>

                <div class="space-y-1">
                    <label
                        for="terastal-type"
                        class="block text-sm font-medium text-accents-5"
                        >Tera Type *</label
                    >
                    <MasterDataAutocomplete
                        options={usageStats?.tera_types || []}
                        bind:value={terastalType}
                        placeholder="Select Type..."
                        required={true}
                        loading={loadingUsageStats}
                    />
                </div>

                <div class="space-y-1">
                    <label
                        for="nature"
                        class="block text-sm font-medium text-accents-5"
                        >Nature *</label
                    >
                    <MasterDataAutocomplete
                        options={usageStats?.natures || []}
                        bind:value={nature}
                        placeholder="Select Nature..."
                        required={true}
                        loading={loadingUsageStats}
                    />
                </div>

                <div class="space-y-1">
                    <label
                        for="ability"
                        class="block text-sm font-medium text-accents-5"
                        >Ability *</label
                    >
                    <MasterDataAutocomplete
                        options={usageStats?.abilities || []}
                        bind:value={ability}
                        placeholder="Select Ability..."
                        required={true}
                        loading={loadingUsageStats}
                    />
                </div>

                <div class="space-y-1">
                    <label
                        for="held-item"
                        class="block text-sm font-medium text-accents-5"
                        >Held Item</label
                    >
                    <MasterDataAutocomplete
                        options={usageStats?.items || []}
                        bind:value={heldItem}
                        placeholder="Select Item..."
                        loading={loadingUsageStats}
                    />
                </div>
            </div>
        </Card>

        <!-- Moves -->
        <Card>
            <h3
                class="text-xl font-bold text-white mb-4 border-b border-accents-2 pb-2"
            >
                Moves
            </h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                {#each moves as move, i}
                    <div class="space-y-1">
                        <label
                            for="move-{i}"
                            class="block text-sm font-medium text-accents-5"
                            >Move {i + 1} {i === 0 ? "*" : ""}</label
                        >
                        <MasterDataAutocomplete
                            options={learnableMoves.length > 0
                                ? learnableMoves.map((m) => ({
                                      name: m.name,
                                      name_ja: m.name_ja || null,
                                      type: m.type || null,
                                      percentage: m.usage_rate || 0,
                                  }))
                                : usageStats?.moves || []}
                            bind:value={moves[i]}
                            placeholder="Select Move..."
                            required={i === 0}
                            loading={loadingUsageStats || loadingMoves}
                        />
                    </div>
                {/each}
            </div>
        </Card>

        <!-- EVs -->
        <Card>
            <div
                class="flex justify-between items-center mb-4 border-b border-accents-2 pb-2"
            >
                <h3 class="text-xl font-bold text-white">
                    EVs (Effort Values)
                </h3>
                <span
                    class="text-sm font-mono {evValid
                        ? 'text-green-400'
                        : 'text-red-400'}">Total: {totalEv}/508</span
                >
            </div>

            <div class="grid grid-cols-3 md:grid-cols-6 gap-4">
                {#each [{ label: "HP", bind: evHp, id: "ev-hp" }, { label: "Attack", bind: evAttack, id: "ev-attack" }, { label: "Defense", bind: evDefense, id: "ev-defense" }, { label: "Sp. Atk", bind: evSpAttack, id: "ev-sp-attack" }, { label: "Sp. Def", bind: evSpDefense, id: "ev-sp-defense" }, { label: "Speed", bind: evSpeed, id: "ev-speed" }] as stat}
                    <div class="space-y-1">
                        <label
                            for={stat.id}
                            class="block text-xs font-medium text-accents-5 text-center"
                            >{stat.label}</label
                        >
                        <input
                            id={stat.id}
                            type="number"
                            bind:value={stat.bind}
                            min="0"
                            max="252"
                            class="w-full bg-black border border-accents-2 rounded p-1 text-center text-white focus:border-white outline-none"
                            on:input={(e) => {
                                // @ts-ignore
                                stat.bind =
                                    parseInt(e.currentTarget.value) || 0;
                                // Force update Svelte reactivity if needed
                                if (stat.label === "HP") evHp = stat.bind;
                                else if (stat.label === "Attack")
                                    evAttack = stat.bind;
                                else if (stat.label === "Defense")
                                    evDefense = stat.bind;
                                else if (stat.label === "Sp. Atk")
                                    evSpAttack = stat.bind;
                                else if (stat.label === "Sp. Def")
                                    evSpDefense = stat.bind;
                                else if (stat.label === "Speed")
                                    evSpeed = stat.bind;
                            }}
                        />
                    </div>
                {/each}
            </div>
        </Card>

        <!-- IVs -->
        <Card>
            <h3
                class="text-xl font-bold text-white mb-4 border-b border-accents-2 pb-2"
            >
                IVs (Individual Values)
            </h3>
            <div class="grid grid-cols-3 md:grid-cols-6 gap-4">
                {#each [{ label: "HP", bind: ivHp, id: "iv-hp" }, { label: "Attack", bind: ivAttack, id: "iv-attack" }, { label: "Defense", bind: ivDefense, id: "iv-defense" }, { label: "Sp. Atk", bind: ivSpAttack, id: "iv-sp-attack" }, { label: "Sp. Def", bind: ivSpDefense, id: "iv-sp-defense" }, { label: "Speed", bind: ivSpeed, id: "iv-speed" }] as stat}
                    <div class="space-y-1">
                        <label
                            for={stat.id}
                            class="block text-xs font-medium text-accents-5 text-center"
                            >{stat.label}</label
                        >
                        <input
                            id={stat.id}
                            type="number"
                            bind:value={stat.bind}
                            min="0"
                            max="31"
                            class="w-full bg-accents-1 border border-accents-2 rounded p-1 text-center text-accents-6 focus:border-white outline-none"
                            on:input={(e) => {
                                // @ts-ignore
                                stat.bind =
                                    parseInt(e.currentTarget.value) || 0;
                                // Force update IVs
                                if (stat.label === "HP") ivHp = stat.bind;
                                else if (stat.label === "Attack")
                                    ivAttack = stat.bind;
                                else if (stat.label === "Defense")
                                    ivDefense = stat.bind;
                                else if (stat.label === "Sp. Atk")
                                    ivSpAttack = stat.bind;
                                else if (stat.label === "Sp. Def")
                                    ivSpDefense = stat.bind;
                                else if (stat.label === "Speed")
                                    ivSpeed = stat.bind;
                            }}
                        />
                    </div>
                {/each}
            </div>
        </Card>

        <!-- Actions -->
        <div class="flex justify-end gap-3 pt-4">
            <Button variant="ghost" onclick={() => dispatch("cancel")}>
                Cancel
            </Button>
            <Button variant="primary" type="submit" disabled={!evValid}>
                {editMode ? "Update Pokemon" : "Save Pokemon"}
            </Button>
        </div>
    </form>
</div>
