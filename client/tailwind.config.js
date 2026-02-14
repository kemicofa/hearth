console.log("Tailwind Config Loaded.");
/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "class",
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx,html}"],
  theme: {
    extend: {
      colors: {
        // Base
        bg: "rgb(var(--bg) / <alpha-value>)",
        fg: "rgb(var(--fg) / <alpha-value>)",

        // Surfaces
        surface: "rgb(var(--surface) / <alpha-value>)",
        "surface-fg": "rgb(var(--surface-fg) / <alpha-value>)",
        popover: "rgb(var(--popover) / <alpha-value>)",
        "popover-fg": "rgb(var(--popover-fg) / <alpha-value>)",

        // Borders & focus
        border: "rgb(var(--border) / <alpha-value>)",
        ring: "rgb(var(--ring) / <alpha-value>)",

        // Brand
        primary: "rgb(var(--primary) / <alpha-value>)",
        "primary-fg": "rgb(var(--primary-fg) / <alpha-value>)",

        // Secondary / muted / accent
        secondary: "rgb(var(--secondary) / <alpha-value>)",
        "secondary-fg": "rgb(var(--secondary-fg) / <alpha-value>)",

        muted: "rgb(var(--muted) / <alpha-value>)",
        "muted-fg": "rgb(var(--muted-fg) / <alpha-value>)",

        accent: "rgb(var(--accent) / <alpha-value>)",
        "accent-fg": "rgb(var(--accent-fg) / <alpha-value>)",

        // Status
        danger: "rgb(var(--danger) / <alpha-value>)",
        "danger-fg": "rgb(var(--danger-fg) / <alpha-value>)",

        warning: "rgb(var(--warning) / <alpha-value>)",
        "warning-fg": "rgb(var(--warning-fg) / <alpha-value>)",

        success: "rgb(var(--success) / <alpha-value>)",
        "success-fg": "rgb(var(--success-fg) / <alpha-value>)",

        info: "rgb(var(--info) / <alpha-value>)",
        "info-fg": "rgb(var(--info-fg) / <alpha-value>)",
      },

      borderRadius: {
        lg: "var(--radius-lg)",
        md: "var(--radius-md)",
        sm: "var(--radius-sm)",
      },
      boxShadow: {
        sm: "0 1px 2px 0 rgb(0 0 0 / 0.06)",
        md: "0 6px 18px -8px rgb(0 0 0 / 0.22)",
      },
    },
  },
  plugins: [],
};
