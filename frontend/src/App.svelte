<script lang="ts">
  import { onMount } from "svelte";
  import Auth from "./lib/Auth.svelte";
  import PokemonManager from "./lib/PokemonManager.svelte";
  import MatchupTable from "./lib/MatchupTable.svelte";
  import MatchupMatrix from "./lib/MatchupMatrix.svelte";
  import TeamsManager from "./lib/TeamsManager.svelte";
  import Navbar from "./lib/components/ui/Navbar.svelte";
  import { authStore } from "./stores/auth";

  // Use state rune if Svelte 5, but strictly sticking to standard let for compatibility if not fully migrated to runes.
  // The user codebase showed `let { ... } = $props()` in my previous files which implies Svelte 5 runes.
  // However, App.svelte is the root and was using `let activeTab = ...` script syntax before.
  // I will stick to Svelte 4/5 script syntax without explicit runes for top-level variable unless I see $state used elsewhere in App.svelte.
  // Wait, I used $props() in the components I created. I should consistently use Svelte 5 runes if possible, or fallback to Svelte 4 syntax if unsure.
  // The existing App.svelte was Svelte 3/4 style. I will update it to be cleaner but maybe stick to standard svelte syntax for top level derived/state if I don't want to fully rewrite logic.
  // Actually, I'll update to use the Navbar component I just made.

  // @ts-ignore
  let activeTab = "pokemon";

  onMount(() => {
    authStore.init();
  });
</script>

{#if !$authStore.isAuthenticated}
  <Auth />
{:else}
  <div class="min-h-screen flex flex-col">
    <Navbar {activeTab} onTabChange={(tab) => (activeTab = tab)} />

    <main class="flex-1 w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {#if activeTab === "pokemon"}
        <PokemonManager />
      {:else if activeTab === "matchups"}
        <MatchupTable />
      {:else if activeTab === "matrix"}
        <MatchupMatrix />
      {:else}
        <TeamsManager />
      {/if}
    </main>
  </div>
{/if}
