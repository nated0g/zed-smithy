# zed-smithy

Smithy language support for [Zed](https://github.com/zed-industries/zed), including syntax highlighting and language server integration.

## Features

- Syntax highlighting via Tree-sitter
- Language server integration with [smithy-language-server](https://github.com/smithy-lang/smithy-language-server)
- Automatic installation of the language server based on your platform and architecture

## Installation

### From Zed extension registry

1. Open Zed
2. Open the command palette (Cmd+Shift+P)
3. Run "Extensions: Install Extension"
4. Search for "Smithy" and install

### Manual installation

- Clone this repo: `git clone https://github.com/joshrutkowski/zed-smithy`
- Clone the [tree-sitter-smithy](https://github.com/indoorvivants/tree-sitter-smithy) repo: `https://github.com/indoorvivants/tree-sitter-smithy`
- CD into the repo: `cd tree-sitter-smithy`
- Build the WASM: `tree-sitter build-wasm` (might require docker-engine running)
- Rename the WASM file to `smithy.wasm`
- Move the WASM file into `smithy/grammars` (this repository)
- Move the `smithy` repository to `~/Library/Application Support/Zed/extensions/installed`

## Language Server

The extension automatically downloads and installs the [Smithy Language Server](https://github.com/smithy-lang/smithy-language-server) (v0.7.0) on first use. The language server provides:

- Code completion
- Go to definition
- Find references
- Diagnostics
- Hover information
- Outline view

The language server binary is downloaded from the official GitHub releases based on your platform and architecture.


