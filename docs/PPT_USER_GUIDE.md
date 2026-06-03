# Logos智道办公软件 PPT Editor User Guide

## Overview

Logos智道办公软件 PPT Editor is a powerful, aerospace-grade presentation editor built with Vue 3 and integrated with Slidev for real-time preview. It provides a WYSIWYG editing experience with support for multiple export formats.

## Features

- **WYSIWYG Editing**: Drag-and-drop interface for creating and editing slides
- **Multiple Element Types**: Text, images, shapes, tables, and code blocks
- **Slide Management**: Add, delete, reorder slides with drag-and-drop
- **Real-time Preview**: Integrated Slidev preview with presenter mode
- **Multiple Export Formats**: Slidev Markdown, Typst, HTML, and PPTX
- **Undo/Redo**: Full history support for all operations
- **Responsive Design**: Works on desktop and tablet devices

## Getting Started

### Installation

```bash
# Install dependencies
bun install

# Start development server
bun run dev
```

### Creating a New Presentation

1. Navigate to the Presentation Editor
2. The editor starts with a blank presentation
3. Add slides using the toolbar or keyboard shortcuts

## Interface Overview

### Toolbar

The toolbar at the top provides quick access to common operations:

- **Add Slide**: Create a new slide
- **Delete Slide**: Remove the current slide
- **Add Text**: Insert a text element
- **Add Image**: Insert an image element
- **Add Shape**: Insert a shape (rectangle, circle, triangle)
- **Add Table**: Insert a table
- **Undo**: Undo the last action
- **Redo**: Redo the last undone action
- **Export Slidev**: Export to Slidev Markdown format
- **Export Typst**: Export to Typst format
- **Export PPTX**: Export to PowerPoint format (requires backend)

### Slide Thumbnails

The left panel shows thumbnail previews of all slides:

- Click a thumbnail to select that slide
- Drag thumbnails to reorder slides
- Active slide is highlighted in blue

### Canvas

The center area is the editing canvas:

- Shows the current slide at actual size
- Click elements to select them
- Drag elements to reposition
- Use resize handles to resize elements
- Click outside elements to deselect

### Properties Panel

The right panel shows properties for the selected element or slide:

- **Element Properties**: Position, size, rotation, opacity, style
- **Slide Properties**: Layout, background color

## Working with Elements

### Adding Elements

1. Click the appropriate button in the toolbar
2. The element appears on the canvas
3. Use the properties panel to customize

### Text Elements

- Double-click to edit text content
- Change font size, color, and weight in properties panel
- Supports basic text formatting

### Image Elements

- Add via toolbar button
- Set image source in properties panel
- Resize using handles or properties panel

### Shape Elements

- Choose from rectangle, circle, or triangle
- Change fill color in properties panel
- Resize and position as needed

### Table Elements

- Default 3x3 table
- Edit cells by clicking
- Resize table via properties panel

### Code Elements

- Add code blocks with syntax highlighting
- Specify programming language
- Edit content directly

## Slide Management

### Adding Slides

- Click "Add Slide" button in toolbar
- New slide appears after current slide
- Default layout is blank

### Deleting Slides

- Select the slide to delete
- Click "Delete Slide" button
- Cannot delete the last slide

### Reordering Slides

- Drag slide thumbnails to reorder
- Drop on target position
- Slide indices update automatically

### Changing Slide Layout

- Select a slide
- Choose layout from properties panel:
  - **Blank**: Empty slide
  - **Title**: Title-only slide
  - **Title and Content**: Title with content area
  - **Two Content**: Two side-by-side content areas

### Changing Slide Background

- Select a slide
- Use color picker in properties panel
- Choose from any color

## Keyboard Shortcuts

### Navigation

- **Arrow Left/Up**: Previous slide
- **Arrow Right/Down**: Next slide
- **Page Up**: Previous slide
- **Page Down**: Next slide
- **Home**: First slide
- **End**: Last slide

### Editing

- **Ctrl+Z**: Undo
- **Ctrl+Y** or **Ctrl+Shift+Z**: Redo
- **Delete**: Delete selected element

### Preview Mode

- **F**: Toggle fullscreen
- **P**: Toggle presenter mode
- **Escape**: Exit preview

## Export Options

### Slidev Markdown

- Export to `.md` file
- Compatible with Slidev presentation framework
- Includes theme and layout information
- Use with `slidev` CLI tool

### Typst

- Export to `.typ` file
- Uses Touying framework
- Compile with Typst compiler
- High-quality PDF output

### HTML

- Export to `.html` file
- Self-contained presentation
- View in any web browser
- No external dependencies

### PPTX

- Export to `.pptx` file
- Compatible with Microsoft PowerPoint
- Requires backend integration
- Full feature support

## Slidev Integration

### Real-time Preview

The Slidev integration provides:

- Live preview of your presentation
- Keyboard navigation
- Fullscreen mode
- Presenter mode with notes

### Presenter Mode

Presenter mode includes:

- **Current Slide Notes**: Display speaker notes
- **Next Slide Preview**: See upcoming slide
- **Timer**: Track presentation duration
- **Slide Thumbnails**: Quick navigation

### Keyboard Navigation in Preview

- **Arrow Keys**: Navigate slides
- **Space**: Next slide
- **F**: Toggle fullscreen
- **P**: Toggle presenter mode
- **Escape**: Exit preview

## Tips and Best Practices

### Performance

- Use shallow rendering for large presentations
- Debounce frequent updates
- Limit history size for memory efficiency

### Organization

- Use descriptive slide titles
- Group related slides together
- Use consistent layouts
- Add speaker notes for reference

### Export Quality

- Test exports before presentation
- Check font compatibility
- Verify image resolution
- Test on target platform

## Troubleshooting

### Common Issues

**Elements not selectable**
- Ensure you're in edit mode (not preview)
- Check if element is locked
- Try clicking outside and reselecting

**Export fails**
- Check file permissions
- Ensure sufficient disk space
- Verify export format compatibility

**Preview not loading**
- Check browser console for errors
- Ensure Slidev dependencies are installed
- Try refreshing the page

**Performance issues**
- Reduce number of elements per slide
- Optimize image sizes
- Clear browser cache

## Advanced Features

### Custom Themes

Modify the presentation theme in the properties panel:

- Change color scheme
- Adjust fonts
- Set background patterns

### Animations

Add animations to elements:

- Select element
- Choose animation type in properties
- Set duration and delay

### Transitions

Add slide transitions:

- Select slide
- Choose transition type
- Set duration

## API Reference

### Document Structure

```typescript
interface PresentationDocument {
  metadata: {
    id: string;
    type: 'presentation';
    title: string;
    author?: string;
    version: string;
    created_at: string;
    updated_at: string;
  };
  theme: PresentationTheme;
  slides: Slide[];
  settings: PresentationSettings;
}
```

### Element Types

- `text`: Text content
- `image`: Image element
- `shape`: Geometric shapes
- `chart`: Data visualization
- `table`: Tabular data
- `video`: Video content
- `code`: Code blocks

## Support

For issues, questions, or contributions:

- GitHub Issues: [project repository]
- Documentation: [docs link]
- Community: [community link]

## License

LOGOS PPT Editor is part of the LOGOS office suite project.
