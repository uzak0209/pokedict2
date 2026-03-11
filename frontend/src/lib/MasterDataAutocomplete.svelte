<script lang="ts">
    import type { UsageDetailDto } from "./api/pokemonMaster";
    import { TYPE_COLORS } from "../data/typeColors";
    import type { PokemonType } from "../types/pokemon";

    export let options: UsageDetailDto[] = [];
    export let value: string = "";
    export let placeholder: string = "Select...";
    export let required: boolean = false;
    export let loading: boolean = false;

    let inputValue: string = "";
    let isOpen = false;
    let filteredOptions: UsageDetailDto[] = [];
    let highlightedIndex = -1;

    // Update display value when value prop changes or options load
    $: {
        if (value) {
            const selected = options.find((o) => o.name === value);
            if (selected) {
                inputValue = selected.name_ja || selected.name;
            } else {
                inputValue = value;
            }
        } else {
            inputValue = "";
        }
    }

    // Update filter when options change
    $: if (options.length > 0) {
        filterOptions(inputValue);
    }

    function filterOptions(query: string) {
        if (!query) {
            filteredOptions = options.slice(0, 30);
            return;
        }

        const lowerQuery = query.toLowerCase();
        filteredOptions = options
            .filter((o) => {
                const jaMatch = o.name_ja?.toLowerCase().includes(lowerQuery);
                const enMatch = o.name.toLowerCase().includes(lowerQuery);
                return jaMatch || enMatch;
            })
            .slice(0, 30);
    }

    function handleInput(event: Event) {
        const target = event.target as HTMLInputElement;
        inputValue = target.value;
        filterOptions(inputValue);
        isOpen = true;
        highlightedIndex = -1;
    }

    function selectOption(option: UsageDetailDto) {
        value = option.name;
        inputValue = option.name_ja || option.name;
        isOpen = false;
        highlightedIndex = -1;
    }

    function handleKeydown(event: KeyboardEvent) {
        if (!isOpen) {
            if (event.key === "ArrowDown") {
                isOpen = true;
                filterOptions(inputValue);
            }
            return;
        }

        switch (event.key) {
            case "ArrowDown":
                event.preventDefault();
                highlightedIndex = Math.min(
                    highlightedIndex + 1,
                    filteredOptions.length - 1,
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
                    filteredOptions[highlightedIndex]
                ) {
                    selectOption(filteredOptions[highlightedIndex]);
                }
                break;
            case "Escape":
                isOpen = false;
                highlightedIndex = -1;
                break;
        }
    }

    function handleFocus() {
        if (value) {
            inputValue = "";
            value = "";
        }
        filterOptions("");
        isOpen = true;
    }

    function handleBlur() {
        setTimeout(() => {
            isOpen = false;
            highlightedIndex = -1;
        }, 200);
    }
</script>

<div class="master-autocomplete relative w-full">
    <input
        type="text"
        bind:value={inputValue}
        on:input={handleInput}
        on:keydown={handleKeydown}
        on:focus={handleFocus}
        on:blur={handleBlur}
        {placeholder}
        {required}
        class="w-full bg-black border border-accents-2 rounded p-2 text-white focus:border-white outline-none transition-colors {loading
            ? 'opacity-50'
            : ''}"
        autocomplete="off"
    />

    {#if loading}
        <div
            class="absolute top-full left-0 right-0 p-2 bg-black border border-accents-2 rounded-md shadow-lg z-50 mt-1 text-accents-5 text-sm text-center"
        >
            Loading...
        </div>
    {:else if isOpen && filteredOptions.length > 0}
        <div
            class="absolute top-full left-0 right-0 max-h-[250px] overflow-y-auto bg-black border border-accents-2 rounded-md shadow-lg z-50 mt-1"
        >
            {#each filteredOptions as option, index (option.name)}
                <button
                    type="button"
                    class="w-full p-2 border-b border-accents-2 last:border-0 text-left flex justify-between items-center transition-colors
                    {index === highlightedIndex
                        ? 'bg-accents-1'
                        : 'hover:bg-accents-1'}"
                    on:click={() => selectOption(option)}
                >
                    <div class="flex items-center gap-2">
                        {#if option.type && TYPE_COLORS[option.type]}
                            <span
                                class="w-3 h-3 rounded-full"
                                style="background-color: {TYPE_COLORS[
                                    option.type
                                ]}"
                            ></span>
                        {/if}
                        <span class="font-medium text-white">
                            {option.name_ja || option.name}
                        </span>
                        {#if option.name_ja && option.name !== option.name_ja}
                            <span class="text-xs text-accents-5">
                                ({option.name})
                            </span>
                        {/if}
                    </div>
                    <span class="text-xs text-green-400 font-medium">
                        {option.percentage.toFixed(1)}%
                    </span>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    /* Scoped styles removed in favor of Tailwind classes in template */
</style>
