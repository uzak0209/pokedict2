<script lang="ts">
    import { onMount } from "svelte";
    import type { PokemonMasterDto } from "./types/api";

    export let value: PokemonMasterDto | null = null;
    export let pokemonList: PokemonMasterDto[] = [];
    export let placeholder: string = "ポケモンを検索...";
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
            filteredPokemon = pokemonList.slice(0, 50); // 最初の50件を表示
            return;
        }

        const lowerQuery = query.toLowerCase();
        filteredPokemon = pokemonList
            .filter((p) => {
                const jaMatch = p.fullname_ja?.toLowerCase().includes(lowerQuery);
                const enMatch = p.fullname.toLowerCase().includes(lowerQuery);
                return jaMatch || enMatch;
            })
            .slice(0, 50); // 最大50件まで表示
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
                if (highlightedIndex >= 0 && filteredPokemon[highlightedIndex]) {
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
        // ちょっと遅延させて、クリックイベントが先に発火するようにする
        setTimeout(() => {
            isOpen = false;
            highlightedIndex = -1;
        }, 200);
    }

    onMount(() => {
        filterPokemon("");
    });
</script>

<div class="pokemon-autocomplete">
    <input
        type="text"
        bind:value={inputValue}
        on:input={handleInput}
        on:keydown={handleKeydown}
        on:focus={handleFocus}
        on:blur={handleBlur}
        {placeholder}
        {required}
        class="autocomplete-input"
        autocomplete="off"
    />

    {#if isOpen && filteredPokemon.length > 0}
        <div class="autocomplete-dropdown">
            {#each filteredPokemon as pokemon, index (pokemon.form_id)}
                <button
                    type="button"
                    class="autocomplete-item"
                    class:highlighted={index === highlightedIndex}
                    on:click={() => selectPokemon(pokemon)}
                >
                    <div class="pokemon-info">
                        <span class="pokemon-name">
                            {pokemon.fullname_ja || pokemon.fullname}
                        </span>
                        {#if pokemon.fullname_ja && pokemon.fullname !== pokemon.fullname_ja}
                            <span class="pokemon-name-en">
                                ({pokemon.fullname})
                            </span>
                        {/if}
                        {#if pokemon.usage}
                            <span class="pokemon-usage">
                                使用率: {(pokemon.usage * 100).toFixed(2)}%
                            </span>
                        {/if}
                    </div>
                    <div class="pokemon-types">
                        <span class="type-badge">{pokemon.type1}</span>
                        {#if pokemon.type2}
                            <span class="type-badge">{pokemon.type2}</span>
                        {/if}
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .pokemon-autocomplete {
        position: relative;
        width: 100%;
    }

    .autocomplete-input {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid #d1d5db;
        border-radius: 0.375rem;
        font-size: 1rem;
        outline: none;
        transition: border-color 0.2s;
    }

    .autocomplete-input:focus {
        border-color: #3b82f6;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
    }

    .autocomplete-dropdown {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        max-height: 300px;
        overflow-y: auto;
        background: white;
        border: 1px solid #d1d5db;
        border-radius: 0.375rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        z-index: 1000;
        margin-top: 0.25rem;
    }

    .autocomplete-item {
        width: 100%;
        padding: 0.75rem;
        border: none;
        background: white;
        text-align: left;
        cursor: pointer;
        display: flex;
        justify-content: space-between;
        align-items: center;
        transition: background-color 0.15s;
    }

    .autocomplete-item:hover,
    .autocomplete-item.highlighted {
        background-color: #f3f4f6;
    }

    .pokemon-info {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .pokemon-name {
        font-weight: 600;
        color: #111827;
    }

    .pokemon-name-en {
        font-size: 0.875rem;
        color: #6b7280;
    }

    .pokemon-usage {
        font-size: 0.75rem;
        color: #9ca3af;
    }

    .pokemon-types {
        display: flex;
        gap: 0.25rem;
    }

    .type-badge {
        padding: 0.125rem 0.5rem;
        background-color: #e5e7eb;
        color: #374151;
        font-size: 0.75rem;
        border-radius: 9999px;
        text-transform: capitalize;
    }
</style>
