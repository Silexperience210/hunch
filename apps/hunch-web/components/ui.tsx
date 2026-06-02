// Hunch design-system primitives. Presentational only (no hooks, no "use client"): they carry the
// dark/orange/monospace look so pages stop repeating inline `style={{...}}`. Dependency-free — no
// shadcn/Radix (CLAUDE.md: readable, forkable, no heavy UI deps on the critical path).

import type {
  ButtonHTMLAttributes,
  CSSProperties,
  InputHTMLAttributes,
  ReactNode,
  SelectHTMLAttributes,
  TextareaHTMLAttributes,
} from "react";

/** Shared "input/secondary surface" style (card bg + border). Kept as an export for the rare spot
 *  that doesn't warrant a component. Prefer the components below. */
export const field: CSSProperties = {
  background: "var(--card)",
  border: "1px solid var(--border)",
  color: "var(--fg)",
};

const SIZES = {
  sm: "px-3 py-1 text-xs",
  md: "px-4 py-2 text-sm",
  lg: "px-4 py-3 text-sm",
} as const;

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary";
  size?: keyof typeof SIZES;
};

/** Action button. `primary` = bitcoin-orange CTA, `secondary` = bordered card surface. */
export function Button({ variant = "secondary", size = "md", className = "", style, ...props }: ButtonProps) {
  const base = "rounded font-bold disabled:opacity-50 disabled:cursor-not-allowed";
  const skin: CSSProperties = variant === "primary" ? { background: "var(--accent)", color: "#000" } : field;
  return <button className={`${base} ${SIZES[size]} ${className}`} style={{ ...skin, ...style }} {...props} />;
}

/** Text input on the card surface; the `.field` class gives it an accent focus border. */
export function Input({ className = "", ...props }: InputHTMLAttributes<HTMLInputElement>) {
  return <input className={`field px-3 py-2 text-sm rounded ${className}`} {...props} />;
}

/** Multi-line text input on the card surface. */
export function Textarea({ className = "", ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <textarea className={`field px-3 py-2 text-sm rounded ${className}`} {...props} />;
}

/** Select on the card surface. */
export function Select({ className = "", children, ...props }: SelectHTMLAttributes<HTMLSelectElement>) {
  return (
    <select className={`field px-2 py-2 text-sm rounded ${className}`} {...props}>
      {children}
    </select>
  );
}

/** Bordered container. `accent` tints the border orange (used for the settlement banner). */
export function Card({
  children,
  className = "",
  accent = false,
  style,
}: {
  children: ReactNode;
  className?: string;
  accent?: boolean;
  style?: CSSProperties;
}) {
  return (
    <div className={`rounded ${className}`} style={{ border: `1px solid ${accent ? "var(--accent)" : "var(--border)"}`, ...style }}>
      {children}
    </div>
  );
}

/** Coloured status line: ok = orange, error = red, info = muted. */
export function Alert({ kind, children, className = "" }: { kind: "info" | "ok" | "error"; children: ReactNode; className?: string }) {
  const color = kind === "error" ? "var(--error)" : kind === "ok" ? "var(--accent)" : "var(--muted)";
  const border = kind === "info" ? "var(--border)" : color;
  return (
    <p className={`text-xs break-all rounded px-3 py-2 ${className}`} style={{ border: `1px solid ${border}`, color }}>
      {children}
    </p>
  );
}
