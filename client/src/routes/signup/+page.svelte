<script lang="ts">
  type Touched = {
    username: boolean;
    email: boolean;
    password: boolean;
    confirmPassword: boolean;
    birthday: boolean;
    acceptTos: boolean;
  };

  type SubmitStatus = "idle" | "submitting" | "success" | "error";
  type SubmitState = { status: SubmitStatus; message: string };

  type FieldErrors = Partial<Record<keyof Touched, string>>;

  let username = $state<string>("");
  let email = $state<string>("");
  let password = $state<string>("");
  let confirmPassword = $state<string>("");
  let birthday = $state<string>(""); // yyyy-mm-dd
  let acceptTos = $state<boolean>(false);

  let showPassword = $state<boolean>(false);
  let showConfirm = $state<boolean>(false);

  let touched = $state<Touched>({
    username: false,
    email: false,
    password: false,
    confirmPassword: false,
    birthday: false,
    acceptTos: false,
  });

  let submitState = $state<SubmitState>({ status: "idle", message: "" });

  const emailOk = (v: string): boolean =>
    /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v.trim());

  const MIN_AGE = 13;

  function ageFromDate(dateStr: string): number {
    const today = new Date();
    const dob = new Date(dateStr);
    let age = today.getFullYear() - dob.getFullYear();
    const m = today.getMonth() - dob.getMonth();
    if (m < 0 || (m === 0 && today.getDate() < dob.getDate())) age--;
    return age;
  }

  const errors = $derived.by<FieldErrors>(() => {
    const e: FieldErrors = {};

    if (touched.username && username.trim().length < 3) {
      e.username = "Username must be at least 3 characters.";
    }

    if (touched.email && !emailOk(email)) {
      e.email = "Please enter a valid email address.";
    }

    if (touched.password && password.length < 8) {
      e.password = "Password must be at least 8 characters.";
    }

    if (touched.confirmPassword && confirmPassword !== password) {
      e.confirmPassword = "Passwords do not match.";
    }

    if (touched.birthday) {
      if (!birthday) {
        e.birthday = "Birthday is required.";
      } else if (ageFromDate(birthday) < MIN_AGE) {
        e.birthday = `You must be at least ${MIN_AGE} years old.`;
      }
    }

    if (touched.acceptTos && !acceptTos) {
      e.acceptTos = "You must accept the Terms of Service.";
    }

    return e;
  });

  const isValid = $derived.by<boolean>(() => {
    return (
      username.trim().length >= 3 &&
      emailOk(email) &&
      password.length >= 8 &&
      confirmPassword === password &&
      !!birthday &&
      ageFromDate(birthday) >= MIN_AGE &&
      acceptTos
    );
  });

  function onBlur(field: keyof Touched): void {
    touched[field] = true;
  }

  function markAllTouched(): void {
    Object.keys(touched).forEach((k) => {
      touched[k as keyof Touched] = true;
    });
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

      const res = await fetch("/api/signup", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          username: username.trim(),
          email: email.trim(),
          password,
          birthday,
          acceptTos,
        }),
      });

      if (!res.ok) {
        const data = await safeJson<ApiError>(res);
        throw new Error(data?.message || "Signup failed.");
      }

      submitState = {
        status: "success",
        message: "Account created! You can now log in.",
      };

      username = "";
      email = "";
      password = "";
      confirmPassword = "";
      birthday = "";
      acceptTos = false;

      Object.keys(touched).forEach((k) => {
        touched[k as keyof Touched] = false;
      });
    } catch (err) {
      submitState = {
        status: "error",
        message: err instanceof Error ? err.message : "Something went wrong.",
      };
    }
  }
</script>

<svelte:head>
  <title>Sign up</title>
</svelte:head>

<div class="min-h-screen bg-bg text-fg">
  <div class="mx-auto flex min-h-screen max-w-md items-center px-4">
    <div class="w-full rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
      <h1 class="text-2xl font-semibold tracking-tight text-surface-fg">
        Create account
      </h1>
      <p class="mt-1 text-sm text-muted-fg">
        All fields are required.
      </p>

      {#if submitState.status === "error"}
        <div class="mt-4 rounded-md border border-border bg-bg p-3">
          <p class="text-sm text-danger">{submitState.message}</p>
        </div>
      {/if}

      {#if submitState.status === "success"}
        <div class="mt-4 rounded-md border border-border bg-bg p-3">
          <p class="text-sm text-success">{submitState.message}</p>
          <a
            href="/login"
            class="mt-3 inline-flex rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-fg"
          >
            Go to login
          </a>
        </div>
      {:else}
        <form class="mt-6 space-y-4" onsubmit={onSubmit} novalidate>
          <!-- Username -->
          <div>
            <label class="text-sm font-medium text-surface-fg">Username</label>
            <input
              class="mt-1 w-full rounded-md border border-border bg-bg px-3 py-2 text-sm
                     ring-1 ring-transparent focus:ring-2 focus:ring-ring"
              bind:value={username}
              onblur={() => onBlur("username")}
            />
            {#if errors.username}
              <p class="mt-1 text-xs text-danger">{errors.username}</p>
            {/if}
          </div>

          <!-- Email -->
          <div>
            <label class="text-sm font-medium text-surface-fg">Email</label>
            <input
              type="email"
              class="mt-1 w-full rounded-md border border-border bg-bg px-3 py-2 text-sm
                     ring-1 ring-transparent focus:ring-2 focus:ring-ring"
              bind:value={email}
              onblur={() => onBlur("email")}
            />
            {#if errors.email}
              <p class="mt-1 text-xs text-danger">{errors.email}</p>
            {/if}
          </div>

          <!-- Birthday -->
          <div>
            <label class="text-sm font-medium text-surface-fg">Birthday</label>
            <input
              type="date"
              class="mt-1 w-full rounded-md border border-border bg-bg px-3 py-2 text-sm
                     ring-1 ring-transparent focus:ring-2 focus:ring-ring"
              bind:value={birthday}
              onblur={() => onBlur("birthday")}
            />
            {#if errors.birthday}
              <p class="mt-1 text-xs text-danger">{errors.birthday}</p>
            {/if}
          </div>

          <!-- Password -->
          <div>
            <label class="text-sm font-medium text-surface-fg">Password</label>
            <input
              type="password"
              class="mt-1 w-full rounded-md border border-border bg-bg px-3 py-2 text-sm
                     ring-1 ring-transparent focus:ring-2 focus:ring-ring"
              bind:value={password}
              onblur={() => onBlur("password")}
            />
            {#if errors.password}
              <p class="mt-1 text-xs text-danger">{errors.password}</p>
            {/if}
          </div>

          <!-- Confirm Password -->
          <div>
            <label class="text-sm font-medium text-surface-fg">
              Confirm password
            </label>
            <input
              type="password"
              class="mt-1 w-full rounded-md border border-border bg-bg px-3 py-2 text-sm
                     ring-1 ring-transparent focus:ring-2 focus:ring-ring"
              bind:value={confirmPassword}
              onblur={() => onBlur("confirmPassword")}
            />
            {#if errors.confirmPassword}
              <p class="mt-1 text-xs text-danger">{errors.confirmPassword}</p>
            {/if}
          </div>

          <!-- Terms -->
          <label class="flex items-start gap-3 rounded-md border border-border bg-bg p-3">
            <input
              type="checkbox"
              class="mt-1 h-4 w-4 accent-primary"
              bind:checked={acceptTos}
              onchange={() => (touched.acceptTos = true)}
            />
            <span class="text-sm text-fg">
              I agree to the
              <a href="/terms" class="text-primary hover:underline">
                Terms of Service
              </a>
            </span>
          </label>
          {#if errors.acceptTos}
            <p class="text-xs text-danger">{errors.acceptTos}</p>
          {/if}

          <button
            type="submit"
            class="mt-2 w-full rounded-md bg-primary px-4 py-2.5 text-sm font-medium
                   text-primary-fg hover:opacity-95 disabled:opacity-60"
            disabled={!isValid || submitState.status === "submitting"}
          >
            {submitState.status === "submitting" ? "Creating account…" : "Create account"}
          </button>
        </form>
      {/if}
    </div>
  </div>
</div>
