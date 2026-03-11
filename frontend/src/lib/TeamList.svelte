<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { getUserTeams, deleteTeam } from "./api/teams";
    import type { TeamResponseDto } from "./types/api";
    import { authStore } from "../stores/auth";
    import Card from "./components/ui/Card.svelte";
    import Button from "./components/ui/Button.svelte";

    export let userId: string;

    const dispatch = createEventDispatcher();

    let teams: TeamResponseDto[] = [];
    let loading = true;
    let error: string | null = null;
    let accessToken: string = "";

    authStore.subscribe((state) => {
        accessToken = state.accessToken || "";
    });

    onMount(async () => {
        await loadTeams();
    });

    async function loadTeams() {
        loading = true;
        error = null;
        try {
            teams = await getUserTeams(userId, accessToken);
        } catch (e: any) {
            error = e.message;
        } finally {
            loading = false;
        }
    }

    function handleCreate() {
        // Since we are using TeamsManager, we can't just change hash if TeamsManager uses local state.
        // Wait, TeamsManager logic was:
        // function handleCreate() { mode = "new"; }
        // BUT TeamList is a child of TeamsManager.
        // TeamList.svelte had `handleCreate` which did `window.location.hash = "/teams/new"`.
        // Now TeamsManager handles "List" vs "New" via state.
        // I need to emit an event from TeamList to parent to request "create".
        // But TeamsManager doesn't listen to a "create" event from TeamList, it has a "Create New Team" button in its own header (see TeamsManager.svelte).
        // So the "Create" button INSIDE TeamList is actually redundant if TeamsManager has one.
        // HOWEVER, TeamsManager's header button calls `handleCreate`.
        // TeamList might be empty, so it might show an empty state with a create button.
        // I should emit a "create" event if I keep the button inside TeamList.
        dispatch("create");
    }

    function handleEdit(teamId: string) {
        dispatch("edit", teamId);
    }

    async function handleDelete(teamId: string) {
        if (!confirm("Are you sure you want to delete this team?")) return;

        try {
            await deleteTeam(teamId, accessToken);
            teams = teams.filter((t) => t.team_id !== teamId);
        } catch (e: any) {
            alert("Failed to delete: " + e.message);
        }
    }
</script>

<div class="mt-6">
    {#if loading}
        <div class="text-center py-12 text-accents-5">Loading teams...</div>
    {:else if error}
        <div class="text-center py-12 text-red-500">{error}</div>
    {:else if teams.length === 0}
        <div
            class="text-center py-12 bg-accents-1 rounded-xl border border-accents-2 border-dashed"
        >
            <p class="text-accents-5 mb-4">No teams found.</p>
            <Button variant="primary" onclick={handleCreate}>
                Create your first team
            </Button>
        </div>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each teams as team}
                <Card
                    class="hover:border-accents-5 transition-colors duration-200 group relative"
                >
                    <div class="flex justify-between items-start mb-4">
                        <h3
                            class="text-lg font-semibold text-white truncate pr-8"
                        >
                            {team.team_name}
                        </h3>
                        <div
                            class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity absolute top-4 right-4 bg-black/80 rounded-md p-1 backdrop-blur-sm border border-accents-2"
                        >
                            <button
                                class="text-accents-5 hover:text-white p-1 rounded transition-colors"
                                onclick={() => handleEdit(team.team_id)}
                                title="Edit"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="16"
                                    height="16"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    ><path
                                        d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"
                                    ></path></svg
                                >
                            </button>
                            <button
                                class="text-accents-5 hover:text-red-500 p-1 rounded transition-colors"
                                onclick={() => handleDelete(team.team_id)}
                                title="Delete"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="16"
                                    height="16"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    ><polyline points="3 6 5 6 21 6"
                                    ></polyline><path
                                        d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                                    ></path></svg
                                >
                            </button>
                        </div>
                    </div>

                    <div class="flex gap-2 flex-wrap">
                        {#each team.pokemon as p}
                            <div class="flex flex-col items-center w-10">
                                <div
                                    class="w-10 h-10 rounded-full bg-accents-2 flex items-center justify-center overflow-hidden border border-accents-3 mb-1"
                                >
                                    <img
                                        src={`/icons/pokemon/${p.form_id}.png`}
                                        alt={p.fullname_jp}
                                        class="w-full h-full object-cover pixelated"
                                        onerror={(e) =>
                                            ((
                                                e.target as HTMLImageElement
                                            ).src =
                                                `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${p.form_id}.png`)}
                                    />
                                </div>
                                <span
                                    class="text-[10px] text-accents-5 truncate w-full text-center"
                                    >{p.fullname_jp}</span
                                >
                            </div>
                        {/each}
                        {#each Array(6 - team.pokemon.length) as _}
                            <div
                                class="flex flex-col items-center w-10 opacity-30"
                            >
                                <div
                                    class="w-8 h-8 rounded-full border border-accents-2 border-dashed flex items-center justify-center text-xs text-accents-5"
                                >
                                    -
                                </div>
                            </div>
                        {/each}
                    </div>
                </Card>
            {/each}
        </div>
    {/if}
</div>
