<script lang="ts">
    import { authStore } from "../../../stores/auth";
    import Button from "./Button.svelte";

    let {
        activeTab,
        onTabChange,
    }: { activeTab: string; onTabChange: (tab: string) => void } = $props();

    const tabs = [
        { id: "pokemon", label: "Pokemon" },
        { id: "matchups", label: "Matchups" },
        { id: "matrix", label: "Matrix" },
        { id: "teams", label: "Teams" },
    ];

    function handleLogout() {
        authStore.logout();
    }
</script>

<nav
    class="sticky top-0 z-50 w-full border-b border-accents-2 bg-black/50 backdrop-blur-xl"
>
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between">
            <!-- Logo / Brand -->
            <div class="flex items-center gap-8">
                <div class="flex-shrink-0">
                    <span
                        class="text-xl font-bold tracking-tight text-white flex items-center gap-2"
                    >
                        <!-- Vercel-like triangle logo simplified -->
                        <svg
                            height="24"
                            viewBox="0 0 75 65"
                            fill="white"
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-auto"
                        >
                            <path d="M37.59.25l36.95 64H.64l36.95-64z" />
                        </svg>
                        PokeDict
                    </span>
                </div>

                <!-- Desktop Nav -->
                <div class="hidden md:block">
                    <div class="flex items-baseline space-x-1">
                        {#each tabs as tab}
                            <button
                                onclick={() => onTabChange(tab.id)}
                                class="px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200
                {activeTab === tab.id
                                    ? 'bg-accents-2 text-white'
                                    : 'text-accents-5 hover:text-white hover:bg-accents-1'}"
                            >
                                {tab.label}
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- User & Logout -->
            <div class="flex items-center gap-4">
                <span class="text-sm text-accents-5 hidden sm:block">
                    {$authStore.user?.username}
                </span>
                <Button variant="secondary" size="sm" onclick={handleLogout}
                    >Log Out</Button
                >
            </div>
        </div>
    </div>
</nav>
