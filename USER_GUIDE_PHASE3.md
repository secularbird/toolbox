# Wiki Editor - New Features Guide

## 📄 Document Import

### How to Import Word Documents

1. Click the **"📄 Import"** button in the top toolbar
2. Click **"Choose File"** and select a Word document (.docx or .doc)
3. Wait for the document to be processed
4. Preview the imported content:
   - Edit the **title** if needed
   - Review and edit the **content** (markdown format)
5. Click **"Import as New Page"** to create the page
6. The page will be created with the tag `imported`

### Supported Formats
- ✅ Microsoft Word (.docx)
- ✅ Microsoft Word 97-2003 (.doc)
- ⏳ PDF (coming in future update)

### Tips
- Large documents may take a few seconds to process
- Formatting is converted to markdown (headings, bold, italic, lists)
- Images in documents are not yet extracted (coming soon)
- You can edit the content before importing

---

## ⊞ Table Insertion

### How to Insert Tables

**Method 1: Toolbar Button**
1. Click the **table button (⊞)** in the editor toolbar
2. Configure the table:
   - Set number of **rows** (1-20)
   - Set number of **columns** (1-10)
   - Toggle **"Include header row"** checkbox
3. Preview the markdown in the dialog
4. Click **"Insert Table"** to add it to your page

**Method 2: Keyboard Shortcut**
- Press `Ctrl+Shift+T` (Windows/Linux) or `Cmd+Shift+T` (Mac)
- Follow the same configuration steps

### Table Format
Tables are inserted as markdown:

```markdown
| Header 1 | Header 2 | Header 3 |
| --- | --- | --- |
| | | |
| | | |
```

### Tips
- Tables are inserted at the current cursor position
- You can edit the table directly in the markdown
- Headers help organize your data
- Keep tables simple for best readability

---

## ⌨️ Keyboard Shortcuts Reference

### Editor Formatting
- `Ctrl+B` / `Cmd+B` - **Bold**
- `Ctrl+I` / `Cmd+I` - *Italic*
- `Ctrl+K` / `Cmd+K` - Insert Link

### Content Insertion
- `Ctrl+Shift+T` / `Cmd+Shift+T` - Insert Table
- `Tab` - Indent
- `Shift+Tab` - Outdent

### History
- `Ctrl+Z` / `Cmd+Z` - Undo
- `Ctrl+Shift+Z` / `Cmd+Shift+Z` - Redo
- `Ctrl+Y` / `Cmd+Y` - Redo (alternative)

### Page Management
- **Right-Click** on page in sidebar - Context menu (Open, Rename, Delete)
- Use toolbar buttons for Save, Import, etc.

---

## 🖱️ Context Menu (Right-Click)

### Quick Page Actions

**Right-click on any page** in the sidebar to access:
- **📄 Open** - Navigate to the page
- **✏️ Rename** - Select the page and focus on title for editing
- **🗑️ Delete** - Delete the page (with confirmation)

### Usage Tips
- Right-click works on any page in the sidebar
- Menu appears at cursor position
- Click outside menu to close
- Delete requires confirmation for safety
- Rename automatically selects the title text for easy editing

---

## 🎨 Markdown Features

### Supported Syntax

**Basic Formatting**
- `**bold**` → **bold**
- `*italic*` → *italic*
- `` `code` `` → `code`
- `[link](url)` → clickable link

**Headings**
```markdown
# Heading 1
## Heading 2
### Heading 3
```

**Lists**
```markdown
- Bullet item
- Another item

1. Numbered item
2. Another item
```

**Code Blocks**
````markdown
```javascript
function hello() {
  console.log("Hello");
}
```
````

**Tables** (use table insertion tool!)
```markdown
| Column 1 | Column 2 |
| --- | --- |
| Data 1 | Data 2 |
```

**Quotes**
```markdown
> This is a quote
```

---

## 🔧 Mermaid Diagrams

The wiki supports all Mermaid diagram types:

**Flowcharts, Sequence Diagrams, Gantt Charts, Class Diagrams, State Diagrams, ER Diagrams, User Journey, Pie Charts, Git Graphs, Requirement Diagrams, Mindmaps, Timelines, C4 Diagrams, XY Charts, Block Diagrams, Packet Diagrams, Architecture Diagrams**

See `MERMAID_SYNTAX_GUIDE.md` for detailed examples.

---

## 📝 Best Practices

### Document Organization
- Use **sections** to organize pages by topic
- Add **tags** for cross-cutting concerns
- Use descriptive **titles**
- Keep pages focused on one topic

### Content Writing
- Start with a clear heading
- Use lists for multiple items
- Add code blocks for technical content
- Use tables for structured data
- Include diagrams for complex concepts

### Importing Documents
- Review imported content for formatting issues
- Edit titles to match wiki naming conventions
- Add appropriate tags after import
- Break large documents into multiple pages if needed

### Tables
- Keep tables small (< 10 columns)
- Use header rows for clarity
- Consider lists for simple data
- Format data consistently

---

## 🐛 Troubleshooting

### Import Issues
- **"Failed to import"**: File may be corrupted or too large
- **Formatting issues**: Some complex Word features may not convert perfectly
- **Solution**: Edit content after import or copy-paste manually

### Table Issues
- **Table not inserting**: Make sure cursor is in the editor
- **Formatting broken**: Check markdown syntax
- **Solution**: Use the table insertion tool for proper formatting

### General Issues
- **Autosave not working**: Check unsaved changes indicator
- **Search not finding pages**: Check spelling and filters
- **Diagrams not rendering**: Check Mermaid syntax

---

## 🚀 Coming Soon

### Phase 3 (Next Updates)
- **Smart Hints**: Auto-suggest page titles and tags
- **Collapsible Images**: Hide/show images to reduce clutter
- **Advanced Search**: Full-text search with filters
- **Mind Maps**: Visualize page connections

### Phase 4
- **Collaboration**: Multi-user editing
- **Comments**: Inline annotations
- **Permissions**: Role-based access control

### Phase 5
- **Export**: PDF and HTML export
- **Templates**: Reusable page templates
- **API**: Integration with other tools

---

## 📚 More Resources

- `MERMAID_SYNTAX_GUIDE.md` - Comprehensive diagram syntax
- `MERMAID_TEST.md` - Example diagrams
- `IMPLEMENTATION_SUMMARY.md` - Technical details
- `PHASE3_SUMMARY.md` - Latest feature updates

---

**Need Help?** Check the documentation files or create an issue in the repository.
