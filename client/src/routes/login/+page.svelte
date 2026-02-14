<!-- src/routes/login/+page.svelte (SvelteKit, SSR disabled, TypeScript, Svelte 5) -->
<script lang="ts">
  type Touched = {
    username: boolean;
    password: boolean;
  };

  type SubmitStatus = "idle" | "submitting" | "success" | "error";
  type SubmitState = { status: SubmitStatus; message: string };

  type FieldErrors = Partial<Record<keyof Touched, string>>;

  let username = $state<string>("");
  let password = $state<string>("");

  let showPassword = $state<boolean>(false);

  let touched = $state<Touched>({
    username: false,
    password: false,
  });

  let submitState = $state<SubmitState>({ status: "idle", message: "" });

  const errors = $derived.by<FieldErrors>(() => {
    const e: FieldErrors = {};

    if (touched.username && username.trim().length < 3) {
      e.username = "Username must be at least 3 characters.";
    }

    if (touched.password && password.length < 1) {
      e.password = "Password is required.";
    }

    return e;
  });

  const isValid = $derived.by<boolean>(() => {
    return username.trim().length >= 3 && password.length > 0;
  });

  function onBlur(field: keyof Touched): void {
    touched[field] = true;
  }

  function markAllTouched(): void {
    touched.username = true;
    touched.password = true;
  }

  async function safeJson<T>(res: Response): Promise<T | null> {
    try {
      return (await res.json()) as T;
    } catch {
      return null;
    }
  }

  async function onSubmit(e: SubmitEvent): Promise<void> {
    e.preventDefault();
    markAllTouched();
    submitState = { status: "idle", message: "" };

    if (!isValid) return;

    submitState = { status: "submitting", message: "" };

    try {
      type ApiError = { message?: string };

      const res = await fetch("/api/login", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          username: username.trim(),
          password,
        }),
      });

      if (!res.ok) {
        const data = await safeJson<ApiError>(res);
        throw new Error(data?.message || "Login failed. Check your credentials and try again.");
      }

      submitState = { status: "success", message: "Welcome back! Redirecting…" };

      // Optional: redirect after success (client-only is fine with ssr=false)
      // You can swap this for `goto("/dashboard")` if using $app/navigation.
      setTimeout(() => {
        window.location.href = "/"; // change to your post-login route
      }, 400);
    } catch (err) {
      const message = err instanceof Error ? err.message : "Something went wrong.";
      submitState = { status: "error", message };
    }
  }
</script>

<svelte:head>
  <title>Log in</title>
</svelte:head>

<div class="min-h-screen bg-bg text-fg">
  <div class="mx-auto flex min-h-screen max-w-6xl items-center px-4 py-10">
    <div class="grid w-full grid-cols-1 gap-8 lg:grid-cols-2">
      <!-- Left: Brand panel -->
      <section class="hidden lg:flex">
        <div class="w-full rounded-lg border border-border bg-surface p-10 shadow-sm">
          <div class="inline-flex items-center gap-2 rounded-md border border-border bg-bg px-3 py-1 text-sm text-muted-fg">
            <span class="h-2 w-2 rounded-full bg-primary"></span>
            Log in
          </div>

          <h1 class="mt-6 text-4xl font-semibold tracking-tight text-surface-fg">Welcome back</h1>
          <p class="mt-4 text-base leading-relaxed text-muted-fg">
            Sign in with your username and password. This UI uses your token-based Tailwind theme and supports light/dark
            mode via the <code class="rounded bg-bg px-1 py-0.5">.dark</code> class.
          </p>

          <div class="mt-8 grid gap-4">
            <div class="rounded-md border border-border bg-bg p-4">
              <p class="text-sm font-medium text-fg">Tip</p>
              <p class="mt-2 text-sm text-muted-fg">
                Use <code class="rounded bg-surface px-1 py-0.5">bg-bg</code>, <code class="rounded bg-surface px-1 py-0.5">text-fg</code>,
                and <code class="rounded bg-surface px-1 py-0.5">border-border</code> consistently for effortless theming.
              </p>
            </div>
          </div>
        </div>
      </section>

      <!-- Right: Form -->
      <section class="flex items-center">
        <div class="w-full rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
          <div class="flex items-start justify-between gap-4">
            <div>
              <h2 class="text-2xl font-semibold tracking-tight text-surface-fg">Log in</h2>
              <p class="mt-1 text-sm text-muted-fg">Enter your credentials to continue.</p>
            </div>

            <div class="hidden sm:flex items-center gap-2 rounded-md border border-border bg-bg px-3 py-1 text-xs text-muted-fg">
              <span class="h-2 w-2 rounded-full bg-success"></span>
              Theme tokens
            </div>
          </div>

          {#if submitState.status === "success"}
            <div class="mt-5 rounded-md border border-border bg-bg p-4">
              <p class="text-sm font-medium text-fg">Success</p>
              <p class="mt-1 text-sm text-muted-fg">{submitState.message}</p>
              <a
                class="mt-3 inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-fg hover:opacity-95"
                href="/"
              >
                Continue
              </a>
            </div>
          {:else}
            {#if submitState.status === "error"}
              <div class="mt-5 rounded-md border border-border bg-bg p-4">
                <p class="text-sm font-medium text-danger">Couldn’t log you in</p>
                <p class="mt-1 text-sm text-muted-fg">{submitState.message}</p>
              </div>
            {/if}

            <form class="mt-6 space-y-4" onsubmit={onSubmit} novalidate>
              <!-- Username -->
              <div class="space-y-1.5">
                <label class="text-sm font-medium text-surface-fg" for="username">Username</label>
                <input
                  id="username"
                  class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-fg placeholder:text-muted-fg
                         outline-none ring-1 ring-transparent focus:ring-2 focus:ring-ring"
                  placeholder="yourname"
                  autocomplete="username"
                  bind:value={username}
                  onblur={() => onBlur("username")}
                />
                {#if errors.username}
                  <p class="text-xs text-danger">{errors.username}</p>
                {/if}
              </div>

              <!-- Password -->
              <div class="space-y-1.5">
                <label class="text-sm font-medium text-surface-fg" for="password">Password</label>

                <div class="flex overflow-hidden rounded-md border border-border bg-bg ring-1 ring-transparent focus-within:ring-2 focus-within:ring-ring">
                  <input
                    id="password"
                    type={showPassword ? "text" : "password"}
                    class="w-full bg-transparent px-3 py-2 text-sm text-fg placeholder:text-muted-fg outline-none"
                    placeholder="Your password"
                    autocomplete="current-password"
                    bind:value={password}
                    onblur={() => onBlur("password")}
                  />
                  <button
                    type="button"
                    class="border-l border-border px-3 text-xs text-muted-fg hover:text-fg"
                    onclick={() => (showPassword = !showPassword)}
                    aria-label={showPassword ? "Hide password" : "Show password"}
                  >
                    {showPassword ? "Hide" : "Show"}
                  </button>
                </div>

                {#if errors.password}
                  <p class="text-xs text-danger">{errors.password}</p>
                {/if}
              </div>

              <!-- Submit -->
              <button
                type="submit"
                class="mt-2 inline-flex w-full items-center justify-center rounded-md bg-primary px-4 py-2.5 text-sm font-medium
                       text-primary-fg shadow-sm ring-1 ring-transparent hover:opacity-95 focus:outline-none focus:ring-2 focus:ring-ring
                       disabled:cursor-not-allowed disabled:opacity-60"
                disabled={!isValid || submitState.status === "submitting"}
              >
                {#if submitState.status === "submitting"}
                  Signing in…
                {:else}
                  Log in
                {/if}
              </button>

              <div class="flex items-center justify-between text-sm">
                <a class="text-primary underline-offset-4 hover:underline" href="/forgot-password">Forgot password?</a>
                <a class="text-primary underline-offset-4 hover:underline" href="/signup">Create account</a>
              </div>
            </form>
          {/if}
        </div>
      </section>
    </div>
  </div>
</div>
