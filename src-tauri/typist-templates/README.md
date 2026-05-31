# Typist Templates

This directory contains Typst document templates for the LOGOS application.

## Available Templates

### 1. Slide Template (`slide.typ`)
A basic slide template for presentations.

**Variables:**
- `title` (optional): The title of the slide (default: "Slide Title")
- `content` (optional): The content of the slide (default: "Slide content goes here")

**Usage:**
```typst
= My Slide Title

This is the slide content with bullet points:
- Point 1
- Point 2
- Point 3
```

### 2. Document Template (`document.typ`)
A standard document template with metadata.

**Variables:**
- `title` (required): The title of the document
- `author` (optional): The author of the document (default: "Author Name")
- `content` (optional): The content of the document (default: "Document content goes here")

**Usage:**
```typst
#set document(title: "My Document", author: "John Doe")

= My Document

This is the main content of the document.

== Section 1

Content for section 1.

== Section 2

Content for section 2.
```

### 3. Code Template (`code.typ`)
A template for code examples and demonstrations.

**Variables:**
- `title` (optional): The title of the code example (default: "Code Example")
- `description` (optional): Description of the code (default: "Code description")
- `code` (optional): The code to display (default: "let x = 10")

**Usage:**
```typst
= My Code Example

This example demonstrates how to use the function.

```typst
let x = 10
let y = 20
let result = x + y
```
```

## Template Variables

Templates use the `{{variable_name}}` syntax for placeholders. When rendering a template:

1. **Required variables** must be provided or an error will occur
2. **Optional variables** can be omitted and will use their default values
3. **Optional variables without defaults** will leave the placeholder unchanged if not provided

## Using Templates Programmatically

```rust
use logos::typist_service::TemplateEngine;

// Create template engine
let mut engine = TemplateEngine::new();

// Register a template
let template = TemplateEngine::create_document_template("my-doc");
engine.register_template(template);

// Render with values
let mut values = HashMap::new();
values.insert("title".to_string(), "My Document".to_string());
values.insert("author".to_string(), "John Doe".to_string());
values.insert("content".to_string(), "Document content".to_string());

let rendered = engine.render("my-doc", &values)?;
```

## Custom Templates

You can create custom templates by:

1. Creating a new `.typ` file in this directory
2. Using `{{variable_name}}` placeholders for dynamic content
3. Defining variables with their metadata (name, default, description, required)
4. Registering the template in the TemplateEngine

## Template Best Practices

1. **Use descriptive variable names** that clearly indicate their purpose
2. **Provide sensible defaults** for optional variables
3. **Include descriptions** for complex variables
4. **Validate templates** before use to check for unclosed placeholders
5. **Test templates** with various input combinations

## File Structure

```
typist-templates/
├── README.md           # This file
├── slide.typ           # Slide presentation template
├── document.typ        # Standard document template
└── code.typ            # Code demonstration template
```

## Adding New Templates

To add a new template:

1. Create a new `.typ` file in this directory
2. Define the template structure with variables
3. Add a corresponding factory method in `TemplateEngine` (e.g., `create_custom_template`)
4. Update this README with the new template documentation
5. Add tests for the new template in `template.rs`

## Example: Creating a Custom Template

```typst
#set page(paper: "a4", margin: (left: 2cm, right: 2cm))

= {{title}}

{{subtitle}}

Author: {{author}}
Date: {{date}}

---

{{content}}
```

```rust
pub fn create_custom_template(name: &str) -> Template {
    Template {
        name: name.to_string(),
        description: "Custom template with metadata".to_string(),
        content: r#"#set page(paper: "a4", margin: (left: 2cm, right: 2cm))

= {{title}}

{{subtitle}}

Author: {{author}}
Date: {{date}}

---

{{content}}
"#.to_string(),
        variables: vec![
            TemplateVariable {
                name: "title".to_string(),
                default_value: Some("Custom Title".to_string()),
                description: Some("The main title".to_string()),
                required: true,
            },
            TemplateVariable {
                name: "subtitle".to_string(),
                default_value: None,
                description: Some("Optional subtitle".to_string()),
                required: false,
            },
            TemplateVariable {
                name: "author".to_string(),
                default_value: Some("Unknown".to_string()),
                description: Some("Document author".to_string()),
                required: false,
            },
            TemplateVariable {
                name: "date".to_string(),
                default_value: None,
                description: Some("Document date".to_string()),
                required: false,
            },
            TemplateVariable {
                name: "content".to_string(),
                default_value: Some("Content goes here".to_string()),
                description: Some("Main document content".to_string()),
                required: false,
            },
        ],
    }
}
```
