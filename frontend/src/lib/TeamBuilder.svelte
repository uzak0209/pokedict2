<script lang="ts">
    import { onMount } from "svelte";
    import MasterPokemonSelectModal from "$lib/MasterPokemonSelectModal.svelte";
    import PokemonSelectModal from "$lib/PokemonSelectModal.svelte";
    import {
        suggestTeamComplementsWithReasoning,
        saveMatchupOverride,
        type TeamSuggestionWithReasoningResponse,
        type SuggestedPokemonWithReasoning,
        type MatrixPokemon,
    } from "$lib/api/client";
    import { getUserPokemon } from "$lib/api/pokemon";
    import type { PokemonMasterDto, PokemonResponseDto } from "$lib/types/api";
    import { authStore } from "../stores/auth";
    import Card from "./components/ui/Card.svelte";
    import Button from "./components/ui/Button.svelte";

    // 選択されたチームメンバー (最大4体)
    // 軸(Core)として扱う
    let teamMembers: PokemonMasterDto[] = [];

    let suggestionResult: TeamSuggestionWithReasoningResponse | null = null;
    let isLoading = false;
    let error: string | null = null;

    // リストデータ
    let masterPokemonList: PokemonMasterDto[] = [];

    // モーダル制御
    let showMasterModal = false;
    let showUserModal = false;

    // 初期化
    onMount(async () => {
        try {
            await fetchMasterData();
        } catch (e) {
            console.error(e);
        }
    });

    async function fetchMasterData() {
        const res = await fetch("/api/pokemon/master/top?limit=500");
        if (res.ok) {
            const data = await res.json();
            masterPokemonList = data.pokemon;
        }
    }

    // マスタデータからポケモンを選択
    function handleMasterSelect(event: CustomEvent<PokemonMasterDto>) {
        const pokemon = event.detail;
        if (teamMembers.length < 4) {
            // 重複チェック
            if (!teamMembers.some((m) => m.form_id === pokemon.form_id)) {
                teamMembers = [...teamMembers, pokemon];
                suggestionResult = null; // 結果をリセット
            }
        }
        showMasterModal = false;
    }

    // 自分のBOXからポケモンを選択
    function handleUserSelect(event: CustomEvent<PokemonResponseDto>) {
        const p = event.detail;
        // PokemonResponseDto -> PokemonMasterDto 変換
        const pokemon: PokemonMasterDto = {
            form_id: p.form_id,
            species_id: p.species_id,
            fullname: p.fullname,
            fullname_ja: p.fullname_jp,
            type1: p.type1_jp || "", // 日本語で代用
            type2: p.type2_jp || null,
            hp: p.ev_hp, // ダミーデータとしてEV値使用
            attack: p.ev_attack,
            defense: p.ev_defense,
            sp_attack: p.ev_special_attack,
            sp_defense: p.ev_special_defense,
            speed: p.ev_speed,
            usage: undefined,
            raw_count: undefined,
            is_setup: undefined,
        } as unknown as PokemonMasterDto;

        if (teamMembers.length < 4) {
            // 重複チェック
            if (!teamMembers.some((m) => m.form_id === pokemon.form_id)) {
                teamMembers = [...teamMembers, pokemon];
                suggestionResult = null; // 結果をリセット
            }
        }
        showUserModal = false;
    }

    function removeMember(index: number) {
        teamMembers = teamMembers.filter((_, i) => i !== index);
        suggestionResult = null;
    }

    async function generateSuggestions() {
        if (teamMembers.length === 0) return;

        isLoading = true;
        error = null;
        suggestionResult = null;

        try {
            const formIds = teamMembers.map((m) => m.form_id);
            suggestionResult = await suggestTeamComplementsWithReasoning(
                formIds,
                $authStore.accessToken || undefined,
            );
        } catch (e: any) {
            console.error(e);
            error = e.message || "Failed to generate suggestions";
        } finally {
            isLoading = false;
        }
    }

    function getShortName(
        name: string | null | undefined,
        fallback: string,
    ): string {
        const val = name || fallback;
        if (!val) return "?";
        return val.length > 5 ? val.slice(0, 5) + "…" : val;
    }

    function isDetailedCovered(
        targetId: number,
        coveredThreats: MatrixPokemon[],
    ): boolean {
        return coveredThreats.some((t) => t.form_id === targetId);
    }

    // マッチアップ修正
    async function toggleMatchup(
        suggestion: SuggestedPokemon,
        threat: MatrixPokemon,
        currentStatus: boolean,
    ) {
        if (!$authStore.isAuthenticated) {
            alert("Login required to save overrides.");
            return;
        }

        const newJudgment = currentStatus ? -1 : 1;

        try {
            await saveMatchupOverride(
                $authStore.accessToken!,
                suggestion.form_id,
                threat.form_id,
                newJudgment,
            );

            // 再生成
            await generateSuggestions();
        } catch (e) {
            alert("Failed to save override.");
            console.error(e);
        }
    }
</script>

<div class="mt-6 space-y-8">
    <div class="space-y-2">
        <h1 class="text-3xl font-bold text-white">
            Team Builder <span
                class="text-sm font-normal text-accents-5 bg-accents-2 px-2 py-0.5 rounded ml-2"
                >Beta</span
            >
        </h1>
        <p class="text-accents-5 max-w-2xl">
            Select up to 4 core Pokemon to analyze coverage against Top 30
            threats and find the best complements.
        </p>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-1 gap-8">
        <!-- Core Selection -->
        <Card>
            <h2 class="text-xl font-bold text-white mb-6">Core Members</h2>

            <div class="flex flex-wrap gap-4 mb-8 items-center">
                {#each teamMembers as member, i}
                    <div
                        class="flex items-center gap-2 bg-accents-1 border border-accents-2 pl-2 pr-4 py-1.5 rounded-full text-white"
                    >
                        <img
                            src={`/icons/pokemon/${member.form_id}.png`}
                            alt={member.fullname}
                            class="w-8 h-8 pixelated"
                        />
                        <span class="font-medium"
                            >{member.fullname_ja || member.fullname}</span
                        >
                        <button
                            class="text-accents-4 hover:text-red-500 transition-colors ml-2"
                            onclick={() => removeMember(i)}>✕</button
                        >
                    </div>
                {/each}

                {#if teamMembers.length < 4}
                    <div class="flex gap-2">
                        <Button
                            variant="secondary"
                            size="sm"
                            onclick={() => (showMasterModal = true)}
                        >
                            + From Master
                        </Button>
                        {#if $authStore.isAuthenticated}
                            <Button
                                variant="ghost"
                                size="sm"
                                onclick={() => (showUserModal = true)}
                            >
                                + From Box
                            </Button>
                        {/if}
                    </div>
                {/if}
            </div>

            <div class="flex flex-col sm:flex-row gap-4 items-center">
                <Button
                    variant="primary"
                    disabled={teamMembers.length === 0 || isLoading}
                    onclick={generateSuggestions}
                    class="w-full sm:w-auto min-w-[200px]"
                >
                    {isLoading ? "Analyzing..." : "Analyze Coverage"}
                </Button>

                {#if error}
                    <div class="text-red-500 text-sm">{error}</div>
                {/if}
            </div>
        </Card>

        <!-- Suggestions Section -->
        {#if suggestionResult}
            <div class="space-y-4">
                <div>
                    <h2 class="text-xl font-bold text-white">
                        Coverage Analysis
                    </h2>
                    <p class="text-accents-5 text-sm mt-1">
                        ○ = Advantageous, - = Disadvantageous/Neutral. Click
                        cell to toggle manual override.
                    </p>
                </div>

                <div
                    class="overflow-x-auto rounded-xl border border-accents-2 bg-black/50 backdrop-blur-sm"
                >
                    <table class="w-full border-collapse text-sm">
                        <thead>
                            <tr>
                                <th
                                    class="sticky left-0 z-20 bg-accents-1/90 backdrop-blur p-2 text-left min-w-[150px] border-b border-accents-2 text-accents-5 font-medium"
                                    >Rank / Candidate</th
                                >
                                <th
                                    class="sticky left-[150px] z-20 bg-accents-1/90 backdrop-blur p-2 text-center min-w-[60px] border-b border-accents-2 border-l border-r text-accents-5 font-medium"
                                    >Score</th
                                >
                                <!-- Threats -->
                                {#each suggestionResult.all_threats as threat}
                                    <th
                                        class="p-2 min-w-[40px] border-b border-accents-2 text-accents-5 font-normal h-[140px] align-bottom relative group hover:bg-accents-1 transition-colors"
                                        title={threat.name_ja || threat.name}
                                    >
                                        <div
                                            class="absolute bottom-2 left-1/2 -translate-x-1/2 w-4 flex flex-col items-center gap-1"
                                        >
                                            <span
                                                class="writing-vertical text-xs whitespace-nowrap tracking-wider"
                                                >{getShortName(
                                                    threat.name_ja,
                                                    threat.name,
                                                )}</span
                                            >
                                            {#if threat.is_setup}
                                                <span
                                                    class="text-[10px]"
                                                    title="Setup">🛠️</span
                                                >
                                            {/if}
                                        </div>
                                    </th>
                                {/each}
                            </tr>
                        </thead>
                        <tbody>
                            <!-- Axis Rows -->
                            {#each suggestionResult.axis_pokemon as axis}
                                <tr
                                    class="hover:bg-accents-1/30 transition-colors"
                                >
                                    <td
                                        class="sticky left-0 z-10 bg-black/90 p-3 border-b border-accents-2 border-r text-white flex items-center gap-2"
                                    >
                                        <span
                                            class="bg-blue-900/50 text-blue-200 text-[10px] px-1.5 py-0.5 rounded uppercase font-bold tracking-wider border border-blue-800"
                                            >Core</span
                                        >
                                        <span class="font-medium truncate"
                                            >{getShortName(
                                                axis.name_ja,
                                                axis.name,
                                            )}</span
                                        >
                                        {#if axis.is_setup}
                                            <span title="Setup">🛠️</span>
                                        {/if}
                                    </td>
                                    <td
                                        class="sticky left-[150px] z-10 bg-black/90 p-2 text-center border-b border-accents-2 border-r text-green-400 font-mono font-bold"
                                    >
                                        {axis.score}
                                    </td>
                                    <!-- Threat Cells -->
                                    {#each suggestionResult.all_threats as threat}
                                        {@const covered = isDetailedCovered(
                                            threat.form_id,
                                            axis.covered_threats,
                                        )}
                                        <td
                                            class="p-0 border-b border-accents-2 text-center cursor-pointer transition-colors border-r border-accents-1/50
                                            {covered
                                                ? 'bg-green-900/20 hover:bg-green-900/40 text-green-400'
                                                : 'text-accents-6 hover:bg-accents-1/50'}"
                                            onclick={() =>
                                                toggleMatchup(
                                                    axis,
                                                    threat,
                                                    covered,
                                                )}
                                            title="Click to toggle"
                                        >
                                            {#if covered}
                                                <span class="font-bold">○</span>
                                            {:else}
                                                <span>-</span>
                                            {/if}
                                        </td>
                                    {/each}
                                </tr>
                            {/each}

                            <!-- Suggestion Rows -->
                            {#each suggestionResult.suggestions as suggestion, i}
                                <tr
                                    class="hover:bg-accents-1/30 transition-colors"
                                >
                                    <td
                                        class="sticky left-0 z-10 bg-black/90 p-3 border-b border-accents-2 border-r text-white group"
                                    >
                                        <div class="flex items-center gap-2 mb-1">
                                            <span
                                                class="bg-purple-900/30 text-purple-300 text-[10px] w-5 h-5 flex items-center justify-center rounded-full font-bold border border-purple-800 group-hover:bg-purple-800 transition-colors"
                                                >{i + 1}</span
                                            >
                                            <span class="font-medium truncate"
                                                >{getShortName(
                                                    suggestion.name_ja,
                                                    suggestion.name,
                                                )}</span
                                            >
                                            {#if suggestion.is_setup}
                                                <span title="Setup">🛠️</span>
                                            {/if}
                                        </div>
                                        {#if suggestion.reasoning}
                                            <div class="text-xs text-accents-5 mt-1 pl-7 italic border-l-2 border-purple-800/50">
                                                {suggestion.reasoning}
                                            </div>
                                        {/if}
                                    </td>
                                    <td
                                        class="sticky left-[150px] z-10 bg-black/90 p-2 text-center border-b border-accents-2 border-r text-green-400 font-mono font-bold"
                                    >
                                        {suggestion.score}
                                    </td>
                                    <!-- Threat Cells -->
                                    {#each suggestionResult.all_threats as threat}
                                        {@const covered = isDetailedCovered(
                                            threat.form_id,
                                            suggestion.covered_threats,
                                        )}
                                        <td
                                            class="p-0 border-b border-accents-2 text-center cursor-pointer transition-colors border-r border-accents-1/50
                                            {covered
                                                ? 'bg-green-900/20 hover:bg-green-900/40 text-green-400'
                                                : 'text-accents-6 hover:bg-accents-1/50'}"
                                            onclick={() =>
                                                toggleMatchup(
                                                    suggestion,
                                                    threat,
                                                    covered,
                                                )}
                                            title="Click to toggle"
                                        >
                                            {#if covered}
                                                <span class="font-bold">○</span>
                                            {:else}
                                                <span>-</span>
                                            {/if}
                                        </td>
                                    {/each}
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            </div>
        {/if}
    </div>
</div>

<!-- Keep original modals for now, or assume they are styled elsewhere or accepted as is -->
{#if showMasterModal}
    <MasterPokemonSelectModal
        pokemonList={masterPokemonList}
        title="Select from Master Data"
        on:select={handleMasterSelect}
        on:close={() => (showMasterModal = false)}
    />
{/if}

{#if showUserModal}
    <PokemonSelectModal
        on:select={handleUserSelect}
        on:close={() => (showUserModal = false)}
    />
{/if}

<style>
    /* Custom vertical writing mode utility */
    .writing-vertical {
        writing-mode: vertical-rl;
        text-orientation: mixed;
        transform: rotate(180deg);
    }
</style>
