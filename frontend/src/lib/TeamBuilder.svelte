<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { authStore } from "../stores/auth";
  import { getTeam, createTeam, updateTeam } from "./api/teams";
  import { getPokemonUsage } from "./api/pokemonMaster";
  import PokemonSelectModal from "./PokemonSelectModal.svelte";
  import { getAllPokemonMaster } from "./api/pokemonMaster";
  import type { PokemonUsageStatsDto } from "./api/pokemonMaster";
  import type {
    CreateTeamRequestDto,
    UpdateTeamRequestDto,
    PokemonDataDto,
    PokemonResponseDto,
    PokemonMasterDto,
  } from "./types/api";

  export let teamId: string | null = null;

  // PokemonDataDtoを拡張してform_idを保持できるようにする
  interface TeamMemberData extends PokemonDataDto {
    form_id?: number;
  }

  const dispatch = createEventDispatcher<{
    save: void;
    cancel: void;
  }>();

  let teamName = "";
  let teamMembers: (TeamMemberData | null)[] = Array(6).fill(null);
  let loading = false;
  let showModal = false;
  let currentSlotIndex: number | null = null;
  let accessToken = ""; // subscribe via authStore
  let pokemonMasterData: PokemonMasterDto[] = [];

  // 使用率データのキャッシュ: form_id -> UsageStats
  let usageStatsCache: Record<number, PokemonUsageStatsDto> = {};

  authStore.subscribe((state) => {
    accessToken = state.accessToken || "";
  });

  onMount(async () => {
    // マスタデータ読み込み（表示用補完のため）
    try {
      const masterRes = await getAllPokemonMaster();
      pokemonMasterData = masterRes.pokemon;
    } catch (e) {
      console.error(e);
    }

    if (teamId) {
      await loadTeam(teamId);
    }
  });

  async function loadTeam(id: string) {
    loading = true;
    try {
      const team = await getTeam(id, accessToken);
      teamName = team.team_name;

      const members: (TeamMemberData | null)[] = Array(6).fill(null);
      for (let i = 0; i < 6; i++) {
        if (team.pokemon[i]) {
          members[i] = mapResponseToData(team.pokemon[i]);
          // 編集時もUsageStatsを取得しておく
          if (members[i]?.form_id) {
            loadUsageStats(members[i]!.form_id!);
          }
        }
      }
      teamMembers = members;
    } catch (e) {
      alert("チーム情報の取得に失敗しました");
      dispatch("cancel");
    } finally {
      loading = false;
    }
  }

  async function loadUsageStats(formId: number) {
    if (usageStatsCache[formId]) return;
    console.log(`[TeamBuilder] Loading usage stats for form_id: ${formId}`);
    try {
      const stats = await getPokemonUsage(formId);
      console.log(`[TeamBuilder] Usage stats loaded:`, stats);
      usageStatsCache[formId] = stats;
      usageStatsCache = usageStatsCache; // Trigger reactivity
    } catch (e) {
      console.error("[TeamBuilder] Failed to load usage stats", e);
    }
  }

  function mapResponseToData(res: PokemonResponseDto): TeamMemberData {
    return {
      form_id: res.form_id,
      pokemon_name: res.fullname,
      terastal_type: (res as any).terastal_type || "Normal",
      ev_hp: (res as any).ev_hp || 0,
      ev_attack: (res as any).ev_attack || 0,
      ev_defense: (res as any).ev_defense || 0,
      ev_special_attack: (res as any).ev_special_attack || 0,
      ev_special_defense: (res as any).ev_special_defense || 0,
      ev_speed: (res as any).ev_speed || 0,
      iv_hp: (res as any).iv_hp ?? 31,
      iv_attack: (res as any).iv_attack ?? 31,
      iv_defense: (res as any).iv_defense ?? 31,
      iv_special_attack: (res as any).iv_special_attack ?? 31,
      iv_special_defense: (res as any).iv_special_defense ?? 31,
      iv_speed: (res as any).iv_speed ?? 31,
      nature: (res as any).nature || "Hardy",
      ability: (res as any).ability || "",
      held_item: (res as any).held_item || undefined,
      moves: (res as any).moves || [],
    };
  }

  function openModal(index: number) {
    currentSlotIndex = index;
    showModal = true;
  }

  function handlePokemonSelect(event: CustomEvent<PokemonResponseDto>) {
    if (currentSlotIndex === null) return;

    const selected = event.detail;
    console.log(`[TeamBuilder] Pokemon selected:`, selected);

    // Usage Statsを取得
    if (selected.form_id) {
      loadUsageStats(selected.form_id);
    } else {
      console.warn("[TeamBuilder] Selected pokemon has no form_id!", selected);
    }

    const memberData: TeamMemberData = {
      form_id: selected.form_id,
      pokemon_name: selected.fullname,
      terastal_type: selected.terastal_type,
      ev_hp: selected.ev_hp,
      ev_attack: selected.ev_attack,
      ev_defense: selected.ev_defense,
      ev_special_attack: selected.ev_special_attack,
      ev_special_defense: selected.ev_special_defense,
      ev_speed: selected.ev_speed,
      iv_hp: selected.iv_hp,
      iv_attack: selected.iv_attack,
      iv_defense: selected.iv_defense,
      iv_special_attack: selected.iv_special_attack,
      iv_special_defense: selected.iv_special_defense,
      iv_speed: selected.iv_speed,
      nature: selected.nature,
      ability: selected.ability,
      held_item: selected.held_item || undefined,
      moves: selected.moves,
    };

    teamMembers[currentSlotIndex] = memberData;
    showModal = false;
    currentSlotIndex = null;
  }

  function removeMember(index: number) {
    teamMembers[index] = null;
  }

  async function handleSave() {
    if (!teamName) {
      alert("チーム名を入力してください");
      return;
    }

    const validMembers = teamMembers.filter(
      (m) => m !== null,
    ) as PokemonDataDto[];

    if (validMembers.length === 0) {
      if (!confirm("メンバーがいませんが保存しますか？")) return;
    }

    loading = true;
    try {
      if (teamId) {
        const request: UpdateTeamRequestDto = {
          team_name: teamName,
          pokemon: validMembers,
        };
        await updateTeam(teamId, request, accessToken);
      } else {
        const createReq: CreateTeamRequestDto = {
          team_name: teamName,
        };
        const newTeam = await createTeam(createReq, accessToken);

        if (validMembers.length > 0) {
          const updateReq: UpdateTeamRequestDto = {
            team_name: teamName,
            pokemon: validMembers,
          };
          await updateTeam(newTeam.team_id, updateReq, accessToken);
        }
      }
      dispatch("save");
    } catch (e: any) {
      alert("保存に失敗しました: " + (e.message || "不明なエラー"));
    } finally {
      loading = false;
    }
  }

  function getDisplayName(member: TeamMemberData): string {
    const master = pokemonMasterData.find(
      (p) => p.fullname === member.pokemon_name,
    );
    if (master?.fullname_ja) return master.fullname_ja;
    return member.pokemon_name;
  }
</script>

<div class="team-builder">
  <div class="header">
    <h2>{teamId ? "チーム編集" : "チーム新規作成"}</h2>
  </div>

  <div class="form-group team-name-group">
    <label for="team-name">チーム名</label>
    <input
      id="team-name"
      type="text"
      bind:value={teamName}
      placeholder="チーム名を入力..."
      maxlength="50"
    />
  </div>

  <div class="members-grid">
    {#each teamMembers as member, i}
      <div class="member-slot {member ? 'filled' : 'empty'}">
        {#if member}
          <div class="slot-content">
            <div class="slot-header">
              <span class="role-badge">Member {i + 1}</span>
              <button class="remove-btn" on:click={() => removeMember(i)}
                >×</button
              >
            </div>
            <div class="pokemon-display">
              <div class="pokemon-name">{getDisplayName(member)}</div>

              <!-- 簡易編集フォーム -->
              <div class="details-form">
                <!-- 持ち物 -->
                <div class="form-row">
                  <label>持ち物</label>
                  <input
                    type="text"
                    bind:value={member.held_item}
                    list="items-list-{i}"
                    placeholder="持ち物"
                  />
                  {#if member.form_id && usageStatsCache[member.form_id]}
                    <datalist id="items-list-{i}">
                      {#each usageStatsCache[member.form_id].items as item}
                        <option value={item.name}
                          >{item.percentage.toFixed(1)}%</option
                        >
                      {/each}
                    </datalist>
                  {/if}
                </div>

                <!-- 特性 -->
                <div class="form-row">
                  <label>特性</label>
                  <input
                    type="text"
                    bind:value={member.ability}
                    list="abilities-list-{i}"
                    placeholder="特性"
                  />
                  {#if member.form_id && usageStatsCache[member.form_id]}
                    <datalist id="abilities-list-{i}">
                      {#each usageStatsCache[member.form_id].abilities as ability}
                        <option value={ability.name}
                          >{ability.percentage.toFixed(1)}%</option
                        >
                      {/each}
                    </datalist>
                  {/if}
                </div>

                <!-- 性格 -->
                <div class="form-row">
                  <label>性格</label>
                  <input
                    type="text"
                    bind:value={member.nature}
                    list="natures-list-{i}"
                    placeholder="性格"
                  />
                  {#if member.form_id && usageStatsCache[member.form_id]}
                    <datalist id="natures-list-{i}">
                      {#each usageStatsCache[member.form_id].natures as nature}
                        <option value={nature.name}
                          >{nature.percentage.toFixed(1)}%</option
                        >
                      {/each}
                    </datalist>
                  {/if}
                </div>

                <!-- テラスタイプ -->
                <div class="form-row">
                  <label>テラス</label>
                  <input
                    type="text"
                    bind:value={member.terastal_type}
                    list="tera-list-{i}"
                    placeholder="Type"
                  />
                  {#if member.form_id && usageStatsCache[member.form_id]}
                    <datalist id="tera-list-{i}">
                      {#each usageStatsCache[member.form_id].tera_types as tera}
                        <option value={tera.name}
                          >{tera.percentage.toFixed(1)}%</option
                        >
                      {/each}
                    </datalist>
                  {/if}
                </div>
              </div>

              <div class="moves-list-edit">
                <label>技構成</label>
                {#if member.form_id && usageStatsCache[member.form_id]}
                  <datalist id="moves-list-{i}">
                    {#each usageStatsCache[member.form_id].moves as move}
                      <option value={move.name}
                        >{move.percentage.toFixed(1)}%</option
                      >
                    {/each}
                  </datalist>
                {/if}
                <div class="moves-grid">
                  {#each Array(4) as _, moveIndex}
                    <input
                      type="text"
                      bind:value={member.moves[moveIndex]}
                      list="moves-list-{i}"
                      placeholder="技{moveIndex + 1}"
                      class="move-input"
                    />
                  {/each}
                </div>
              </div>
            </div>
            <button class="edit-slot-btn" on:click={() => openModal(i)}
              >入れ替え</button
            >
          </div>
        {:else}
          <button class="add-btn" on:click={() => openModal(i)}>
            <span class="plus-icon">+</span>
            <span class="add-text">ポケモンを追加</span>
          </button>
        {/if}
      </div>
    {/each}
  </div>

  <div class="actions">
    <button class="btn-secondary" on:click={() => dispatch("cancel")}
      >キャンセル</button
    >
    <button class="btn-primary" on:click={handleSave} disabled={loading}>
      {loading ? "保存中..." : "保存する"}
    </button>
  </div>
</div>

{#if showModal}
  <PokemonSelectModal
    on:select={handlePokemonSelect}
    on:close={() => (showModal = false)}
  />
{/if}

<style>
  .team-builder {
    padding: 1rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .header h2 {
    margin-top: 0;
    color: #333;
  }

  .team-name-group {
    margin-bottom: 2rem;
  }

  .team-name-group label {
    display: block;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #555;
  }

  .team-name-group input {
    width: 100%;
    font-size: 1.25rem;
    padding: 0.75rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  .members-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .member-slot {
    min-height: 480px;
    border-radius: 8px;
    transition: all 0.2s;
    position: relative;
  }

  .member-slot.empty {
    border: 2px dashed #ccc;
    background: #fdfdfd;
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 280px;
  }

  .member-slot.empty:hover {
    border-color: #4caf50;
    background: #f0f9f0;
  }

  .member-slot.filled {
    border: 1px solid #ddd;
    background: white;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
    padding: 1rem;
  }

  .add-btn {
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
    color: #888;
    width: 100%;
    height: 100%;
    justify-content: center;
  }

  .plus-icon {
    font-size: 3rem;
    margin-bottom: 0.5rem;
  }

  .slot-content {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .slot-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }

  .role-badge {
    font-size: 0.8rem;
    background: #eee;
    padding: 2px 6px;
    border-radius: 4px;
    color: #555;
  }

  .remove-btn {
    background: none;
    border: none;
    color: #999;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
  }

  .remove-btn:hover {
    color: #f44336;
  }

  .pokemon-display {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .pokemon-name {
    font-size: 1.2rem;
    font-weight: 700;
    color: #333;
    margin-bottom: 1rem;
    text-align: center;
  }

  .details-form {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .form-row {
    display: flex;
    align-items: center;
    width: 100%;
  }

  .form-row label {
    width: 60px;
    font-size: 0.85rem;
    color: #666;
    font-weight: 600;
  }

  .form-row input {
    flex: 1;
    padding: 0.4rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .moves-list-edit {
    margin-bottom: 1rem;
  }

  .moves-list-edit label {
    display: block;
    font-size: 0.85rem;
    color: #666;
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  .moves-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
  }

  .move-input {
    width: 100%;
    padding: 0.4rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .edit-slot-btn {
    width: 100%;
    padding: 0.5rem;
    background: #fff;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    color: #555;
    margin-top: auto;
  }

  .edit-slot-btn:hover {
    background: #f5f5f5;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid #eee;
  }

  .btn-primary,
  .btn-secondary {
    padding: 0.75rem 2rem;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    border: none;
  }

  .btn-primary {
    background-color: #4caf50;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: #43a047;
  }

  .btn-primary:disabled {
    background-color: #a5d6a7;
    cursor: not-allowed;
  }

  .btn-secondary {
    background-color: #eee;
    color: #333;
  }

  .btn-secondary:hover {
    background-color: #e0e0e0;
  }
</style>
