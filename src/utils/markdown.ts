import { marked, type RendererObject } from 'marked';
import hljs from 'highlight.js';
import plantumlEncoder from 'plantuml-encoder';

// Generate a unique ID for each diagram using crypto.randomUUID with fallback
export function generateDiagramId(type: string): string {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) {
    return `${type}-${crypto.randomUUID()}`;
  }
  // Fallback for environments without crypto.randomUUID
  return `${type}-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

// Efficient HTML escaping function using string replacement
function escapeHtml(text: string): string {
  const escapeMap: Record<string, string> = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;',
  };
  return text.replace(/[&<>"']/g, (char) => escapeMap[char]);
}

// Table counter for tracking table indices in rendered content
let tableIndex = 0;

// Reset table counter before each render
export function resetTableCounter(): void {
  tableIndex = 0;
}

const renderer: RendererObject = {
  table({ header, rows }) {
    const currentIndex = tableIndex++;
    const headerHtml = `<thead><tr>${header.map(cell => `<th>${cell.text}</th>`).join('')}</tr></thead>`;
    const rowsHtml = rows.map(row => `<tr>${row.map(cell => `<td>${cell.text}</td>`).join('')}</tr>`).join('');
    return `<table class="editable-table" data-table-index="${currentIndex}">${headerHtml}<tbody>${rowsHtml}</tbody></table>`;
  },
  code({ text, lang }) {
    const normalizedLang = (lang || '').toLowerCase();

    // Handle PlantUML diagrams
    if (normalizedLang === 'plantuml') {
      try {
        const encoded = plantumlEncoder.encode(text);
        // Using public PlantUML server - consider self-hosting for privacy
        const url = `https://www.plantuml.com/plantuml/svg/${encoded}`;
        const id = generateDiagramId('plantuml');
        // Use diagram type or first meaningful line as alt text
        const lines = text.split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('@'));
        const altText = lines[0] || 'PlantUML Diagram';
        return `<div class="diagram-container plantuml-container" id="${id}">
          <img src="${url}" alt="${escapeHtml(altText)}" class="diagram-image" />
        </div>`;
      } catch (err) {
        console.error('PlantUML encoding error:', err);
        return `<pre class="diagram-error">Error rendering PlantUML diagram</pre>`;
      }
    }

    // Handle Mermaid diagrams
    if (normalizedLang === 'mermaid' || normalizedLang === 'mermiad') {
      const escapedText = escapeHtml(text);
      return `<div class="mermaid">${escapedText}</div>`;
    }

    // Handle regular code blocks with syntax highlighting
    const language = normalizedLang && hljs.getLanguage(normalizedLang) ? normalizedLang : undefined;
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
    // Reset table counter before each render
    resetTableCounter();
    const raw = marked.parse(content) as string;
    return sanitizeHtml(raw);
  } catch (err) {
    console.error('Markdown render error:', err);
    return `<p>Error rendering markdown</p>`;
  }
}

// Extract markdown tables from content with their positions
export interface TableMatch {
  markdown: string;
  startIndex: number;
  endIndex: number;
}

export function extractMarkdownTablesWithPositions(content: string): TableMatch[] {
  const tables: TableMatch[] = [];
  const lines = content.split('\n');
  let currentTable: string[] = [];
  let inTable = false;
  let currentCharIndex = 0;
  let tableStartCharIndex = 0;
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmedLine = line.trim();
    
    // Check if this line is part of a table (starts and ends with |)
    const isTableLine = trimmedLine.startsWith('|') && trimmedLine.endsWith('|');
    // Check if this is a separator line with optional alignment markers
    const isSeparatorLine = /^\|(?:\s*:?-+:?\s*\|)+$/.test(trimmedLine);
    
    if (isTableLine || isSeparatorLine) {
      if (!inTable) {
        inTable = true;
        currentTable = [];
        tableStartCharIndex = currentCharIndex;
      }
      currentTable.push(line);
    } else {
      if (inTable && currentTable.length >= 2) {
        // A valid table must have at least header + separator
        const tableMarkdown = currentTable.join('\n');
        tables.push({
          markdown: tableMarkdown,
          startIndex: tableStartCharIndex,
          endIndex: currentCharIndex - 1 // -1 to exclude the newline
        });
      }
      inTable = false;
      currentTable = [];
    }
    
    // Track character position (add 1 for newline)
    currentCharIndex += line.length + 1;
  }
  
  // Handle table at the end of content
  if (inTable && currentTable.length >= 2) {
    const tableMarkdown = currentTable.join('\n');
    tables.push({
      markdown: tableMarkdown,
      startIndex: tableStartCharIndex,
      endIndex: content.length
    });
  }
  
  return tables;
}

// Extract markdown tables from content (legacy function for backward compatibility)
export function extractMarkdownTables(content: string): string[] {
  return extractMarkdownTablesWithPositions(content).map(t => t.markdown);
}

// Replace a specific table in content by index using tracked positions
export function replaceMarkdownTable(content: string, tableIndex: number, newTable: string): string {
  const tables = extractMarkdownTablesWithPositions(content);
  if (tableIndex < 0 || tableIndex >= tables.length) {
    return content;
  }
  
  const tableMatch = tables[tableIndex];
  return content.substring(0, tableMatch.startIndex) + newTable + content.substring(tableMatch.endIndex);
}

// Lightweight sanitizer to strip scripts and dangerous attributes
export function sanitizeHtml(dirty: string): string {
  const parser = new DOMParser();
  const doc = parser.parseFromString(dirty, 'text/html');

  // Allow SVG and diagram-related elements for PlantUML and Mermaid
  const blockedTags = new Set(['script', 'style', 'iframe', 'object', 'embed', 'link']);

  const traverse = (node: Element | ChildNode) => {
    if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as Element;
      const tagName = el.tagName.toLowerCase();
      
      if (blockedTags.has(tagName)) {
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
