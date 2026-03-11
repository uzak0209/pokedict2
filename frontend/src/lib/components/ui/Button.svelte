<script lang="ts">
    type Variant = "primary" | "secondary" | "ghost" | "danger";
    type Size = "sm" | "md" | "lg";

    let {
        variant = "primary",
        size = "md",
        class: className = "",
        children,
        onclick,
        disabled = false,
        ...rest
    }: {
        variant?: Variant;
        size?: Size;
        class?: string;
        children?: any;
        onclick?: (e: MouseEvent) => void;
        disabled?: boolean;
        [key: string]: any;
    } = $props();

    const baseStyles =
        "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background";

    const variants = {
        primary: "bg-white text-black hover:bg-gray-200",
        secondary:
            "bg-accents-2 text-white border border-accents-2 hover:bg-accents-3",
        ghost: "hover:bg-accents-2 text-white",
        danger: "bg-red-600 text-white hover:bg-red-700",
    };

    const sizes = {
        sm: "h-8 px-3 text-xs",
        md: "h-10 py-2 px-4",
        lg: "h-12 px-8",
    };
</script>

<button
    class="{baseStyles} {variants[variant]} {sizes[size]} {className}"
    {onclick}
    {disabled}
    {...rest}
>
    {@render children?.()}
</button>
