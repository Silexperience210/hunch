// Robust clipboard copy. Prefers the async Clipboard API (needs a secure context + user gesture),
// and falls back to a hidden <textarea> + execCommand so the copy button works even where
// navigator.clipboard is unavailable or blocked.

export async function copyText(text: string): Promise<void> {
  try {
    if (typeof navigator !== "undefined" && navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text);
      return;
    }
  } catch {
    /* fall through to the legacy path */
  }

  const ta = document.createElement("textarea");
  ta.value = text;
  ta.setAttribute("readonly", "");
  ta.style.position = "fixed";
  ta.style.top = "-1000px";
  ta.style.opacity = "0";
  document.body.appendChild(ta);
  ta.focus();
  ta.select();
  try {
    if (!document.execCommand("copy")) throw new Error("copy command rejected");
  } finally {
    document.body.removeChild(ta);
  }
}
