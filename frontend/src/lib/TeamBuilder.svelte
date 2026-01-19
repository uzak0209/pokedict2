<script lang="ts">
  import { writable } from 'svelte/store';
  import type { TeamMember } from '../types/pokemon';
  import PokemonSlot from './PokemonSlot.svelte';

  // 軸4体 + 補完2体の構成
  let coreMembers = writable<(TeamMember | null)[]>(Array(4).fill(null));
  let supportMembers = writable<(TeamMember | null)[]>(Array(2).fill(null));
  let teamName = '新しいパーティ';

  function addCoreMember(index: number) {
    console.log(`Add core member at index ${index}`);
    // TODO: ポケモン選択モーダルを開く
  }

  function addSupportMember(index: number) {
    console.log(`Add support member at index ${index}`);
    // TODO: ポケモン選択モーダルを開く
  }

  function removeMember(role: 'core' | 'support', index: number) {
    if (role === 'core') {
      coreMembers.update(members => {
        members[index] = null;
        return members;
      });
    } else {
      supportMembers.update(members => {
        members[index] = null;
        return members;
      });
    }
  }
</script>

<div class="container mx-auto p-6 max-w-7xl">
  <!-- Header -->
  <div class="mb-8">
    <h1 class="text-4xl font-bold text-gray-900 mb-2">
      ポケモン構築ビルダー
    </h1>
    <p class="text-gray-600">
      軸4体 + 補完2体の構築理論に基づいたパーティ作成
    </p>
  </div>

  <!-- Team Name -->
  <div class="bg-white rounded-lg shadow-md p-4 mb-6">
    <label for="team-name" class="block text-sm font-medium text-gray-700 mb-2">
      パーティ名
    </label>
    <input
      id="team-name"
      type="text"
      bind:value={teamName}
      class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
      placeholder="パーティ名を入力..."
    />
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Main Team Area -->
    <div class="lg:col-span-2 space-y-6">
      <!-- 軸4体 -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-2xl font-bold text-gray-900">
            軸ポケモン
          </h2>
          <span class="text-sm text-gray-500 bg-blue-100 px-3 py-1 rounded-full">
            4体
          </span>
        </div>
        <p class="text-sm text-gray-600 mb-4">
          行動保証があるポケモンやテラス込みで動けるポケモンから組む
        </p>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          {#each $coreMembers as member, index}
            <PokemonSlot
              {member}
              role="core"
              slotNumber={index + 1}
              onSelect={() => addCoreMember(index)}
              onRemove={() => removeMember('core', index)}
            />
          {/each}
        </div>
      </div>

      <!-- 補完2体 -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-2xl font-bold text-gray-900">
            補完ポケモン
          </h2>
          <span class="text-sm text-gray-500 bg-green-100 px-3 py-1 rounded-full">
            2体
          </span>
        </div>
        <p class="text-sm text-gray-600 mb-4">
          軸4体で重いポケモンの対策枠
        </p>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          {#each $supportMembers as member, index}
            <PokemonSlot
              {member}
              role="support"
              slotNumber={index + 1}
              onSelect={() => addSupportMember(index)}
              onRemove={() => removeMember('support', index)}
            />
          {/each}
        </div>
      </div>
    </div>

    <!-- Sidebar -->
    <div class="space-y-6">
      <!-- Team Summary -->
      <div class="bg-white rounded-lg shadow-md p-6">
        <h3 class="text-lg font-bold text-gray-900 mb-4">パーティ構成</h3>

        <div class="space-y-3">
          <div class="flex items-center justify-between py-2 border-b">
            <span class="text-sm text-gray-600">軸ポケモン</span>
            <span class="font-bold text-blue-600">
              {$coreMembers.filter(m => m !== null).length}/4
            </span>
          </div>
          <div class="flex items-center justify-between py-2 border-b">
            <span class="text-sm text-gray-600">補完ポケモン</span>
            <span class="font-bold text-green-600">
              {$supportMembers.filter(m => m !== null).length}/2
            </span>
          </div>
          <div class="flex items-center justify-between py-2">
            <span class="text-sm font-medium text-gray-700">合計</span>
            <span class="font-bold text-gray-900">
              {$coreMembers.filter(m => m !== null).length + $supportMembers.filter(m => m !== null).length}/6
            </span>
          </div>
        </div>
      </div>

      <!-- Construction Theory -->
      <div class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-lg p-6 border border-blue-200">
        <h3 class="text-lg font-bold text-gray-900 mb-3">💡 構築のポイント</h3>
        <ul class="space-y-2 text-sm text-gray-700">
          <li class="flex items-start">
            <span class="mr-2">1.</span>
            <span>行動保証のあるポケモンから組み始める</span>
          </li>
          <li class="flex items-start">
            <span class="mr-2">2.</span>
            <span>相性の良いポケモン3体を追加</span>
          </li>
          <li class="flex items-start">
            <span class="mr-2">3.</span>
            <span>軸4体で重いポケモンを確認</span>
          </li>
          <li class="flex items-start">
            <span class="mr-2">4.</span>
            <span>対策となる補完ポケモン2体を追加</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</div>
