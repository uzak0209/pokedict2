<script lang="ts">
    import { onMount } from "svelte";
    import { getPokemon, updatePokemon } from "./api/pokemon";
    import PokemonForm from "./PokemonForm.svelte";
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
            error = e.message || "ポケモンの読み込みに失敗しました";
        } finally {
            loading = false;
        }
    }

    async function handleUpdate(event: CustomEvent<UpdatePokemonRequestDto>) {
        if (!pokemon) return;

        const request = event.detail;
        try {
            pokemon = await updatePokemon(
                pokemonId,
                request,
                accessToken,
            );
            editMode = false;
            alert("更新しました");
        } catch (e: any) {
            alert("更新に失敗しました: " + (e.message || "不明なエラー"));
        }
    }

    function formatStats(
        pokemon: PokemonResponse,
        statType: "ev" | "iv",
    ): string {
        if (statType === "ev") {
            return `HP: ${pokemon.ev_hp}, 攻撃: ${pokemon.ev_attack}, 防御: ${pokemon.ev_defense}, 特攻: ${pokemon.ev_special_attack}, 特防: ${pokemon.ev_special_defense}, 素早さ: ${pokemon.ev_speed}`;
        } else {
            return `HP: ${pokemon.iv_hp}, 攻撃: ${pokemon.iv_attack}, 防御: ${pokemon.iv_defense}, 特攻: ${pokemon.iv_special_attack}, 特防: ${pokemon.iv_special_defense}, 素早さ: ${pokemon.iv_speed}`;
        }
    }
</script>

<div class="pokemon-detail">
    {#if loading}
        <div class="loading">読み込み中...</div>
    {:else if error}
        <div class="error">{error}</div>
        <button
            class="btn-primary"
            on:click={() => (window.location.href = "#/pokemon")}
        >
            一覧に戻る
        </button>
    {:else if pokemon && !editMode}
        <div class="detail-view">
            <div class="header">
                <h2>{pokemon.nickname || pokemon.fullname_jp}</h2>
                <div class="header-actions">
                    <button
                        class="btn-secondary"
                        on:click={() => (editMode = true)}>編集</button
                    >
                    <button
                        class="btn-secondary"
                        on:click={() => (window.location.href = "#/pokemon")}
                    >
                        一覧に戻る
                    </button>
                </div>
            </div>

            {#if pokemon.nickname}
                <div class="species-name">種族: {pokemon.fullname_jp}</div>
            {/if}

            <section class="detail-section">
                <h3>基本情報</h3>
                <div class="info-grid">
                    <div class="info-item">
                        <span class="label">性格:</span>
                        <span>{pokemon.nature}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">特性:</span>
                        <span>{pokemon.ability}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">テラスタイプ:</span>
                        <span>{pokemon.terastal_type}</span>
                    </div>
                    {#if pokemon.held_item}
                        <div class="info-item">
                            <span class="label">持ち物:</span>
                            <span>{pokemon.held_item}</span>
                        </div>
                    {/if}
                </div>
            </section>

            <section class="detail-section">
                <h3>技</h3>
                <div class="moves-list">
                    {#each pokemon.moves as move, i}
                        <div class="move-item">
                            <span class="move-number">{i + 1}.</span>
                            <span>{move}</span>
                        </div>
                    {/each}
                </div>
            </section>

            <section class="detail-section">
                <h3>努力値</h3>
                <div class="stats-display">
                    <div class="stat-bar">
                        <span class="stat-label">HP</span>
                        <div class="bar">
                            <div
                                class="bar-fill"
                                style="width: {(pokemon.ev_hp / 252) * 100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.ev_hp}</span>
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">攻撃</span>
                        <div class="bar">
                            <div
                                class="bar-fill"
                                style="width: {(pokemon.ev_attack / 252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.ev_attack}</span>
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">防御</span>
                        <div class="bar">
                            <div
                                class="bar-fill"
                                style="width: {(pokemon.ev_defense / 252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.ev_defense}</span>
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">特攻</span>
                        <div class="bar">
                            <div
                                class="bar-fill"
                                style="width: {(pokemon.ev_special_attack /
                                    252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value"
                            >{pokemon.ev_special_attack}</span
                        >
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">特防</span>
                        <div class="bar">
                            <div
                                class="bar-fill"
                                style="width: {(pokemon.ev_special_defense /
                                    252) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value"
                            >{pokemon.ev_special_defense}</span
                        >
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">素早さ</span>
                        <div class="bar">
                            <div
                                class="bar-fill"
                                style="width: {(pokemon.ev_speed / 252) * 100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.ev_speed}</span>
                    </div>
                </div>
            </section>

            <section class="detail-section">
                <h3>個体値</h3>
                <div class="stats-display">
                    <div class="stat-bar">
                        <span class="stat-label">HP</span>
                        <div class="bar">
                            <div
                                class="bar-fill iv"
                                style="width: {(pokemon.iv_hp / 31) * 100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.iv_hp}</span>
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">攻撃</span>
                        <div class="bar">
                            <div
                                class="bar-fill iv"
                                style="width: {(pokemon.iv_attack / 31) * 100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.iv_attack}</span>
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">防御</span>
                        <div class="bar">
                            <div
                                class="bar-fill iv"
                                style="width: {(pokemon.iv_defense / 31) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.iv_defense}</span>
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">特攻</span>
                        <div class="bar">
                            <div
                                class="bar-fill iv"
                                style="width: {(pokemon.iv_special_attack /
                                    31) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value"
                            >{pokemon.iv_special_attack}</span
                        >
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">特防</span>
                        <div class="bar">
                            <div
                                class="bar-fill iv"
                                style="width: {(pokemon.iv_special_defense /
                                    31) *
                                    100}%"
                            ></div>
                        </div>
                        <span class="stat-value"
                            >{pokemon.iv_special_defense}</span
                        >
                    </div>
                    <div class="stat-bar">
                        <span class="stat-label">素早さ</span>
                        <div class="bar">
                            <div
                                class="bar-fill iv"
                                style="width: {(pokemon.iv_speed / 31) * 100}%"
                            ></div>
                        </div>
                        <span class="stat-value">{pokemon.iv_speed}</span>
                    </div>
                </div>
            </section>
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

<style>
    .pokemon-detail {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
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

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .header h2 {
        margin: 0;
        color: #333;
    }

    .header-actions {
        display: flex;
        gap: 0.5rem;
    }

    .species-name {
        color: #666;
        font-size: 1.1rem;
        margin-bottom: 2rem;
    }

    .detail-section {
        margin-bottom: 2rem;
        padding: 1.5rem;
        background: #f9f9f9;
        border-radius: 8px;
    }

    .detail-section h3 {
        margin-top: 0;
        margin-bottom: 1rem;
        color: #555;
    }

    .info-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
    }

    .info-item {
        display: flex;
        gap: 0.5rem;
    }

    .label {
        font-weight: 600;
        color: #555;
    }

    .moves-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .move-item {
        padding: 0.75rem;
        background: white;
        border-radius: 4px;
        display: flex;
        gap: 0.5rem;
    }

    .move-number {
        font-weight: 600;
        color: #666;
    }

    .stats-display {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .stat-bar {
        display: grid;
        grid-template-columns: 80px 1fr 60px;
        align-items: center;
        gap: 1rem;
    }

    .stat-label {
        font-weight: 600;
        color: #555;
    }

    .bar {
        height: 24px;
        background: #e0e0e0;
        border-radius: 12px;
        overflow: hidden;
    }

    .bar-fill {
        height: 100%;
        background: linear-gradient(90deg, #4caf50, #8bc34a);
        transition: width 0.3s;
    }

    .bar-fill.iv {
        background: linear-gradient(90deg, #2196f3, #64b5f6);
    }

    .stat-value {
        text-align: right;
        font-weight: 600;
        color: #333;
    }

    .btn-primary,
    .btn-secondary {
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
    }

    .btn-secondary:hover {
        background-color: #e0e0e0;
    }
</style>
