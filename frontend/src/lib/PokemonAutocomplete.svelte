<script lang="ts">
    import { onMount } from "svelte";
    import type { PokemonMasterDto } from "./types/api";

    export let value: PokemonMasterDto | null = null;
    export let pokemonList: PokemonMasterDto[] = [];
    export let placeholder: string = "Search Pokemon...";
    export let required: boolean = false;

    let inputValue: string = value?.fullname_ja || value?.fullname || "";
    let isOpen = false;
    let filteredPokemon: PokemonMasterDto[] = [];
    let highlightedIndex = -1;

    $: {
        if (value) {
            inputValue = value.fullname_ja || value.fullname;
        }
    }

    function filterPokemon(query: string) {
        if (!query) {
            filteredPokemon = pokemonList.slice(0, 50); // Show first 50
            return;
        }

        const lowerQuery = query.toLowerCase();
        filteredPokemon = pokemonList
            .filter((p) => {
                const jaMatch = p.fullname_ja
                    ?.toLowerCase()
                    .includes(lowerQuery);
                const enMatch = p.fullname.toLowerCase().includes(lowerQuery);
                return jaMatch || enMatch;
            })
            .slice(0, 50); // Limit to 50
    }

    function handleInput(event: Event) {
        const target = event.target as HTMLInputElement;
        inputValue = target.value;
        filterPokemon(inputValue);
        isOpen = true;
        highlightedIndex = -1;
    }

    function selectPokemon(pokemon: PokemonMasterDto) {
        value = pokemon;
        inputValue = pokemon.fullname_ja || pokemon.fullname;
        isOpen = false;
        highlightedIndex = -1;
    }

    function handleKeydown(event: KeyboardEvent) {
        if (!isOpen) {
            if (event.key === "ArrowDown") {
                isOpen = true;
                filterPokemon(inputValue);
            }
            return;
        }

        switch (event.key) {
            case "ArrowDown":
                event.preventDefault();
                highlightedIndex = Math.min(
                    highlightedIndex + 1,
                    filteredPokemon.length - 1,
                );
                break;
            case "ArrowUp":
                event.preventDefault();
                highlightedIndex = Math.max(highlightedIndex - 1, 0);
                break;
            case "Enter":
                event.preventDefault();
                if (
                    highlightedIndex >= 0 &&
                    filteredPokemon[highlightedIndex]
                ) {
                    selectPokemon(filteredPokemon[highlightedIndex]);
                }
                break;
            case "Escape":
                isOpen = false;
                highlightedIndex = -1;
                break;
        }
    }

    function handleFocus() {
        filterPokemon(inputValue);
        isOpen = true;
    }

    function handleBlur() {
        // Delay to allow click event to fire
        setTimeout(() => {
            isOpen = false;
            highlightedIndex = -1;
        }, 200);
    }

    onMount(() => {
        filterPokemon("");
    });
</script>

<div class="pokemon-autocomplete relative w-full">
    <input
        type="text"
        bind:value={inputValue}
        on:input={handleInput}
        on:keydown={handleKeydown}
        on:focus={handleFocus}
        on:blur={handleBlur}
        {placeholder}
        {required}
        class="w-full bg-black border border-accents-2 rounded p-2 text-white focus:border-white outline-none transition-colors"
        autocomplete="off"
    />

    {#if isOpen && filteredPokemon.length > 0}
        <div
            class="absolute top-full left-0 right-0 max-h-[300px] overflow-y-auto bg-black border border-accents-2 rounded-md shadow-lg z-50 mt-1"
        >
            {#each filteredPokemon as pokemon, index (pokemon.form_id)}
                <button
                    type="button"
                    class="w-full p-3 border-b border-accents-2 last:border-0 text-left flex justify-between items-center transition-colors
                    {index === highlightedIndex
                        ? 'bg-accents-1'
                        : 'hover:bg-accents-1'}"
                    on:click={() => selectPokemon(pokemon)}
                >
                    <div class="flex flex-col gap-0.5">
                        <span class="font-bold text-white text-sm">
                            {pokemon.fullname_ja || pokemon.fullname}
                        </span>
                        {#if pokemon.fullname_ja && pokemon.fullname !== pokemon.fullname_ja}
                            <span class="text-xs text-accents-5">
                                ({pokemon.fullname})
                            </span>
                        {/if}
                        {#if pokemon.usage}
                            <span class="text-xs text-green-400">
                                Usage: {(pokemon.usage * 100).toFixed(2)}%
                            </span>
                        {/if}
                    </div>
                    <div class="flex gap-1">
                        <span
                            class="px-2 py-0.5 rounded-full text-xs bg-accents-2 text-white capitalize"
                            >{pokemon.type1}</span
                        >
                        {#if pokemon.type2}
                            <span
                                class="px-2 py-0.5 rounded-full text-xs bg-accents-2 text-white capitalize"
                                >{pokemon.type2}</span
                            >
                        {/if}
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    /* Scoped styles removed in favor of Tailwind classes in template */
</style>
