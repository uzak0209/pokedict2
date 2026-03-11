<script lang="ts">
    let {
        type = "text",
        value = $bindable(),
        placeholder = "",
        required = false,
        disabled = false,
        class: className = "",
        ...rest
    }: {
        type?: string;
        value?: string | number;
        placeholder?: string;
        required?: boolean;
        disabled?: boolean;
        class?: string;
        [key: string]: any;
    } = $props();
</script>

<input
    {type}
    bind:value
    {placeholder}
    {required}
    {disabled}
    class="flex h-10 w-full rounded-md border border-accents-2 bg-black px-3 py-2 text-sm text-white placeholder:text-accents-4 focus:outline-none focus:ring-2 focus:ring-white/20 disabled:cursor-not-allowed disabled:opacity-50 {className}"
    {...rest}
/>
