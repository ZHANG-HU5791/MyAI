import MarkdownIt from "markdown-it";
import DOMPurify from "dompurify";

const md = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: true,
});

/**
 * Incremental markdown parser for streaming.
 * Handles unclosed code blocks and partial markdown safely.
 */
export class MarkdownStreamParser {
  private buffer = "";

  append(chunk: string): string {
    this.buffer += chunk;
    return this.renderSafe();
  }

  reset(): void {
    this.buffer = "";
  }

  private renderSafe(): string {
    let text = this.buffer;

    // Count unclosed code fences
    const fenceCount = (text.match(/```/g) || []).length;
    if (fenceCount % 2 !== 0) {
      text += "\n```"; // Temporarily close for rendering
    }

    // Render through markdown-it, then sanitize via DOMPurify
    const rawHtml = md.render(text);
    return DOMPurify.sanitize(rawHtml);
  }
}

/**
 * One-shot markdown render (non-streaming).
 */
export function renderMarkdown(text: string): string {
  const rawHtml = md.render(text);
  return DOMPurify.sanitize(rawHtml);
}
