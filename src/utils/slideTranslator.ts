/**
 * HTML to Typst Slide (Touying framework) translator utility
 * Converts TipTap editor HTML output to Typst slide markup using Touying framework
 */

export interface SlideConfig {
  theme?: string;
  aspectRatio?: '16-9' | '4-3';
  showSlideNumbers?: boolean;
}

function validateConfig(config: SlideConfig): Required<SlideConfig> {
  return {
    theme: config.theme || 'university-theme',
    aspectRatio: config.aspectRatio || '16-9',
    showSlideNumbers: config.showSlideNumbers !== undefined ? config.showSlideNumbers : true
  };
}

function sanitizeTypstContent(content: string): string {
  // Escape special Typst characters
  return content
    .replace(/\\/g, '\\\\')
    .replace(/"/g, '\\"')
    .replace(/\$/g, '\\$')
    .replace(/#/g, '\\#')
    .replace(/{/g, '\\{')
    .replace(/}/g, '\\}')
    .replace(/_/g, '\\_')
    .replace(/&/g, '\\&')
    .replace(/\*/g, '\\*')
    .replace(/\[/g, '\\[')
    .replace(/]/g, '\\]')
    .replace(/\|/g, '\\|')
    .replace(/</g, '<')
    .replace(/>/g, '>');
}

export function htmlToTypstSlides(html: string, config: SlideConfig = {}): string {
  if (!html || typeof html !== 'string') {
    return generateEmptySlide(validateConfig(config));
  }

  const validatedConfig = validateConfig(config);
  const { theme, aspectRatio } = validatedConfig;

  // Detect horizontal rules as slide breaks
  const slides = html.split(/<hr\s*\/?>|<hr>|<hr \/>/g).filter(s => s.trim());

  if (slides.length === 0) {
    return generateEmptySlide(validatedConfig);
  }

  let typst = `#import "@preview/touying:0.5.2": *
#show: ${theme}.with(aspect-ratio: "${aspectRatio}")\n\n`;

  slides.forEach((slideContent, index) => {
    typst += convertSlideContent(slideContent, index + 1);
  });

  return typst;
}

function generateEmptySlide(config: Required<SlideConfig>): string {
  const { theme, aspectRatio } = config;
  return `#import "@preview/touying:0.5.2": *
#show: ${theme}.with(aspect-ratio: "${aspectRatio}")\n\n= Empty Slide\n\n`;
}

function convertSlideContent(html: string, slideNumber: number): string {
  let content = html.trim();

  if (!content) {
    return `= Slide ${slideNumber}\n\n`;
  }

  // Extract title (first h1 or h2)
  const titleMatch = content.match(/<h[12]>(.*?)<\/h[12]>/);
  const title = titleMatch ? sanitizeTypstContent(titleMatch[1]) : `Slide ${slideNumber}`;

  // Remove title from content
  content = content.replace(/<h[12]>.*?<\/h[12]>/, '');

  // Convert content to Touying slide format
  content = convertSlideBody(content);

  return `= ${title}\n${content}\n\n`;
}

function convertSlideBody(html: string): string {
  let body = html;

  // Convert headings (h3 becomes section, h4+ becomes subsection)
  body = body.replace(/<h3>(.*?)<\/h3>/g, '== $1\n');
  body = body.replace(/<h4>(.*?)<\/h4>/g, '=== $1\n');
  body = body.replace(/<h5>(.*?)<\/h5>/g, '==== $1\n');
  body = body.replace(/<h6>(.*?)<\/h6>/g, '===== $1\n');

  // Convert bold and italic
  body = body.replace(/<strong>(.*?)<\/strong>/g, '*$1*');
  body = body.replace(/<b>(.*?)<\/b>/g, '*$1*');
  body = body.replace(/<em>(.*?)<\/em>/g, '_$1_');
  body = body.replace(/<i>(.*?)<\/i>/g, '_$1_');

  // Convert strikethrough
  body = body.replace(/<s>(.*?)<\/s>/g, '#strike($1)');
  body = body.replace(/<strike>(.*?)<\/strike>/g, '#strike($1)');
  body = body.replace(/<del>(.*?)<\/del>/g, '#strike($1)');

  // Convert code
  body = body.replace(/<code>(.*?)<\/code>/g, '`$1`');
  body = body.replace(/<pre>(.*?)<\/pre>/g, '```\n$1\n```');

  // Convert blockquotes as highlighted blocks
  body = body.replace(
    /<blockquote>(.*?)<\/blockquote>/g,
    '#block(fill: rgb("f0f0f0"), inset: 8pt, radius: 4pt)[\n  $1\n]\n'
  );

  // Convert paragraphs
  body = body.replace(/<p>(.*?)<\/p>/g, '$1\n\n');

  // Convert unordered lists
  body = body.replace(/<ul>(.*?)<\/ul>/g, (_: string, listContent: string) => {
    const items = listContent.match(/<li>(.*?)<\/li>/g) || [];
    const typstItems = items
      .map((item: string) => {
        const content = item.replace(/<\/?li>/g, '').trim();
        return `- ${content}`;
      })
      .join('\n');
    return `${typstItems}\n\n`;
  });

  // Convert ordered lists with proper numbering
  let listCounter = 0;
  body = body.replace(/<ol>(.*?)<\/ol>/g, (_: string, listContent: string) => {
    const items = listContent.match(/<li>(.*?)<\/li>/g) || [];
    const typstItems = items
      .map((item: string) => {
        const content = item.replace(/<\/?(li|ol)>/g, '').trim();
        listCounter++;
        return `${listCounter}. ${content}`;
      })
      .join('\n');
    return `${typstItems}\n\n`;
  });

  // Convert tables
  body = body.replace(/<table>(.*?)<\/table>/g, (_: string, tableContent: string) => {
    const rows = tableContent.match(/<tr>(.*?)<\/tr>/g) || [];
    if (rows.length === 0) {
return '';
}

    const firstRowCells = rows[0]?.match(/<(td|th)>(.*?)<\/(td|th)>/g) || [];
    const colCount = firstRowCells.length;

    let tableTypst = `#table(\n  columns: (${Array(colCount).fill('auto').join(', ')}),\n`;

    rows.forEach((row: string) => {
      const cells = row.match(/<(td|th)>(.*?)<\/(td|th)>/g) || [];
      const cellContents = cells.map((cell: string) => {
        const content = cell.replace(/<\/?(td|th)>/g, '').trim();
        return `[${content}]`;
      });
      tableTypst += `  ${cellContents.join(', ')}\n`;
    });

    tableTypst += ')\n';
    return tableTypst;
  });

  // Convert line breaks
  body = body.replace(/<br\s*\/?>/g, '\n');

  // Remove all remaining HTML tags
  body = body.replace(/<[^>]*>/g, '');

  // Clean up excessive whitespace
  body = body.replace(/\n{3,}/g, '\n\n');

  return body.trim();
}

// Alternative: Convert markdown with page breaks to slides
export function markdownToTypstSlides(markdown: string, config: SlideConfig = {}): string {
  if (!markdown || typeof markdown !== 'string') {
    return generateEmptySlide(validateConfig(config));
  }

  const validatedConfig = validateConfig(config);
  const { theme, aspectRatio } = validatedConfig;

  // Detect --- as slide breaks
  const slides = markdown.split(/^---$/gm).filter(s => s.trim());

  if (slides.length === 0) {
    return generateEmptySlide(validatedConfig);
  }

  let typst = `#import "@preview/touying:0.5.2": *
#show: ${theme}.with(aspect-ratio: "${aspectRatio}")\n\n`;

  slides.forEach(slideContent => {
    typst += convertMarkdownSlide(slideContent.trim());
  });

  return typst;
}

function convertMarkdownSlide(markdown: string): string {
  if (!markdown) {
    return '= Untitled\n\n';
  }

  const lines = markdown.split('\n');
  let title = 'Untitled';
  let content = '';
  let titleFound = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();

    // First heading is the slide title
    if (line.startsWith('#') && !titleFound) {
      const level = line.match(/^#+/)?.[0].length || 1;
      if (level <= 2) {
        title = sanitizeTypstContent(line.replace(/^#+\s*/, ''));
        titleFound = true;
        continue;
      }
    }

    content += line + '\n';
  }

  // Convert markdown content to Typst
  content = content
    .replace(/^###\s+(.*)$/gm, '== $1')
    .replace(/^####\s+(.*)$/gm, '=== $1')
    .replace(/^#####\s+(.*)$/gm, '==== $1')
    .replace(/^######\s+(.*)$/gm, '===== $1')
    .replace(/\*\*(.*?)\*\*/g, '*$1*')
    .replace(/_(.*?)_/g, '_$1_')
    .replace(/`(.*?)`/g, '`$1`')
    .replace(/^- (.*)$/gm, '- $1')
    .replace(/^\d+\. (.*)$/gm, '1. $1');

  return `= ${title}\n${content}\n\n`;
}
