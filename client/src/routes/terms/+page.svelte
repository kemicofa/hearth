<!-- src/routes/terms/+page.svelte (SvelteKit, SSR disabled, TypeScript, Svelte 5) -->
<script lang="ts">
  const lastUpdated = "February 14, 2026";

  // You can set your product/company name here (or replace with env/config).
  const productName = "YourApp";
  const supportEmail = "support@yourapp.example";
</script>

<svelte:head>
  <title>Terms of Service</title>
  <meta
    name="description"
    content="Terms of Service for a chat and community platform."
  />
</svelte:head>

<div class="min-h-screen bg-bg text-fg">
  <div class="mx-auto max-w-3xl px-4 py-10 sm:py-14">
    <header class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
      <div class="inline-flex items-center gap-2 rounded-md border border-border bg-bg px-3 py-1 text-xs text-muted-fg">
        <span class="h-2 w-2 rounded-full bg-primary"></span>
        Legal
      </div>

      <h1 class="mt-4 text-3xl font-semibold tracking-tight text-surface-fg">
        Terms of Service
      </h1>
      <p class="mt-2 text-sm text-muted-fg">
        Last updated: <span class="text-fg">{lastUpdated}</span>
      </p>

      <p class="mt-4 text-sm leading-relaxed text-muted-fg">
        These Terms of Service (“Terms”) govern your access to and use of <span class="text-fg">{productName}</span>, a
        community chat platform similar to a Discord-style service (“Service”). By creating an account, accessing, or
        using the Service, you agree to these Terms.
      </p>

      <div class="mt-5 rounded-md border border-border bg-bg p-4">
        <p class="text-sm font-medium text-fg">Privacy promise</p>
        <p class="mt-1 text-sm text-muted-fg">
          We do <span class="text-fg">not</span> sell your personal data and we do <span class="text-fg">not</span> share
          your personal data with third parties for marketing or advertising. We only use service providers when needed
          to operate the Service (for example, hosting and security) and they are bound to protect your data.
        </p>
      </div>
    </header>

    <main class="mt-8 space-y-6">
      <!-- Quick summary -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">Quick summary</h2>
        <ul class="mt-4 space-y-2 text-sm text-muted-fg">
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            Be respectful. Don’t harass, threaten, or share illegal content.
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            You’re responsible for what you post and for keeping your account secure.
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            We don’t share your data with third parties for marketing.
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            We may moderate content and enforce rules to keep communities safe.
          </li>
        </ul>
      </section>

      <!-- 1. Eligibility -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">1) Eligibility</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          You must be at least the minimum age required by the laws of your country to use the Service. If you are under
          the age of majority where you live, you may use the Service only with consent of a parent or legal guardian.
        </p>
      </section>

      <!-- 2. Account -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">2) Your account</h2>
        <ul class="mt-3 space-y-3 text-sm text-muted-fg">
          <li>
            <span class="text-fg">Account information.</span> You agree to provide accurate information and keep it
            up-to-date.
          </li>
          <li>
            <span class="text-fg">Security.</span> You are responsible for maintaining the confidentiality of your login
            credentials and for all activity under your account.
          </li>
          <li>
            <span class="text-fg">Impersonation.</span> Do not impersonate others or misrepresent your affiliation with
            any person or organization.
          </li>
        </ul>
      </section>

      <!-- 3. Community guidelines -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">3) Community rules (acceptable use)</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          You may not use the Service to engage in or promote harmful, illegal, or abusive behavior. This includes (but
          is not limited to):
        </p>

        <ul class="mt-4 space-y-2 text-sm text-muted-fg">
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            Harassment, hate speech, or targeted abuse.
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            Threats, encouragement of violence, or doxxing (sharing private personal information without permission).
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            Sexual content involving minors, exploitation, or non-consensual intimate imagery.
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            Malware, phishing, scams, spam, or attempts to gain unauthorized access to accounts or systems.
          </li>
          <li class="flex items-start gap-2">
            <span class="mt-1 inline-block h-1.5 w-1.5 rounded-full bg-primary"></span>
            Infringing content (copyright/trademark), or content you don’t have rights to share.
          </li>
        </ul>

        <div class="mt-5 rounded-md border border-border bg-bg p-4">
          <p class="text-sm font-medium text-fg">Server owners & moderators</p>
          <p class="mt-1 text-sm text-muted-fg">
            Communities may have their own rules. Server owners and moderators may remove content or members consistent
            with their rules and these Terms.
          </p>
        </div>
      </section>

      <!-- 4. Content -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">4) Your content and permissions</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          You retain ownership of the content you post (“User Content”). To operate the Service, you grant
          <span class="text-fg"> {productName}</span> a limited, worldwide, non-exclusive, royalty-free license to host,
          store, reproduce, and display your User Content solely for the purpose of providing and improving the Service
          (for example, showing your messages to the people you send them to).
        </p>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          You represent that you have the rights to post your User Content and that your User Content does not violate
          laws or the rights of others.
        </p>
      </section>

      <!-- 5. Privacy / data -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">5) Privacy and data</h2>

        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          We collect and process data to run the Service—such as account information, messages you send, server
          membership, and technical data (like device and log information) for security and reliability.
        </p>

        <div class="mt-4 rounded-md border border-border bg-bg p-4">
          <p class="text-sm font-medium text-fg">No third-party sharing (marketing)</p>
          <p class="mt-1 text-sm text-muted-fg">
            We do not share your personal data with third parties for marketing or advertising purposes.
          </p>
        </div>

        <p class="mt-4 text-sm leading-relaxed text-muted-fg">
          We may use trusted service providers (for example: hosting, database, monitoring, email delivery) strictly to
          operate the Service. They are required to protect your data and only process it on our instructions.
        </p>

        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          If you delete your account, we will remove or anonymize your personal data unless we must retain it for legal
          or security reasons (e.g., fraud prevention, legal obligations).
        </p>
      </section>

      <!-- 6. Moderation -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">6) Moderation and enforcement</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          We may investigate violations of these Terms and take action, including removing content, restricting features,
          suspending, or terminating accounts. We may also cooperate with law enforcement when required by law or when
          necessary to protect users and the Service.
        </p>
      </section>

      <!-- 7. Safety -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">7) Safety and reporting</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          If you see something that violates these Terms or you believe a user is at risk of harm, please report it. We
          review reports and may take action based on severity, context, and applicable law.
        </p>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          Contact: <a class="text-primary underline-offset-4 hover:underline" href={"mailto:" + supportEmail}>{supportEmail}</a>
        </p>
      </section>

      <!-- 8. Service changes -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">8) Changes to the Service</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          We may update or change the Service over time, including adding or removing features. We may also need to
          interrupt the Service for maintenance or security. We will try to minimize disruption when feasible.
        </p>
      </section>

      <!-- 9. Termination -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">9) Termination</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          You can stop using the Service at any time. We may suspend or terminate your access if you violate these Terms
          or if required to do so for legal or security reasons. Certain provisions (like content licenses, disclaimers,
          and limitation of liability) may survive termination to the extent allowed by law.
        </p>
      </section>

      <!-- 10. Disclaimers -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">10) Disclaimers</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          The Service is provided “as is” and “as available.” To the extent permitted by law, we disclaim warranties of
          merchantability, fitness for a particular purpose, and non-infringement. We do not guarantee uninterrupted or
          error-free operation, or that content will always be available.
        </p>
      </section>

      <!-- 11. Liability -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">11) Limitation of liability</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          To the extent permitted by law, <span class="text-fg">{productName}</span> and its affiliates will not be
          liable for indirect, incidental, special, consequential, or punitive damages, or for any loss of profits,
          revenues, data, goodwill, or other intangible losses resulting from your use of the Service.
        </p>
      </section>

      <!-- 12. Governing law -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">12) Governing law</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          These Terms are governed by the laws of the jurisdiction where the Service provider is established, excluding
          conflict-of-law rules. If you are a consumer, you may also have mandatory rights under the laws of your country.
        </p>
      </section>

      <!-- 13. Changes to terms -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">13) Changes to these Terms</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          We may update these Terms from time to time. If changes are material, we will take reasonable steps to notify
          you (for example, by posting a notice in the Service). Continued use of the Service after the effective date of
          the updated Terms means you accept the updated Terms.
        </p>
      </section>

      <!-- 14. Contact -->
      <section class="rounded-lg border border-border bg-surface p-6 shadow-sm sm:p-8">
        <h2 class="text-xl font-semibold text-surface-fg">14) Contact</h2>
        <p class="mt-3 text-sm leading-relaxed text-muted-fg">
          Questions about these Terms? Contact us at{" "}
          <a class="text-primary underline-offset-4 hover:underline" href={"mailto:" + supportEmail}>
            {supportEmail}
          </a>
          .
        </p>

        <div class="mt-5 rounded-md border border-border bg-bg p-4">
          <p class="text-xs text-muted-fg">
            This template is provided for convenience and does not constitute legal advice. Consider having counsel
            review it for your jurisdiction and product specifics (payments, refunds, EU consumer terms, etc.).
          </p>
        </div>
      </section>
    </main>

    <footer class="mt-10 text-center text-xs text-muted-fg">
      © {new Date().getFullYear()} {productName}. All rights reserved.
    </footer>
  </div>
</div>
