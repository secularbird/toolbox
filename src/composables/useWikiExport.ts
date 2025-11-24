import { ref } from 'vue';
import jsPDF from 'jspdf';
import { toPng } from 'html-to-image';
import { Document, Packer, Paragraph, TextRun, ImageRun, HeadingLevel, AlignmentType } from 'docx';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { marked } from 'marked';
import mermaid from 'mermaid';
import plantumlEncoder from 'plantuml-encoder';

interface ExportOptions {
  title: string;
  content: string;
  format: 'pdf' | 'docx';
}

export function useWikiExport() {
  const isExporting = ref(false);
  const exportError = ref<string | null>(null);

  /**
   * Fetch PlantUML diagram as PNG data
   */
  async function fetchPlantUMLImage(code: string): Promise<string | null> {
    try {
      const encoded = plantumlEncoder.encode(code);
      const url = `https://www.plantuml.com/plantuml/png/${encoded}`;
      
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error('Failed to fetch PlantUML diagram');
      }
      
      const blob = await response.blob();
      return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onloadend = () => resolve(reader.result as string);
        reader.onerror = reject;
        reader.readAsDataURL(blob);
      });
    } catch (error) {
      console.error('Error fetching PlantUML image:', error);
      return null;
    }
  }

  /**
   * Render a Mermaid diagram to PNG
   */
  async function renderMermaidToPng(code: string, id: string): Promise<string | null> {
    try {
      // Create a temporary container
      const container = document.createElement('div');
      container.style.position = 'absolute';
      container.style.left = '-9999px';
      container.style.top = '-9999px';
      container.style.background = 'white';
      container.style.padding = '20px';
      document.body.appendChild(container);

      // Create mermaid element
      const mermaidDiv = document.createElement('div');
      mermaidDiv.className = 'mermaid';
      mermaidDiv.textContent = code;
      mermaidDiv.id = id;
      container.appendChild(mermaidDiv);

      // Render the diagram
      await mermaid.run({ nodes: [mermaidDiv] });

      // Wait a bit for rendering to complete
      await new Promise(resolve => setTimeout(resolve, 100));

      // Convert to PNG
      const dataUrl = await toPng(container, {
        backgroundColor: 'white',
        pixelRatio: 2,
      });

      // Cleanup
      document.body.removeChild(container);

      return dataUrl;
    } catch (error) {
      console.error('Error rendering Mermaid diagram:', error);
      return null;
    }
  }

  /**
   * Parse markdown and extract diagrams
   */
  async function parseMarkdownWithDiagrams(content: string) {
    const elements: Array<{
      type: 'text' | 'heading' | 'image' | 'diagram';
      level?: number;
      content?: string;
      imageData?: string;
      alt?: string;
    }> = [];

    const lines = content.split('\n');
    let i = 0;
    let codeBlockType = '';
    let codeBlockContent: string[] = [];
    let textBuffer: string[] = [];

    const flushTextBuffer = () => {
      if (textBuffer.length > 0) {
        const text = textBuffer.join('\n').trim();
        if (text) {
          // Parse headings
          const headingMatch = text.match(/^(#{1,6})\s+(.+)$/);
          if (headingMatch) {
            elements.push({
              type: 'heading',
              level: headingMatch[1].length,
              content: headingMatch[2],
            });
          } else {
            elements.push({
              type: 'text',
              content: text,
            });
          }
        }
        textBuffer = [];
      }
    };

    while (i < lines.length) {
      const line = lines[i];

      // Check for code block start
      if (line.startsWith('```')) {
        flushTextBuffer();
        
        const lang = line.slice(3).trim().toLowerCase();
        codeBlockType = lang;
        codeBlockContent = [];
        i++;

        // Collect code block content
        while (i < lines.length && !lines[i].startsWith('```')) {
          codeBlockContent.push(lines[i]);
          i++;
        }

        const code = codeBlockContent.join('\n');

        // Process diagram code blocks
        if (codeBlockType === 'mermaid') {
          const id = `mermaid-export-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
          const imageData = await renderMermaidToPng(code, id);
          if (imageData) {
            elements.push({
              type: 'diagram',
              imageData,
              alt: 'Mermaid Diagram',
            });
          }
        } else if (codeBlockType === 'plantuml') {
          const imageData = await fetchPlantUMLImage(code);
          if (imageData) {
            elements.push({
              type: 'diagram',
              imageData,
              alt: 'PlantUML Diagram',
            });
          }
        } else {
          // Regular code block - treat as text
          textBuffer.push('```' + codeBlockType);
          textBuffer.push(...codeBlockContent);
          textBuffer.push('```');
        }

        codeBlockType = '';
        codeBlockContent = [];
        i++;
      } else {
        // Check for images in markdown syntax
        const imageMatch = line.match(/!\[([^\]]*)\]\(([^)]+)\)/);
        if (imageMatch) {
          flushTextBuffer();
          
          try {
            const response = await fetch(imageMatch[2]);
            const blob = await response.blob();
            const imageData = await new Promise<string>((resolve, reject) => {
              const reader = new FileReader();
              reader.onloadend = () => resolve(reader.result as string);
              reader.onerror = reject;
              reader.readAsDataURL(blob);
            });
            
            elements.push({
              type: 'image',
              imageData,
              alt: imageMatch[1] || 'Image',
            });
          } catch (error) {
            console.error('Error loading image:', error);
            textBuffer.push(line);
          }
        } else {
          textBuffer.push(line);
        }
        
        i++;
      }
    }

    flushTextBuffer();

    return elements;
  }

  /**
   * Export to PDF
   */
  async function exportToPDF(options: ExportOptions): Promise<void> {
    try {
      isExporting.value = true;
      exportError.value = null;

      const elements = await parseMarkdownWithDiagrams(options.content);

      // Create PDF document
      const pdf = new jsPDF({
        orientation: 'portrait',
        unit: 'mm',
        format: 'a4',
      });

      const pageWidth = pdf.internal.pageSize.getWidth();
      const pageHeight = pdf.internal.pageSize.getHeight();
      const margin = 20;
      const maxWidth = pageWidth - 2 * margin;
      let yPosition = margin;

      // Add title
      pdf.setFontSize(20);
      pdf.setFont('helvetica', 'bold');
      pdf.text(options.title, margin, yPosition);
      yPosition += 15;

      // Process elements
      for (const element of elements) {
        // Check if we need a new page
        if (yPosition > pageHeight - margin - 20) {
          pdf.addPage();
          yPosition = margin;
        }

        if (element.type === 'heading') {
          const fontSize = 18 - (element.level! - 1) * 2;
          pdf.setFontSize(fontSize);
          pdf.setFont('helvetica', 'bold');
          
          const lines = pdf.splitTextToSize(element.content!, maxWidth);
          pdf.text(lines, margin, yPosition);
          yPosition += (lines.length * fontSize * 0.4) + 8;
        } else if (element.type === 'text') {
          pdf.setFontSize(11);
          pdf.setFont('helvetica', 'normal');
          
          // Remove markdown formatting for basic display
          const plainText = element.content!
            .replace(/\*\*(.+?)\*\*/g, '$1')
            .replace(/\*(.+?)\*/g, '$1')
            .replace(/`(.+?)`/g, '$1')
            .replace(/~~(.+?)~~/g, '$1');
          
          const lines = pdf.splitTextToSize(plainText, maxWidth);
          
          // Check space and add text
          for (const line of lines) {
            if (yPosition > pageHeight - margin - 10) {
              pdf.addPage();
              yPosition = margin;
            }
            pdf.text(line, margin, yPosition);
            yPosition += 6;
          }
          yPosition += 4;
        } else if (element.type === 'diagram' || element.type === 'image') {
          if (element.imageData) {
            try {
              // Calculate image dimensions
              const maxImageWidth = maxWidth;
              const maxImageHeight = 100;

              // Check if we need a new page for the image
              if (yPosition + maxImageHeight > pageHeight - margin) {
                pdf.addPage();
                yPosition = margin;
              }

              pdf.addImage(
                element.imageData,
                'PNG',
                margin,
                yPosition,
                maxImageWidth,
                maxImageHeight,
                undefined,
                'FAST'
              );
              yPosition += maxImageHeight + 10;
            } catch (error) {
              console.error('Error adding image to PDF:', error);
            }
          }
        }
      }

      // Save the PDF
      const pdfBlob = pdf.output('blob');
      const pdfArray = await pdfBlob.arrayBuffer();

      const savePath = await save({
        defaultPath: `${options.title}.pdf`,
        filters: [
          {
            name: 'PDF',
            extensions: ['pdf'],
          },
        ],
      });

      if (savePath) {
        await writeFile(savePath, new Uint8Array(pdfArray));
        console.log('PDF exported successfully:', savePath);
      }
    } catch (error) {
      console.error('Error exporting to PDF:', error);
      exportError.value = error instanceof Error ? error.message : 'Failed to export PDF';
      throw error;
    } finally {
      isExporting.value = false;
    }
  }

  /**
   * Export to Word (DOCX)
   */
  async function exportToWord(options: ExportOptions): Promise<void> {
    try {
      isExporting.value = true;
      exportError.value = null;

      const elements = await parseMarkdownWithDiagrams(options.content);

      // Create document sections
      const docElements: any[] = [];

      // Add title
      docElements.push(
        new Paragraph({
          text: options.title,
          heading: HeadingLevel.TITLE,
          spacing: { after: 400 },
        })
      );

      // Process elements
      for (const element of elements) {
        if (element.type === 'heading') {
          const headingLevels = [
            HeadingLevel.HEADING_1,
            HeadingLevel.HEADING_2,
            HeadingLevel.HEADING_3,
            HeadingLevel.HEADING_4,
            HeadingLevel.HEADING_5,
            HeadingLevel.HEADING_6,
          ];
          
          docElements.push(
            new Paragraph({
              text: element.content!,
              heading: headingLevels[Math.min(element.level! - 1, 5)],
              spacing: { before: 240, after: 120 },
            })
          );
        } else if (element.type === 'text') {
          // Remove markdown formatting for basic display
          const plainText = element.content!
            .replace(/\*\*(.+?)\*\*/g, '$1')
            .replace(/\*(.+?)\*/g, '$1')
            .replace(/`(.+?)`/g, '$1')
            .replace(/~~(.+?)~~/g, '$1');

          // Split by lines and create paragraphs
          const lines = plainText.split('\n').filter(line => line.trim());
          for (const line of lines) {
            docElements.push(
              new Paragraph({
                children: [new TextRun(line)],
                spacing: { after: 120 },
              })
            );
          }
        } else if (element.type === 'diagram' || element.type === 'image') {
          if (element.imageData) {
            try {
              // Convert base64 to buffer
              const base64Data = element.imageData.split(',')[1];
              const buffer = Uint8Array.from(atob(base64Data), c => c.charCodeAt(0));

              docElements.push(
                new Paragraph({
                  children: [
                    new ImageRun({
                      data: buffer,
                      transformation: {
                        width: 500,
                        height: 300,
                      },
                    }),
                  ],
                  spacing: { before: 120, after: 120 },
                  alignment: AlignmentType.CENTER,
                })
              );
            } catch (error) {
              console.error('Error adding image to Word:', error);
            }
          }
        }
      }

      // Create the document
      const doc = new Document({
        sections: [
          {
            properties: {},
            children: docElements,
          },
        ],
      });

      // Generate and save
      const blob = await Packer.toBlob(doc);
      const arrayBuffer = await blob.arrayBuffer();

      const savePath = await save({
        defaultPath: `${options.title}.docx`,
        filters: [
          {
            name: 'Word Document',
            extensions: ['docx'],
          },
        ],
      });

      if (savePath) {
        await writeFile(savePath, new Uint8Array(arrayBuffer));
        console.log('Word document exported successfully:', savePath);
      }
    } catch (error) {
      console.error('Error exporting to Word:', error);
      exportError.value = error instanceof Error ? error.message : 'Failed to export Word document';
      throw error;
    } finally {
      isExporting.value = false;
    }
  }

  /**
   * Main export function
   */
  async function exportPage(options: ExportOptions): Promise<void> {
    if (options.format === 'pdf') {
      await exportToPDF(options);
    } else if (options.format === 'docx') {
      await exportToWord(options);
    } else {
      throw new Error(`Unsupported export format: ${options.format}`);
    }
  }

  return {
    isExporting,
    exportError,
    exportPage,
    exportToPDF,
    exportToWord,
  };
}
