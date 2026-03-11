<script lang="ts">
    import { authStore } from "../stores/auth";
    import TeamList from "./TeamList.svelte";
    import TeamBuilder from "./TeamBuilder.svelte";
    import Button from "./components/ui/Button.svelte";
    import { onMount } from "svelte";

    let mode: "list" | "new" | "edit" = "list";
    let selectedTeamId: string | null = null;
    let userId = "";

    authStore.subscribe((state) => {
        userId = state.user?.userId || "";
    });

    // Handle Internal Routing
    function handleEdit(event: CustomEvent<string>) {
        selectedTeamId = event.detail;
        mode = "edit";
    }

    function handleCreate() {
        selectedTeamId = null;
        mode = "new";
    }

    function handleSave() {
        mode = "list";
        selectedTeamId = null;
    }

    function handleCancel() {
        mode = "list";
        selectedTeamId = null;
    }
</script>

<div class="space-y-6">
    {#if mode === "list"}
        <div
            class="flex justify-between items-center bg-black/50 p-6 rounded-xl border border-accents-2 backdrop-blur-sm"
        >
            <div>
                <h2 class="text-2xl font-bold text-white">Your Teams</h2>
                <p class="text-accents-5">
                    Manage and analyze your Pokemon teams.
                </p>
            </div>
            <Button variant="primary" onclick={handleCreate}>
                Create New Team
            </Button>
        </div>

        <TeamList {userId} on:edit={handleEdit} />
    {:else if mode === "new"}
        <div class="mb-4">
            <Button variant="ghost" onclick={handleCancel} class="pl-0 gap-2">
                ← Back to Teams
            </Button>
        </div>
        <TeamBuilder on:save={handleSave} on:cancel={handleCancel} />
    {:else if mode === "edit" && selectedTeamId}
        <div class="mb-4">
            <Button variant="ghost" onclick={handleCancel} class="pl-0 gap-2">
                ← Back to Teams
            </Button>
        </div>
        <TeamBuilder on:save={handleSave} on:cancel={handleCancel} />
    {/if}
</div>
