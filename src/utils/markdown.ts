import { marked, type RendererObject } from 'marked';
import hljs from 'highlight.js';

const renderer: RendererObject = {
  code({ text, lang }) {
    const language = lang && hljs.getLanguage(lang) ? lang : undefined;
    const highlighted = language
      ? hljs.highlight(text, { language }).value
      : hljs.highlightAuto(text).value;
    const languageClass = language ? ` language-${language}` : '';
    return `<pre><code class="hljs${languageClass}">${highlighted}</code></pre>`;
  },
};

marked.use({ renderer });
marked.setOptions({
  breaks: true,
  gfm: true,
});

export function renderMarkdown(content: string): string {
  try {
    const raw = marked.parse(content) as string;
    return sanitizeHtml(raw);
  } catch (err) {
    console.error('Markdown render error:', err);
    return `<p>Error rendering markdown</p>`;
  }
}

// Lightweight sanitizer to strip scripts and dangerous attributes
export function sanitizeHtml(dirty: string): string {
  const parser = new DOMParser();
  const doc = parser.parseFromString(dirty, 'text/html');

  const blockedTags = new Set(['script', 'style', 'iframe', 'object', 'embed', 'link']);

  const traverse = (node: Element | ChildNode) => {
    if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as Element;
      if (blockedTags.has(el.tagName.toLowerCase())) {
        el.remove();
        return;
      }

      // Remove on* handlers and javascript: URLs
      Array.from(el.attributes).forEach((attr) => {
        const name = attr.name.toLowerCase();
        const value = attr.value;
        if (name.startsWith('on')) {
          el.removeAttribute(attr.name);
        }
        if ((name === 'href' || name === 'src') && value.trim().toLowerCase().startsWith('javascript:')) {
          el.removeAttribute(attr.name);
        }
      });
    }

    Array.from(node.childNodes).forEach(traverse);
  };

  Array.from(doc.body.childNodes).forEach(traverse);
  return doc.body.innerHTML;
}

export function wrapSelection(
  textarea: HTMLTextAreaElement,
  before: string,
  after: string = before
) {
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const text = textarea.value;
  const selection = text.substring(start, end);

  const newText =
    text.substring(0, start) +
    before +
    selection +
    after +
    text.substring(end);

  textarea.value = newText;
  textarea.setSelectionRange(
    start + before.length,
    end + before.length
  );
  textarea.focus();

  return newText;
}

export function insertAtCursor(textarea: HTMLTextAreaElement, text: string) {
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const original = textarea.value;

  const newText =
    original.substring(0, start) + text + original.substring(end);

  textarea.value = newText;
  textarea.setSelectionRange(start + text.length, start + text.length);
  textarea.focus();

  return newText;
}

export const markdownFormats = {
  bold: { before: '**', after: '**' },
  italic: { before: '_', after: '_' },
  code: { before: '`', after: '`' },
  codeBlock: { before: '```\n', after: '\n```' },
  h1: { before: '# ', after: '' },
  h2: { before: '## ', after: '' },
  h3: { before: '### ', after: '' },
  quote: { before: '> ', after: '' },
  link: { before: '[', after: '](url)' },
  ul: { before: '- ', after: '' },
  ol: { before: '1. ', after: '' },
};
