## Polyglot LSP
Polyglot LSP in rust under development

## 💻 Presentation

**Rust implementation of a polyglot (cross-language) Visual Studio Code extension**

This project is a work in progress. It is a Rust implementation of a polyglot (cross-language) Visual Studio Code extension. It is based on the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) and [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) library.

The project is linked with [PolyglotAST](https://github.com/bsauvat/polyglot_ast), which is a project that build the AST of a polyglot program. 
PolyglotAST currently supports Java and partially supports JavaScript and Python.

Today, it is possible to use the extension PolyglotLSP to get the polyglot concrete syntax tree of a polyglot program.
Other polyglot analysis features are under development.

## 📂 Main files changed in the project

- **'Cargo.toml'** : adding dependencies to PolyglotAST project
- **'crates/base-db/src/lib.rs'** : adding polyglot parsing functions
- **'crates/ide/src/lib.rs'** : adding polyglot functions
- **'crates/ide/src/syntax_tree.rs'** : adding polyglot functions
- **'crates/polyglot-analyzer/src/caps.rs'** : adding FileOperationFilter to recognize Java, JavaScript and Python files
- **'crates/polyglot-analyzer/src/handlers/request.rs'** : adding code to display the polyglot concrete syntax tree of a polyglot program
- **'crates/polyglot-analyzer/src/main_loop.rs'** : adding DocumentFilter to recognize Java, JavaScript and Python files
- **'crates/polyglot-analyzer/src/reload.rs'** : adding format for Java, JavaScript and Python files
- **'crates/project_model/src/polyjson_project.rs'** : implementation of PolyJsonProject
- **'crates/project_model/src/workspace.rs'** : adding PolyJsonProject to Workspace
- **'editors/code/language-configurations-(java/js/python).json'** : adding language configurations for Java, JavaScript and Python files
- **'editors/code/package.json'** : link to language configurations files 
- **'editors/code/src/(ast_inspector.ts/client.ts/util.ts)'** : adding last configurations for Java, JavaScript and Python files

## 📝 Running the extension

PolyglotLSP is not yet published on the Visual Studio Code marketplace. To run the extension, you need to :
- **Clone** this repository : 
```bash 
git clone https://github.com/bsauvat/polyglot-lsp.git
```
- go to **Run and Debug** section in Visual Studio Code and choose the **Run Extension (Debug Build)** configuration
- wait for a few seconds / minutes for the extension to compile
- wait for the extension polyglot-analyzer to be launched in the new Visual Studio Code window
- open a polyglot program (e.g. `examples/polyglot.java`) with the extension
- with the extension, **open the command palette (Ctrl+Shift+P) or F1**, type and select **'Show Syntax Tree'**

And then a half window will appear on the right side of the screen extension with the polyglot concrete syntax tree of the program.

## 📌 Example of PolyglotLSP usage

Here is an example of the extension with a polyglot Java program :

<img src="assets/Capture_JavaTest2_white.png" alt="Image 1" width="100%">

The extension displays the polyglotAST of JavaTest2.java on the right side of the screen in the window 'tree.rast'

