# CraftCN

A "shadcn/ui" for Minecraft Plugins - CLI tool for scaffolding UI primitives.

## Overview

CraftCN is a CLI tool and component registry designed to solve the UI fragmentation problem in the Minecraft ecosystem. Instead of shading massive libraries that cause version conflicts, CraftCN scaffolds source-available, headless UI primitives directly into your project.

## Features

- 🎨 **Theming Engine**: Global design system with `UITheme.java` ensuring consistent visual identity
- ⚡ **Performance**: Sub-100ms execution time with zero JVM startup overhead (Rust-based)
- 🤖 **LLM-Optimized**: Token-efficient component contexts for AI agents
- 📦 **Zero Dependencies**: No compiled JARs - all source code is editable
- 🔧 **Modern Java**: Records, Pattern Matching, Lambdas

## Installation

```bash
cargo install craftcn
```

Or download the binary from the [releases](https://github.com/modpotato/craftcn/releases) page.

## Quick Start

```bash
# Initialize CraftCN in your project
craftcn init

# List available components
craftcn list

# Add a component
craftcn add paginated-menu

# Get LLM-optimized context
craftcn context paginated-menu
```

## Commands

### `craftcn init`

Initialize CraftCN in your project.

```bash
craftcn init [--package <package>] [--theme <theme>]
```

Prompts for:
- Root package name (e.g., `com.example.plugin`)
- Theme selection (default, dark, ocean)

Generates:
- `craftcn.json` configuration file
- `src/.../ui/UITheme.java`
- `src/.../ui/core/BaseMenu.java`

### `craftcn add <component>`

Add a component to your project.

```bash
craftcn add <component> [--force]
```

Automatically resolves dependencies and generates all required files.

### `craftcn context <component>`

Get LLM-optimized component context.

```bash
craftcn context <component> [--verbose]
```

Outputs a minified, token-efficient summary of the component's public API.

Example output:
```java
// Context: PaginatedMenu<T>
// Category: A
// Dependencies: base-menu, item-builder
//
// Class: PaginatedMenu<T>
//   Type Parameters: T
// - void setSource(List<T> items)
// - void setRenderer(BiConsumer<T, Integer> renderer)
// - void nextPage()
// - void previousPage()
```

### `craftcn list`

List available components.

```bash
craftcn list [--category <A|B|C|D>] [--installed]
```

### `craftn theme`

Manage themes.

```bash
craftcn theme list                    # List available themes
craftcn theme info <theme>            # Get theme details
craftcn theme apply <theme>           # Apply a theme
```

## Component Catalog

### Category A: Inventory GUIs

- **base-menu**: Abstract parent class with click handling and cleanup
- **paginated-menu**: Auto-calculating pagination with navigation
- **confirmation-menu**: Modal dialog with confirm/deny callbacks
- **form-menu**: Anvil/Sign wrapper for string input
- **selection-menu**: Multi-select scrolling list

### Category B: Chat Widgets

- **chat-prompt**: Captures next chat message with timeout support
- **interactive-message**: Builder pattern for hover/click events

### Category C: HUD & Visuals

- **scoreboard-sidebar**: Flicker-free packet-based scoreboard
- **bossbar-timer**: Countdown with Adventure BossBar wrapper
- **toast-notification**: Achievement-style popups

### Category D: Utilities

- **item-builder**: Fluent API for ItemStack creation
- **head-util**: Player head fetching with caching

## Themes

### Default
Classic Minecraft styling with vibrant colors and clear contrast.

### Dark
Modern dark theme with deep blacks and subtle accents.

### Ocean
Calm oceanic theme with blues and teals.

## Architecture

### The CLI (Rust)
- Single static binary
- Zero JVM startup overhead
- Distributed via cargo install

### The Registry
- Static JSON API (hosted on GitHub Pages)
- Component catalog with dependency trees
- Source-available Java code

### The Primitives (Java)
- Strictly Paper API (latest) and Adventure (Kyori)
- No NMS unless absolutely necessary
- Modern Java (Records, Pattern Matching, Lambdas)

## Configuration

`craftcn.json` is stored in your project root:

```json
{
  "version": "0.1.0",
  "package": "com.example.plugin",
  "theme": "default",
  "components": ["base-menu", "paginated-menu"]
}
```

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Running locally

```bash
cargo run -- init
cargo run -- add paginated-menu
cargo run -- context paginated-menu
```

## License

GNU GPLv3

## Contributing

Contributions are welcome! Please read our contributing guidelines.

## Support

- GitHub Issues: https://github.com/modpotato/craftcn/issues

## Acknowledgments

- Inspired by [shadcn/ui](https://ui.shadcn.com/)
- Built with [Rust](https://www.rust-lang.org/)
- Powered by [PaperMC](https://papermc.io/) and [Adventure](https://docs.advntr.dev/)
