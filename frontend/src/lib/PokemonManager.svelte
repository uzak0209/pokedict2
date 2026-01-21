<script lang="ts">
    import { onMount } from "svelte";
    import { createPokemon } from "./api/pokemon";
    import PokemonList from "./PokemonList.svelte";
    import PokemonDetail from "./PokemonDetail.svelte";
    import PokemonForm from "./PokemonForm.svelte";
    import TeamList from "./TeamList.svelte";
    import TeamBuilder from "./TeamBuilder.svelte";
    import type { CreatePokemonRequestDto } from "./types/api";
    import { authStore } from "../stores/auth";

    // 認証ストアからアクセストークンとユーザー情報を取得
    let accessToken = "";
    let userId = "";

    authStore.subscribe((state) => {
        accessToken = state.accessToken || "";
        userId = state.user?.userId || "";
    });

    let currentSection: "pokemon" | "teams" = "pokemon";
    let currentMode: "list" | "detail" | "new" | "edit" = "list";
    let currentId: string | null = null;

    onMount(() => {
        handleHashChange();
        window.addEventListener("hashchange", handleHashChange);
        return () => window.removeEventListener("hashchange", handleHashChange);
    });

    function handleHashChange() {
        // デフォルトハッシュ
        const hash = window.location.hash.slice(1) || "/pokemon";

        if (hash === "/pokemon" || hash === "") {
            currentSection = "pokemon";
            currentMode = "list";
            currentId = null;
        } else if (hash === "/pokemon/new") {
            currentSection = "pokemon";
            currentMode = "new";
            currentId = null;
        } else if (hash.startsWith("/pokemon/")) {
            currentSection = "pokemon";
            currentMode = "detail";
            currentId = hash.split("/")[2];
        } else if (hash === "/teams") {
            currentSection = "teams";
            currentMode = "list";
            currentId = null;
        } else if (hash === "/teams/new") {
            currentSection = "teams";
            currentMode = "new"; // TeamBuilder (new)
            currentId = null;
        } else if (hash.startsWith("/teams/")) {
            currentSection = "teams";
            currentMode = "edit"; // TeamBuilder (edit)
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
            alert("登録に失敗しました: " + (e.message || "不明なエラー"));
        }
    }

    function handleCancel() {
        if (currentSection === "pokemon") {
            window.location.hash = "/pokemon";
        } else {
            window.location.hash = "/teams";
        }
    }

    function handleTeamCreate() {
        window.location.hash = "/teams/new";
    }

    function handleTeamEdit(event: CustomEvent<string>) {
        window.location.hash = `/teams/${event.detail}`;
    }
</script>

<div class="pokemon-manager">
    <nav class="navbar">
        <h1>
            PokeDict - {currentSection === "pokemon"
                ? "ポケモン管理"
                : "チーム管理"}
        </h1>
        <div class="nav-links">
            <a href="#/pokemon" class:active={currentSection === "pokemon"}
                >ポケモン管理</a
            >
            <a href="#/teams" class:active={currentSection === "teams"}
                >チーム構築</a
            >
        </div>
    </nav>

    <main>
        {#if currentSection === "pokemon"}
            {#if currentMode === "list"}
                <div class="section-actions">
                    <a href="#/pokemon/new" class="btn-primary"
                        >新規ポケモン登録</a
                    >
                </div>
                <PokemonList {accessToken} />
            {:else if currentMode === "new"}
                <PokemonForm
                    on:submit={handleCreatePokemon}
                    on:cancel={handleCancel}
                />
            {:else if currentMode === "detail" && currentId}
                <PokemonDetail pokemonId={currentId} {accessToken} />
            {/if}
        {:else if currentSection === "teams"}
            {#if currentMode === "list"}
                <!-- TeamList内で作成ボタンを持つため、ここでは不要だが一貫性のために置いても良い -->
                <TeamList {userId} on:edit={handleTeamEdit} />
            {:else if currentMode === "new"}
                <TeamBuilder
                    teamId={null}
                    on:save={() => (window.location.hash = "/teams")}
                    on:cancel={handleCancel}
                />
            {:else if currentMode === "edit" && currentId}
                <TeamBuilder
                    teamId={currentId}
                    on:save={() => (window.location.hash = "/teams")}
                    on:cancel={handleCancel}
                />
            {/if}
        {/if}
    </main>
</div>

<style>
    .pokemon-manager {
        min-height: 100vh;
        background: #f5f5f5;
    }

    .navbar {
        background: white;
        border-bottom: 2px solid #e0e0e0;
        padding: 1rem 2rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        position: sticky;
        top: 0;
        z-index: 100;
    }

    .navbar h1 {
        margin: 0;
        color: #333;
        font-size: 1.5rem;
    }

    .nav-links {
        display: flex;
        gap: 1.5rem;
    }

    .nav-links a {
        text-decoration: none;
        color: #666;
        font-weight: 600;
        padding: 0.5rem 1rem;
        border-radius: 4px;
        transition: all 0.2s;
    }

    .nav-links a:hover {
        background: #f0f0f0;
        color: #333;
    }

    .nav-links a.active {
        background: #4caf50;
        color: white;
    }

    main {
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .section-actions {
        margin-bottom: 2rem;
        display: flex;
        justify-content: flex-end;
    }

    .btn-primary {
        background-color: #4caf50;
        color: white;
        text-decoration: none;
        padding: 0.75rem 1.5rem;
        border-radius: 4px;
        font-weight: 600;
        transition: background 0.2s;
        display: inline-block;
    }

    .btn-primary:hover {
        background-color: #43a047;
    }
</style>
