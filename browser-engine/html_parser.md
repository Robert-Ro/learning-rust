# Part 2: HTML parser

HTML has its own unique [parsing algorithm](http://www.whatwg.org/specs/web-apps/current-work/multipage/syntax.html#parsing). Unlike parsers for most programming languages and file formats, the HTML parsing algorithm does not reject invalid input. Instead it includes specific error-handling instructions, so web browsers can agree on how to display event web pages, _even ones that don't comform to the syntax rules_.

## A Simple HTML Dialect

following syntax is allowed:

- Balanced tags: `<p>...</p>`
- Attributes with quoted values: id="main"
- Text nodes: `<em>...</em>`

Everything else is unsupported, including:

- Comments
- Doctype declarations
- Escaped characters (like &amp;) and CDATA sections
- Self-closing tags: `<br/>` or `<br>` with no closing tag
- Error handling (e.g. unbalanced or improperly nested tags)
- Namespaces and other XHTML syntax: `<html:body>`
- Character encoding detection

Rust strings are stored as UTF-8 byte arrays. To go to the next character, we can't just advance by one byte. Instead we use char_indices which correctly handles multi-byte characters.

## solutions

1. definition Node struct compatiable with sub Nodes
2. use `Vec<char>` iterator
  - check endof file
  - ignore whitespaces
  - check tag open and close
  - check attribute name start and end
  - check attribute value start and end

## APIs

- char_indices: Returns an iterator over the [`char`]s of a string slice, and their positions
