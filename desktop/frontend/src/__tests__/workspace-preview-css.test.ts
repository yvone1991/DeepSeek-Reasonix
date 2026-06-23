// Run: tsx src/__tests__/workspace-preview-css.test.ts

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testDir = dirname(fileURLToPath(import.meta.url));
const styles = readFileSync(resolve(testDir, "../styles.css"), "utf8");

let passed = 0;
let failed = 0;

function eq(a: unknown, b: unknown, label: string) {
  if (a === b) {
    process.stdout.write(`  PASS  ${label}\n`);
    passed += 1;
  } else {
    process.stdout.write(`  FAIL  ${label}: expected ${JSON.stringify(b)}, got ${JSON.stringify(a)}\n`);
    failed += 1;
  }
}

function matchingBlocks(selector: string): string[] {
  const blocks: string[] = [];
  const rule = /([^{}]+)\{([^{}]*)\}/g;
  let match: RegExpExecArray | null;
  while ((match = rule.exec(styles)) !== null) {
    const selectors = match[1].split(",").map((part) => part.trim());
    if (selectors.includes(selector)) blocks.push(match[2]);
  }
  return blocks;
}

function finalDeclaration(selector: string, property: string): string | undefined {
  let value: string | undefined;
  for (const block of matchingBlocks(selector)) {
    const declaration = new RegExp(`(?:^|;)\\s*${property}\\s*:\\s*([^;]+)`, "g");
    let match: RegExpExecArray | null;
    while ((match = declaration.exec(block)) !== null) {
      value = match[1].trim();
    }
  }
  return value;
}

console.log("\nworkspace preview css");

eq(finalDeclaration(".workspace-preview__body--code", "overflow"), "hidden", "code preview body does not create a nested scroller");
eq(finalDeclaration(".workspace-preview__body--code", "display"), "flex", "code preview body hosts an editor-like viewport");
eq(finalDeclaration(".workspace-preview__body--code .code-block", "display"), "flex", "code block fills the preview viewport");
eq(finalDeclaration(".workspace-preview__body--code .code", "overflow"), "auto", "code viewport owns horizontal and vertical scrolling");
eq(finalDeclaration(".workspace-preview__body--code .code", "min-height"), "0", "code viewport can shrink inside the preview pane");
eq(finalDeclaration(".workspace-preview__body--code .code", "margin"), "0", "code viewport scrollbar sits at the visible pane bottom");

console.log(`\n${passed} passed, ${failed} failed`);
if (failed > 0) process.exit(1);
