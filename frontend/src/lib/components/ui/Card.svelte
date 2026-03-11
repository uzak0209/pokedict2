<script lang="ts">
    let {
        class: className = "",
        children,
        ...rest
    }: {
        class?: string;
        children?: any;
        [key: string]: any;
    } = $props();
</script>

<div
    class="rounded-xl border border-accents-2 bg-black/50 backdrop-blur-sm p-6 {className}"
    {...rest}
>
    {@render children?.()}
</div>
