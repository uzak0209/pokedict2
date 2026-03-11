<script lang="ts">
    import { onMount } from "svelte";
    import Card from "./components/ui/Card.svelte";

    interface MatrixPokemon {
        form_id: number;
        name: string;
        name_ja: string | null;
        is_setup: boolean;
    }

    interface MatrixCell {
        form_id: number;
        opponent_form_id: number;
        p: number;
        n: number;
    }

    interface MatchupMatrix {
        pokemon: MatrixPokemon[];
        cells: MatrixCell[];
    }

    let matrix: MatchupMatrix | null = null;
    let loading = true;
    let cellMap: Map<string, MatrixCell> = new Map();
    let limit = 30;

    onMount(() => {
        loadMatrix();
    });

    async function loadMatrix() {
        loading = true;
        try {
            const response = await fetch(
                `/api/pokemon/master/matrix?limit=${limit}`,
            );
            if (!response.ok) throw new Error("Failed to fetch matrix");
            matrix = await response.json();

            // セルデータをマップに変換
            cellMap = new Map();
            if (matrix) {
                for (const cell of matrix.cells) {
                    const key = `${cell.form_id}-${cell.opponent_form_id}`;
                    cellMap.set(key, cell);
                }
            }
        } catch (e) {
            console.error("Failed to load matrix:", e);
        } finally {
            loading = false;
        }
    }

    function getCell(
        formId: number,
        opponentId: number,
    ): MatrixCell | undefined {
        return cellMap.get(`${formId}-${opponentId}`);
    }

    function getCellSymbol(formId: number, opponentId: number): string {
        if (formId === opponentId) return "-";
        const cell = getCell(formId, opponentId);
        if (!cell || cell.n < 20) return "?";
        if (cell.p <= 0.45) return "○";
        if (cell.p >= 0.55) return "●"; // Use solid circle for loss or X? X is clearer. Let's stick to X.
        if (cell.p >= 0.55) return "✕";
        return "△";
    }

    function getCellClass(formId: number, opponentId: number): string {
        if (formId === opponentId) return "self";
        const cell = getCell(formId, opponentId);
        if (!cell || cell.n < 20) return "unknown";
        if (cell.p <= 0.45) return "win";
        if (cell.p >= 0.55) return "lose";
        return "even";
    }

    function getCellTitle(formId: number, opponentId: number): string {
        if (formId === opponentId) return "Same Pokemon";
        const cell = getCell(formId, opponentId);
        if (!cell) return "No Data";
        return `Win Rate: ${(cell.p * 100).toFixed(1)}% (n=${Math.round(cell.n)})`;
    }

    function getShortName(p: MatrixPokemon): string {
        const name = p.name_ja || p.name;
        return name.length > 6 ? name.slice(0, 5) + "…" : name;
    }
</script>

<div class="mt-6 space-y-6">
    <div class="space-y-2">
        <h2 class="text-3xl font-bold text-white">
            Matchup Matrix <span
                class="bg-accents-2 text-accents-5 text-sm px-2 py-1 rounded"
                >Top {limit}</span
            >
        </h2>
        <div class="flex items-center gap-4 text-sm">
            <span class="flex items-center gap-1.5 text-green-400 font-medium"
                ><span class="w-2 h-2 rounded-full bg-green-500"></span>○
                Advantage</span
            >
            <span class="flex items-center gap-1.5 text-yellow-400 font-medium"
                ><span class="w-2 h-2 rounded-full bg-yellow-500"></span>△ Even</span
            >
            <span class="flex items-center gap-1.5 text-red-400 font-medium"
                ><span class="w-2 h-2 rounded-full bg-red-500"></span>✕
                Disadvantage</span
            >
            <span class="flex items-center gap-1.5 text-accents-5"
                ><span class="w-2 h-2 rounded-full bg-accents-4"></span>? No
                Data</span
            >
        </div>
    </div>

    {#if loading}
        <div class="text-center py-12 text-accents-5">Loading matrix...</div>
    {:else if matrix}
        <Card class="p-0 overflow-hidden border-accents-2 bg-black/80">
            <div class="overflow-x-auto max-w-full custom-scrollbar">
                <table class="border-collapse text-xs whitespace-nowrap w-full">
                    <thead>
                        <tr>
                            <th
                                class="sticky top-0 left-0 z-30 bg-black/95 p-2 min-w-[80px] border-b border-r border-accents-2"
                            ></th>
                            {#each matrix.pokemon as opponent}
                                <th
                                    class="sticky top-0 z-20 bg-black/90 p-2 border-b border-accents-2 h-[100px] align-bottom text-left group hover:bg-accents-1 transition-colors min-w-[28px]"
                                    title={opponent.name_ja || opponent.name}
                                >
                                    <div
                                        class="writing-vertical text-accents-4 group-hover:text-white transition-colors flex items-center gap-1"
                                    >
                                        {getShortName(opponent)}
                                        {#if opponent.is_setup}
                                            <span
                                                class="text-[8px]"
                                                title="Setup">🛠️</span
                                            >
                                        {/if}
                                    </div>
                                </th>
                            {/each}
                        </tr>
                    </thead>
                    <tbody>
                        {#each matrix.pokemon as pokemon}
                            <tr class="hover:bg-accents-1/30">
                                <th
                                    class="sticky left-0 z-10 bg-black/90 p-2 border-r border-accents-2 text-right text-accents-4 font-medium min-w-[80px] group hover:bg-accents-1 hover:text-white transition-colors"
                                    title={pokemon.name_ja || pokemon.name}
                                >
                                    <div
                                        class="flex items-center justify-end gap-1"
                                    >
                                        {getShortName(pokemon)}
                                        {#if pokemon.is_setup}
                                            <span
                                                class="text-[8px]"
                                                title="Setup">🛠️</span
                                            >
                                        {/if}
                                    </div>
                                </th>
                                {#each matrix.pokemon as opponent}
                                    <td
                                        class="w-7 h-7 text-center border border-accents-1/20 cursor-help transition-colors
                                        {getCellClass(
                                            pokemon.form_id,
                                            opponent.form_id,
                                        )}"
                                        title={getCellTitle(
                                            pokemon.form_id,
                                            opponent.form_id,
                                        )}
                                    >
                                        {getCellSymbol(
                                            pokemon.form_id,
                                            opponent.form_id,
                                        )}
                                    </td>
                                {/each}
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </Card>
    {:else}
        <div class="text-center py-12 text-accents-5">
            No matrix data available
        </div>
    {/if}
</div>

<style>
    .writing-vertical {
        writing-mode: vertical-rl;
        text-orientation: mixed;
        transform: rotate(180deg);
        margin: 0 auto;
    }

    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: #111;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: #333;
        border-radius: 4px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: #555;
    }

    /* Cell coloring */
    .self {
        background-color: #222;
        color: transparent;
    }
    .win {
        background-color: rgba(22, 163, 74, 0.15); /* green-600/15 */
        color: #4ade80; /* green-400 */
    }
    .win:hover {
        background-color: rgba(22, 163, 74, 0.3);
    }
    .even {
        background-color: rgba(234, 179, 8, 0.15); /* yellow-500/15 */
        color: #facc15; /* yellow-400 */
    }
    .even:hover {
        background-color: rgba(234, 179, 8, 0.3);
    }
    .lose {
        background-color: rgba(220, 38, 38, 0.15); /* red-600/15 */
        color: #f87171; /* red-400 */
    }
    .lose:hover {
        background-color: rgba(220, 38, 38, 0.3);
    }
    .unknown {
        background-color: #1a1a1a;
        color: #444;
    }
</style>
