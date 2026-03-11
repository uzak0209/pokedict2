<script lang="ts">
  import { authApi } from "$lib/api/auth";
  import { authStore } from "../stores/auth";
  import { ApiError } from "$lib/api/client";
  import Button from "./components/ui/Button.svelte";
  import Input from "./components/ui/Input.svelte";
  import Card from "./components/ui/Card.svelte";

  let mode: "login" | "register" = "login";
  let email = "";
  let password = "";
  let username = "";
  let error = "";
  let loading = false;

  async function handleSubmit() {
    error = "";
    loading = true;

    try {
      if (mode === "register") {
        // 登録
        await authApi.register({ username, email, password });
        // 登録成功後、自動的にログイン
        const loginResponse = await authApi.login({ email, password });
        authStore.login(loginResponse);
      } else {
        // ログイン
        const response = await authApi.login({ email, password });
        authStore.login(response);
      }
    } catch (err) {
      if (err instanceof ApiError) {
        error = err.message;
      } else {
        error = "エラーが発生しました";
      }
    } finally {
      loading = false;
    }
  }

  function toggleMode() {
    mode = mode === "login" ? "register" : "login";
    error = "";
  }
</script>

<div class="min-h-screen flex items-center justify-center p-4">
  <Card class="w-full max-w-md">
    <div class="text-center mb-8">
      <!-- Logo icon -->
      <div class="flex justify-center mb-4">
        <svg
          height="40"
          viewBox="0 0 75 65"
          fill="white"
          xmlns="http://www.w3.org/2000/svg"
          class="h-10 w-auto"
        >
          <path d="M37.59.25l36.95 64H.64l36.95-64z" />
        </svg>
      </div>
      <h1 class="text-2xl font-bold text-white mb-2">
        {mode === "login" ? "Welcome back" : "Create an account"}
      </h1>
      <p class="text-accents-5">
        {mode === "login"
          ? "Enter your details to sign in."
          : "Enter your details to register."}
      </p>
    </div>

    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
      class="space-y-4"
    >
      {#if mode === "register"}
        <div>
          <label
            for="username"
            class="block text-sm font-medium text-accents-5 mb-1"
          >
            Username
          </label>
          <Input
            id="username"
            type="text"
            bind:value={username}
            required
            minlength="3"
            maxlength="50"
            placeholder="Username"
          />
        </div>
      {/if}

      <div>
        <label
          for="email"
          class="block text-sm font-medium text-accents-5 mb-1"
        >
          Email
        </label>
        <Input
          id="email"
          type="email"
          bind:value={email}
          required
          placeholder="email@example.com"
        />
      </div>

      <div>
        <label
          for="password"
          class="block text-sm font-medium text-accents-5 mb-1"
        >
          Password
        </label>
        <Input
          id="password"
          type="password"
          bind:value={password}
          required
          minlength="8"
          placeholder="••••••••"
        />
      </div>

      {#if error}
        <div
          class="bg-red-900/20 border border-red-900/50 text-red-500 px-4 py-3 rounded-lg text-sm"
        >
          {error}
        </div>
      {/if}

      <Button type="submit" disabled={loading} class="w-full">
        {#if loading}
          Processing...
        {:else}
          {mode === "login" ? "Sign In" : "Sign Up"}
        {/if}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <p class="text-accents-5 text-sm">
        {mode === "login"
          ? "Don't have an account?"
          : "Already have an account?"}
        <button
          onclick={toggleMode}
          class="text-white hover:underline font-medium ml-1 bg-transparent border-none cursor-pointer"
        >
          {mode === "login" ? "Sign Up" : "Sign In"}
        </button>
      </p>
    </div>
  </Card>
</div>
