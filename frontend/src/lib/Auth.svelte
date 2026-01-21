<script lang="ts">
  import { authApi } from '$lib/api/auth';
  import { authStore } from '../stores/auth';
  import { ApiError } from '$lib/api/client';
  
  let mode: 'login' | 'register' = 'login';
  let email = '';
  let password = '';
  let username = '';
  let error = '';
  let loading = false;

  async function handleSubmit() {
    error = '';
    loading = true;

    try {
      if (mode === 'register') {
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
        error = 'エラーが発生しました';
      }
    } finally {
      loading = false;
    }
  }

  function toggleMode() {
    mode = mode === 'login' ? 'register' : 'login';
    error = '';
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 p-4">
  <div class="bg-white rounded-2xl shadow-xl w-full max-w-md p-8">
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold text-gray-900 mb-2">
        {mode === 'login' ? 'ログイン' : 'ユーザー登録'}
      </h1>
      <p class="text-gray-600">
        {mode === 'login' ? 'ポケモン辞書へようこそ' : '新しいアカウントを作成'}
      </p>
    </div>

    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      {#if mode === 'register'}
        <div>
          <label for="username" class="block text-sm font-medium text-gray-700 mb-1">
            ユーザー名
          </label>
          <input
            id="username"
            type="text"
            bind:value={username}
            required
            minlength="3"
            maxlength="50"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
            placeholder="ユーザー名を入力"
          />
        </div>
      {/if}

      <div>
        <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
          メールアドレス
        </label>
        <input
          id="email"
          type="email"
          bind:value={email}
          required
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
          placeholder="email@example.com"
        />
      </div>

      <div>
        <label for="password" class="block text-sm font-medium text-gray-700 mb-1">
          パスワード
        </label>
        <input
          id="password"
          type="password"
          bind:value={password}
          required
          minlength="8"
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
          placeholder="8文字以上のパスワード"
        />
      </div>

      {#if error}
        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
          {error}
        </div>
      {/if}

      <button
        type="submit"
        disabled={loading}
        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-4 rounded-lg transition disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if loading}
          処理中...
        {:else}
          {mode === 'login' ? 'ログイン' : '登録'}
        {/if}
      </button>
    </form>

    <div class="mt-6 text-center">
      <button
        on:click={toggleMode}
        class="text-blue-600 hover:text-blue-800 font-medium transition"
      >
        {mode === 'login' ? 'アカウントを作成' : 'ログインに戻る'}
      </button>
    </div>
  </div>
</div>
