<script lang="ts">
  import type { PokemonType } from "../types/pokemon";
  import { TYPE_COLORS, TYPE_LABELS_JP } from "../data/typeColors";

  export let type: PokemonType;
  export let size: "sm" | "md" | "lg" = "md";

  const sizeClasses = {
    sm: "px-2 py-0.5 text-xs",
    md: "px-3 py-1 text-sm",
    lg: "px-4 py-1.5 text-base",
  };
  $: capitalizedType = (type &&
    type.charAt(0).toUpperCase() + type.slice(1).toLowerCase()) as PokemonType;
</script>

<span
  class="inline-block rounded-md font-bold text-white shadow-sm {sizeClasses[
    size
  ]}"
  style="background-color: {TYPE_COLORS[capitalizedType]}"
>
  {TYPE_LABELS_JP[capitalizedType]}
</span>
