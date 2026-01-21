<script lang="ts">
  import { onMount } from "svelte";
  import Auth from "./lib/Auth.svelte";
  import PokemonManager from "./lib/PokemonManager.svelte";
  import { authStore } from "./stores/auth";

  onMount(() => {
    authStore.init();
  });

  function handleLogout() {
    authStore.logout();
  }
</script>

{#if !$authStore.isAuthenticated}
  <Auth />
{:else}
  <div class="app-container">
    <header class="app-header">
      <div class="user-info">
        <span>{$authStore.user?.username}</span>
        <button on:click={handleLogout} class="logout-btn">ログアウト</button>
      </div>
    </header>
    <PokemonManager />
  </div>
{/if}

<style>
  .app-container {
    min-height: 100vh;
    background: #f5f5f5;
  }

  .app-header {
    background: #333;
    color: white;
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: flex-end;
  }

  .user-info {
    display: flex;
    align-items: center;
    gap: 1rem;
    font-size: 0.9rem;
  }

  .logout-btn {
    background: #555;
    color: white;
    border: none;
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    cursor: pointer;
  }

  .logout-btn:hover {
    background: #666;
  }
</style>
