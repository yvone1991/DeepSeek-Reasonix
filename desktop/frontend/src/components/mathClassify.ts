export function isLikelyInlineMath(math: string): boolean {
  if (!math || math !== math.trim() || math.includes("\n")) return false;
  if (math.includes("://") || math.includes("](")) return false;
  // Pure-digit/decimal/percentage strings (multi-digit, with optional decimal
  // or %) are too often currency / percentages in prose to risk classifying
  // as math. Single-digit strings ($1$, $2$…) are exempted below — they are
  // commonly math indices in physics / linear-algebra prose.
  if (/^\$?\d{2,}(?:\.\d+)?%?$/.test(math)) return false;

  if (/\\[A-Za-z]+\b/.test(math)) return true;
  if (/[\^_{}|]/.test(math)) return true;
  if (/\b(?:alpha|beta|gamma|sum|int|prod|lim|infty|sqrt|frac|sin|cos|tan|log|ln|max|min|partial|nabla|left|right)\b/.test(math)) return true;
  if (/^[A-Za-z]\s*\([^)]{1,80}\)$/.test(math)) return true;
  if (/[A-Za-z0-9)\]}]\s*[+\-*/=<>]\s*[A-Za-z0-9([{\\]/.test(math)) return true;

  // One-sided comparison operators: expression starts or ends with a
  // comparison operator and has an operand on the other side. Covers
  // cases like "< B" (less than B, with the left operand implicit in
  // surrounding prose), "> 0", "<= 5", "B <", etc. The existing operator
  // rule above requires operands on both sides; this rule relaxes that
  // for the common LLM pattern where the comparison is against an
  // implicit value.
  if (/^(?:<=?|>=?|≠|≤|≥)\s*[A-Za-z0-9]|[A-Za-z0-9]\s*(?:<=?|>=?|≠|≤|≥)$/.test(math)) return true;

  // Comma-separated single tokens (letter, digit, or LaTeX command) —
  // typical of math enumeration, ordered-pair / tuple notation, or matrix
  // element lists. Optionally wrapped in matching parens. Examples:
  //   "A, B, C"  (a set or list of variables)
  //   "1, 2, 3"  (a sequence)
  //   "x, y, z"  (coordinates)
  //   "a, b"     (a pair)
  //   "\\alpha, \\beta"  (Greek pair)
  //   "(A, B)"   (ordered pair, when normalizeMath has stripped outer parens)
  // Currency and env-var usage almost never looks like this.
  if (/^\(?(?:[A-Za-z0-9]|\\[A-Za-z]+)(?:\s*,\s*(?:[A-Za-z0-9]|\\[A-Za-z]+)){1,10}\)?$/.test(math)) return true;

  if (/[A-Za-z]\s+[A-Za-z]/.test(math)) return false;
  if (/^[A-Z][A-Z0-9_]{1,}$/.test(math)) return false;
  if (/^v\d+(?:\.\d+)*$/i.test(math)) return false;
  if (/^[A-Za-z]{2,}$/.test(math)) return false;

  // Single digit (e.g. "$1$", "$2$") is commonly a math index in physics,
  // linear-algebra, and set-notation prose. Currency without a closing "$"
  // is the much more common English-prose form, so the asymmetry is safe.
  if (/^\d$/.test(math)) return true;

  // Single letter (uppercase or lowercase) is allowed as a math token.
  // Lowercase letters (a, b, x, y, z, …) are the most common math
  // variables. Single *uppercase* letters (S, A, G, V, R, Z, …) are
  // very common math names in non-English math prose (sets, algebras,
  // groups, vector spaces) and the closing-dollar form $X$ is essentially
  // never written for the English word I/A by hand. Both single-letter
  // shapes are therefore safe to classify as math.
  return /^[A-Za-z]$/.test(math);
}
