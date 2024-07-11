# `repolog`

`repolog` is a command-line tool designed to enhance code organization and facilitate code review processes. It provides functionality for adding title comments to files and exporting files of a specific language from a project.

## Features

- **Title Addition**: Recursively adds commented titles to files in a repository, indicating their location within the project structure.
- **File Export**: Concatenates all files of a specified programming language into a single file for easier review or analysis.
- **Language Support**: Currently supports Rust and Python, with an extensible design for easy addition of more languages.
- **Flexible Output**: Can write to stdout or a specified file.

## Installation

To install `repolog`, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo, you can install `repolog` with `cargo`

```bash
cargo install repolog
```

## Usage

`repolog` has two main commands: `title` and `export`.

### Adding Titles

To add titles to all files of a specific language in a directory:

```bash
repolog title --lang <language> <path>
```

For example, to add titles to all Rust files in the current directory:

```bash
repolog title --lang rust .
```

### Exporting Files

To export all files of a specific language from a directory:

```bash
repolog export --lang <language> --output <output_file> <path>
```

For example, to export all Python files from the current directory to a file named `combined_python_files.py`:

```bash
repolog export --lang python --output combined_python_files.py .
```

If you don't specify an output file, the result will be printed to stdout:

```bash
repolog export --lang rust .
```

## Use Cases

1. **Code Review**: Export all files of a specific language to a single file for easier review, especially useful for pull requests or code audits.

2. **Project Organization**: Add consistent title comments to all files in a project, making it easier to understand the project structure at a glance.

3. **Documentation Generation**: Use the export feature to generate a single file containing all code of a specific language, which can be useful for documentation purposes or for creating a "literate programming" style document.

4. **Refactoring**: When moving files around in a large project, use the title feature to ensure all files have up-to-date location comments.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
