import mammoth from 'mammoth';

export interface ImportResult {
  title: string;
  content: string;
  error?: string;
}

export function useDocumentImport() {
  async function importDocx(file: File): Promise<ImportResult> {
    try {
      const arrayBuffer = await file.arrayBuffer();
      const result = await mammoth.convertToHtml({ arrayBuffer });
      
      // Extract title from filename
      const title = file.name.replace(/\.docx?$/i, '');
      
      // Convert HTML to basic markdown
      let content = result.value || '';
      content = content
        .replace(/<h1[^>]*>(.*?)<\/h1>/gi, '# $1\n')
        .replace(/<h2[^>]*>(.*?)<\/h2>/gi, '## $1\n')
        .replace(/<h3[^>]*>(.*?)<\/h3>/gi, '### $1\n')
        .replace(/<strong[^>]*>(.*?)<\/strong>/gi, '**$1**')
        .replace(/<em[^>]*>(.*?)<\/em>/gi, '*$1*')
        .replace(/<p[^>]*>(.*?)<\/p>/gi, '$1\n\n')
        .replace(/<br\s*\/?>/gi, '\n')
        .replace(/<[^>]*>/g, ''); // Remove remaining HTML tags
      
      // Add any conversion warnings as comments
      if (result.messages.length > 0) {
        const warnings = result.messages
          .filter((m: any) => m.type === 'warning')
          .map((m: any) => `<!-- Warning: ${m.message} -->`)
          .join('\n');
        if (warnings) {
          content = warnings + '\n\n' + content;
        }
      }
      
      return { title, content };
    } catch (error) {
      return {
        title: '',
        content: '',
        error: `Failed to import Word document: ${error}`
      };
    }
  }

  async function importDocument(file: File): Promise<ImportResult> {
    const ext = file.name.toLowerCase();
    
    if (ext.endsWith('.docx') || ext.endsWith('.doc')) {
      return importDocx(file);
    } else {
      return {
        title: '',
        content: '',
        error: `Unsupported file type. Currently only Word documents (.docx, .doc) are supported.`
      };
    }
  }

  return {
    importDocument,
    importDocx
  };
}
