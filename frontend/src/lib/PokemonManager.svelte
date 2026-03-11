<script lang="ts">
    import { onMount } from "svelte";
    import { createPokemon } from "./api/pokemon";
    import PokemonList from "./PokemonList.svelte";
    import PokemonDetail from "./PokemonDetail.svelte";
    import PokemonForm from "./PokemonForm.svelte";
    import type { CreatePokemonRequestDto } from "./types/api";
    import { authStore } from "../stores/auth";
    import Button from "./components/ui/Button.svelte";

    // 認証ストアからアクセストークンとユーザー情報を取得
    let accessToken = "";

    authStore.subscribe((state) => {
        accessToken = state.accessToken || "";
    });

    let currentMode: "list" | "detail" | "new" = "list";
    let currentId: string | null = null;

    onMount(() => {
        handleHashChange();
        window.addEventListener("hashchange", handleHashChange);
        return () => window.removeEventListener("hashchange", handleHashChange);
    });

    function handleHashChange() {
        const hash = window.location.hash.slice(1) || "/pokemon";

        if (hash === "/pokemon" || hash === "") {
            currentMode = "list";
            currentId = null;
        } else if (hash === "/pokemon/new") {
            currentMode = "new";
            currentId = null;
        } else if (hash.startsWith("/pokemon/")) {
            currentMode = "detail";
            currentId = hash.split("/")[2];
        }
    }

    async function handleCreatePokemon(
        event: CustomEvent<CreatePokemonRequestDto>,
    ) {
        try {
            await createPokemon(event.detail, accessToken);
            window.location.hash = "/pokemon";
        } catch (e: any) {
            alert("Registration failed: " + (e.message || "Unknown error"));
        }
    }

    function handleCancel() {
        window.location.hash = "/pokemon";
    }
</script>

<div class="space-y-6">
    <div
        class="flex justify-between items-center bg-black/50 p-6 rounded-xl border border-accents-2 backdrop-blur-sm"
    >
        <div>
            <h2 class="text-2xl font-bold text-white">Pokemon Manager</h2>
            <p class="text-accents-5">Manage your master pokemon database.</p>
        </div>
        {#if currentMode === "list"}
            <a href="#/pokemon/new">
                <Button variant="primary">New Pokemon</Button>
            </a>
        {:else}
            <Button variant="ghost" onclick={handleCancel}>Cancel</Button>
        {/if}
    </div>

    <main class="min-h-[60vh]">
        {#if currentMode === "list"}
            <PokemonList {accessToken} />
        {:else if currentMode === "new"}
            <PokemonForm
                on:submit={handleCreatePokemon}
                on:cancel={handleCancel}
            />
        {:else if currentMode === "detail" && currentId}
            <PokemonDetail pokemonId={currentId} {accessToken} />
        {/if}
    </main>
</div>
