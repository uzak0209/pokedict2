<script lang="ts">
  import type { TeamMember, TeamRole } from '../types/pokemon';
  import TypeBadge from './TypeBadge.svelte';

  export let member: TeamMember | null;
  export let role: TeamRole;
  export let slotNumber: number;
  export let onSelect: () => void;
  export let onRemove: () => void;

  const roleColors = {
    core: 'border-blue-400 bg-blue-50',
    support: 'border-green-400 bg-green-50'
  };

  const roleLabels = {
    core: '軸',
    support: '補完'
  };
</script>

{#if !member}
  <!-- Empty Slot -->
  <button
    on:click={onSelect}
    class="relative bg-white rounded-lg border-2 border-dashed border-gray-300 p-6 h-40 flex flex-col items-center justify-center hover:border-blue-400 hover:bg-blue-50 transition-colors cursor-pointer group"
  >
    <div class="absolute top-2 left-2 text-xs font-medium text-gray-400">
      {roleLabels[role]} {slotNumber}
    </div>
    <div class="text-gray-400 group-hover:text-blue-500 transition-colors">
      <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
    </div>
    <span class="mt-2 text-sm text-gray-500 group-hover:text-blue-600">
      ポケモンを追加
    </span>
  </button>
{:else}
  <!-- Filled Slot -->
  <div class="relative bg-white rounded-lg shadow-md border-2 {roleColors[role]} p-4 hover:shadow-lg transition-shadow group">
    <!-- Role Label -->
    <div class="absolute top-2 left-2 text-xs font-medium text-gray-500 bg-white px-2 py-0.5 rounded">
      {roleLabels[role]} {slotNumber}
    </div>

    <!-- Remove Button -->
    <button
      on:click={onRemove}
      class="absolute top-2 right-2 p-1 rounded-full bg-red-500 text-white hover:bg-red-600 opacity-0 group-hover:opacity-100 transition-opacity"
      aria-label="削除"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>

    <!-- Pokemon Info -->
    <div class="mt-6">
      <div class="flex items-start justify-between mb-2">
        <div class="flex-1">
          <h3 class="font-bold text-lg text-gray-900">{member.pokemon.nameJp}</h3>
          <p class="text-sm text-gray-600">{member.pokemon.name}</p>
        </div>
      </div>

      <!-- Types -->
      <div class="flex gap-1 flex-wrap mb-3">
        {#each member.pokemon.types as type}
          <TypeBadge {type} size="sm" />
        {/each}
      </div>

      <!-- Item/Ability -->
      {#if member.pokemon.item || member.pokemon.ability}
        <div class="text-xs text-gray-600 space-y-1">
          {#if member.pokemon.item}
            <div class="flex items-center gap-1">
              <span class="font-semibold">持ち物:</span>
              <span>{member.pokemon.item}</span>
            </div>
          {/if}
          {#if member.pokemon.ability}
            <div class="flex items-center gap-1">
              <span class="font-semibold">特性:</span>
              <span>{member.pokemon.ability}</span>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}
