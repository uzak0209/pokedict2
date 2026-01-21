<script lang="ts">
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import type {
        CreatePokemonRequestDto,
        PokemonResponseDto as PokemonResponse,
        PokemonMasterDto,
    } from "./types/api";
    import {
        getAllPokemonMaster,
        getPokemonUsage,
        type PokemonUsageStatsDto,
    } from "./api/pokemonMaster";
    import PokemonAutocomplete from "./PokemonAutocomplete.svelte";

    export let editMode: boolean = false;
    export let initialData: PokemonResponse | null = null;

    const dispatch = createEventDispatcher<{
        submit: CreatePokemonRequestDto;
        cancel: void;
    }>();

    // ポケモンマスタデータ
    let pokemonMasterData: PokemonMasterDto[] = [];
    let loadingMasterData = true;
    let selectedPokemon: PokemonMasterDto | null = null;

    // Usage Stats データ
    let usageStats: PokemonUsageStatsDto | null = null;
    let loadingUsageStats = false;

    // タイプのリスト
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

    // 性格のリスト
    const natures = [
        "Hardy",
        "Lonely",
        "Brave",
        "Adamant",
        "Naughty",
        "Bold",
        "Docile",
        "Relaxed",
        "Impish",
        "Lax",
        "Timid",
        "Hasty",
        "Serious",
        "Jolly",
        "Naive",
        "Modest",
        "Mild",
        "Quiet",
        "Bashful",
        "Rash",
        "Calm",
        "Gentle",
        "Sassy",
        "Careful",
        "Quirky",
    ];

    // フォームの初期値
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

    // 選択されたポケモンが変わったら、名前を更新 + Usage Stats を取得
    $: if (selectedPokemon) {
        pokemonName = selectedPokemon.fullname;
        pokemonNameJp = selectedPokemon.fullname_ja || selectedPokemon.fullname;
        loadUsageStats(selectedPokemon.form_id);
    }

    async function loadUsageStats(formId: number) {
        loadingUsageStats = true;
        usageStats = null;
        console.log(`[PokemonForm] Loading usage stats for form_id: ${formId}`);
        try {
            usageStats = await getPokemonUsage(formId);
            console.log(`[PokemonForm] Usage stats loaded:`, usageStats);
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

            // 編集モードで既存データがある場合、選択済みポケモンを設定
            if (editMode && initialData) {
                selectedPokemon =
                    pokemonMasterData.find(
                        (p) =>
                            p.fullname === initialData.fullname ||
                            p.fullname_ja === initialData.fullname_jp,
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
            alert("ポケモン名、特性、技（最低1つ）は必須です");
            return;
        }

        if (!evValid) {
            alert("努力値が不正です（合計508以下、各252以下）");
            return;
        }

        // user_id はバックエンドがJWTから取得するため、リクエストには含めない
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

<div class="pokemon-form">
    <h2>{editMode ? "ポケモン編集" : "ポケモン登録"}</h2>

    <form on:submit|preventDefault={handleSubmit}>
        <!-- 基本情報 -->
        <section class="form-section">
            <h3>基本情報</h3>

            <div class="form-group">
                <label for="pokemon-name">ポケモン名 *</label>
                {#if loadingMasterData}
                    <input type="text" placeholder="読み込み中..." disabled />
                {:else}
                    <PokemonAutocomplete
                        bind:value={selectedPokemon}
                        pokemonList={pokemonMasterData}
                        placeholder="ポケモンを検索（例: ピカチュウ）"
                        required={true}
                    />
                {/if}
            </div>

            <div class="form-group">
                <label for="nickname">ニックネーム</label>
                <input
                    id="nickname"
                    type="text"
                    bind:value={nickname}
                    maxlength="100"
                />
            </div>

            <div class="form-group">
                <label for="terastal-type">テラスタイプ *</label>
                <input
                    id="terastal-type"
                    type="text"
                    bind:value={terastalType}
                    list="tera-types-list"
                    required
                />
                <datalist id="tera-types-list">
                    {#if usageStats?.tera_types}
                        {#each usageStats.tera_types as tera}
                            <option value={tera.name}
                                >{tera.percentage.toFixed(1)}%</option
                            >
                        {/each}
                    {:else}
                        {#each types as type}
                            <option value={type}>{type}</option>
                        {/each}
                    {/if}
                </datalist>
            </div>

            <div class="form-group">
                <label for="nature">性格 *</label>
                <input
                    id="nature"
                    type="text"
                    bind:value={nature}
                    list="natures-list"
                    required
                />
                <datalist id="natures-list">
                    {#if usageStats?.natures}
                        {#each usageStats.natures as nat}
                            <option value={nat.name}
                                >{nat.percentage.toFixed(1)}%</option
                            >
                        {/each}
                    {:else}
                        {#each natures as nat}
                            <option value={nat}>{nat}</option>
                        {/each}
                    {/if}
                </datalist>
            </div>

            <div class="form-group">
                <label for="ability">特性 *</label>
                <input
                    id="ability"
                    type="text"
                    bind:value={ability}
                    list="abilities-list"
                    required
                    maxlength="100"
                />
                {#if usageStats?.abilities}
                    <datalist id="abilities-list">
                        {#each usageStats.abilities as ab}
                            <option value={ab.name}
                                >{ab.percentage.toFixed(1)}%</option
                            >
                        {/each}
                    </datalist>
                {/if}
            </div>

            <div class="form-group">
                <label for="held-item">持ち物</label>
                <input
                    id="held-item"
                    type="text"
                    bind:value={heldItem}
                    list="items-list"
                    maxlength="100"
                />
                {#if usageStats?.items}
                    <datalist id="items-list">
                        {#each usageStats.items as item}
                            <option value={item.name}
                                >{item.percentage.toFixed(1)}%</option
                            >
                        {/each}
                    </datalist>
                {/if}
            </div>
        </section>

        <!-- 技 -->
        <section class="form-section">
            <h3>技</h3>
            {#if usageStats?.moves}
                <datalist id="moves-list">
                    {#each usageStats.moves as move}
                        <option value={move.name}
                            >{move.percentage.toFixed(1)}%</option
                        >
                    {/each}
                </datalist>
            {/if}
            {#each moves as move, i}
                <div class="form-group">
                    <label for="move-{i}">技 {i + 1} {i === 0 ? "*" : ""}</label
                    >
                    <input
                        id="move-{i}"
                        type="text"
                        bind:value={moves[i]}
                        list="moves-list"
                        required={i === 0}
                        maxlength="50"
                    />
                </div>
            {/each}
        </section>

        <!-- 努力値 -->
        <section class="form-section">
            <h3>努力値 (合計: {totalEv}/508) {!evValid ? "❌" : "✅"}</h3>
            <div class="stats-grid">
                <div class="form-group">
                    <label for="ev-hp">HP</label>
                    <input
                        id="ev-hp"
                        type="number"
                        bind:value={evHp}
                        min="0"
                        max="252"
                    />
                </div>
                <div class="form-group">
                    <label for="ev-attack">攻撃</label>
                    <input
                        id="ev-attack"
                        type="number"
                        bind:value={evAttack}
                        min="0"
                        max="252"
                    />
                </div>
                <div class="form-group">
                    <label for="ev-defense">防御</label>
                    <input
                        id="ev-defense"
                        type="number"
                        bind:value={evDefense}
                        min="0"
                        max="252"
                    />
                </div>
                <div class="form-group">
                    <label for="ev-sp-attack">特攻</label>
                    <input
                        id="ev-sp-attack"
                        type="number"
                        bind:value={evSpAttack}
                        min="0"
                        max="252"
                    />
                </div>
                <div class="form-group">
                    <label for="ev-sp-defense">特防</label>
                    <input
                        id="ev-sp-defense"
                        type="number"
                        bind:value={evSpDefense}
                        min="0"
                        max="252"
                    />
                </div>
                <div class="form-group">
                    <label for="ev-speed">素早さ</label>
                    <input
                        id="ev-speed"
                        type="number"
                        bind:value={evSpeed}
                        min="0"
                        max="252"
                    />
                </div>
            </div>
        </section>

        <!-- 個体値 -->
        <section class="form-section">
            <h3>個体値</h3>
            <div class="stats-grid">
                <div class="form-group">
                    <label for="iv-hp">HP</label>
                    <input
                        id="iv-hp"
                        type="number"
                        bind:value={ivHp}
                        min="0"
                        max="31"
                    />
                </div>
                <div class="form-group">
                    <label for="iv-attack">攻撃</label>
                    <input
                        id="iv-attack"
                        type="number"
                        bind:value={ivAttack}
                        min="0"
                        max="31"
                    />
                </div>
                <div class="form-group">
                    <label for="iv-defense">防御</label>
                    <input
                        id="iv-defense"
                        type="number"
                        bind:value={ivDefense}
                        min="0"
                        max="31"
                    />
                </div>
                <div class="form-group">
                    <label for="iv-sp-attack">特攻</label>
                    <input
                        id="iv-sp-attack"
                        type="number"
                        bind:value={ivSpAttack}
                        min="0"
                        max="31"
                    />
                </div>
                <div class="form-group">
                    <label for="iv-sp-defense">特防</label>
                    <input
                        id="iv-sp-defense"
                        type="number"
                        bind:value={ivSpDefense}
                        min="0"
                        max="31"
                    />
                </div>
                <div class="form-group">
                    <label for="iv-speed">素早さ</label>
                    <input
                        id="iv-speed"
                        type="number"
                        bind:value={ivSpeed}
                        min="0"
                        max="31"
                    />
                </div>
            </div>
        </section>

        <!-- ボタン -->
        <div class="form-actions">
            <button
                type="button"
                class="btn-secondary"
                on:click={() => dispatch("cancel")}
            >
                キャンセル
            </button>
            <button type="submit" class="btn-primary" disabled={!evValid}>
                {editMode ? "更新" : "登録"}
            </button>
        </div>
    </form>
</div>

<style>
    .pokemon-form {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
    }

    h2 {
        margin-bottom: 2rem;
        color: #333;
    }

    .form-section {
        margin-bottom: 2rem;
        padding: 1.5rem;
        background: #f9f9f9;
        border-radius: 8px;
    }

    .form-section h3 {
        margin-top: 0;
        margin-bottom: 1rem;
        color: #555;
    }

    .form-group {
        margin-bottom: 1rem;
    }

    .form-group label {
        display: block;
        margin-bottom: 0.5rem;
        font-weight: 500;
        color: #666;
    }

    .form-group input,
    .form-group select {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 1rem;
    }

    .form-group input:focus,
    .form-group select:focus {
        outline: none;
        border-color: #4caf50;
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 1rem;
    }

    .form-actions {
        display: flex;
        gap: 1rem;
        justify-content: flex-end;
        margin-top: 2rem;
    }

    .btn-primary,
    .btn-secondary {
        padding: 0.75rem 2rem;
        border: none;
        border-radius: 4px;
        font-size: 1rem;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .btn-primary {
        background-color: #4caf50;
        color: white;
    }

    .btn-primary:hover:not(:disabled) {
        background-color: #45a049;
    }

    .btn-primary:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    .btn-secondary {
        background-color: #f0f0f0;
        color: #333;
    }

    .btn-secondary:hover {
        background-color: #e0e0e0;
    }
</style>
