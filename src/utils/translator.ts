/**
 * HTML to Typst translator utility
 * Converts TipTap editor HTML output to Typst markup
 */

export function htmlToTypst(html: string): string {
  let typst = html;

  // 1. Add Typst header with page settings
  const header = '#set page(paper: "a4", margin: (left: 20mm, right: 20mm, top: 25mm, bottom: 25mm))\n#set text(font: "SimSun", size: 11pt)\n\n';

  typst = typst
    // 2. Convert headings
    .replace(/<h1>(.*?)<\/h1>/g, '= $1\n')
    .replace(/<h2>(.*?)<\/h2>/g, '== $1\n')
    .replace(/<h3>(.*?)<\/h3>/g, '=== $1\n')

    // 3. Convert bold and italic
    .replace(/<strong>(.*?)<\/strong>/g, '*$1*')
    .replace(/<b>(.*?)<\/b>/g, '*$1*')
    .replace(/<em>(.*?)<\/em>/g, '_$1_')
    .replace(/<i>(.*?)<\/i>/g, '_$1_')

    // 4. Convert strikethrough
    .replace(/<s>(.*?)<\/s>/g, '#strike($1)')
    .replace(/<strike>(.*?)<\/strike>/g, '#strike($1)')
    .replace(/<del>(.*?)<\/del>/g, '#strike($1)')

    // 5. Convert code
    .replace(/<code>(.*?)<\/code>/g, '`$1`')
    .replace(/<pre>(.*?)<\/pre>/g, '```\n$1\n```')

    // 6. Convert blockquotes
    .replace(
      /<blockquote>(.*?)<\/blockquote>/g,
      '#block(fill: rgb("f0f0f0"), inset: 8pt, radius: 4pt)[\n  $1\n]\n'
    )

    // 7. Convert paragraphs
    .replace(/<p>(.*?)<\/p>/g, '$1\n\n')

    // 8. Convert horizontal rules
    .replace(/<hr\s*\/?>/g, '#line(length: 100%, stroke: 0.5pt)\n\n')

    // 9. Convert tables (simplified)
    .replace(/<table>(.*?)<\/table>/g, (_: string, tableContent: string) => {
      // Extract table rows
      const rows = tableContent.match(/<tr>(.*?)<\/tr>/g) || [];
      if (rows.length === 0) {
return '';
}

      // Extract cells from first row to determine column count
      const firstRowCells = rows[0]?.match(/<(td|th)>(.*?)<\/(td|th)>/g) || [];
      const colCount = firstRowCells.length;

      // Build Typst table
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
    })

    // 10. Convert lists
    .replace(/<ul>(.*?)<\/ul>/g, (_: string, listContent: string) => {
      const items = listContent.match(/<li>(.*?)<\/li>/g) || [];
      const typstItems = items
        .map((item: string) => {
          const content = item.replace(/<\/?li>/g, '').trim();
          return `- ${content}`;
        })
        .join('\n');
      return `${typstItems}\n\n`;
    })
    .replace(/<ol>(.*?)<\/ol>/g, (_: string, listContent: string) => {
      const items = listContent.match(/<li>(.*?)<\/li>/g) || [];
      const typstItems = items
        .map((item: string, index: number) => {
          const content = item.replace(/<\/?li>/g, '').trim();
          return `${index + 1}. ${content}`;
        })
        .join('\n');
      return `${typstItems}\n\n`;
    })

    // 11. Convert line breaks
    .replace(/<br\s*\/?>/g, '\n')

    // 12. Remove all remaining HTML tags
    .replace(/<[^>]*>/g, '')

    // 13. Clean up excessive whitespace
    .replace(/\n{3,}/g, '\n\n');

  return header + typst;
}
