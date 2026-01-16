# Contributing to CraftCN

Thank you for your interest in contributing to CraftCN! This guide will help you get started.

## Quick Start

1. Fork the CraftCN repository
2. Create a new branch: `git checkout -b feature/my-component`
3. Add your component to `registry/components/<component-name>/`
4. Update `registry/index.json` with your component metadata
5. Test your changes locally
6. Commit and push: `git push origin feature/my-component`
7. Submit a Pull Request to the main CraftCN repository

## Component Development

### File Structure

Each component should follow this structure:

```
registry/components/<component-name>/
└── ui/
    ├── <package>/
    │   └── <ComponentFile>.java
    └── ...
```

### Component Metadata

Add your component to `registry/index.json`:

```json
{
  "name": "your-component-name",
  "category": "A|B|C|D",
  "description": "Brief description of what this component does",
  "dependencies": ["base-menu", "other-component"],
  "files": [
    { "path": "ui/package/ComponentFile.java" }
  ]
}
```

### Component Categories

- **A**: Inventory GUIs (BaseMenu, PaginatedMenu, etc.)
- **B**: Chat Widgets (ChatPrompt, InteractiveMessage, etc.)
- **C**: HUD & Visuals (ScoreboardSidebar, BossBarTimer, etc.)
- **D**: Utilities (ItemBuilder, HeadUtil, etc.)

### Coding Standards

#### Java Components

1. **Zero Magic Strings**
   - All text strings must be defined as methods or constants
   - Do not hardcode strings like "Previous Page" in component code
   - Use `UITheme.PREVIOUS_BUTTON` or similar theme constants

2. **Async-Safe Defaults**
   - Data loading operations must accept CompletableFuture
   - Run heavy computations off the main thread
   - Only sync back to Bukkit for inventory operations

3. **Use Base Classes**
   - Extend appropriate base classes from `ui/core/`
   - Follow the existing patterns for click handlers

4. **Package Structure**
   - All components should be under `com.craftcn.ui.*` in the registry
   - Package rewriting will automatically transform to user's package

5. **Documentation**
   - Add public JavaDoc for all public methods
   - Include usage examples

#### Rust CLI

1. **Error Handling**
   - Always return `anyhow::Result` for fallible operations
   - Provide meaningful context with `.context()` method

2. **Type Safety**
   - Avoid `.unwrap()` calls except in tests with clear assertions
   - Use `?` operator for error propagation

3. **Performance**
   - Clone only when necessary
   - Use references where possible
   - Cache expensive operations

4. **Testing**
   - Write tests for critical paths
   - Use `tempfile` crate for test file creation

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test <test_name>
```

### Example Component Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrites_package_correctly() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());
        
        let input = "package com.craftcn.ui.menus;";
        let expected = "package com.example.plugin.ui.menus;";
        
        assert_eq!(manipulator.rewrite_package(input).unwrap(), expected);
    }
}
```

## Submitting Changes

### Commit Message Format

```
<component-type>(<component-name>): <brief description>

<details>

- Added new component: <component-name>
- Category: <A|B|C|D>
- Dependencies: <list of dependencies>
- Files: <list of Java files>
```

Example:
```
add(inventory): Add PaginatedMenu component

- Added new paginated inventory menu with auto page calculation
- Category: A
- Dependencies: base-menu, item-builder
- Files: ui/menus/PaginatedMenu.java
```

### Pull Request Guidelines

1. **Keep it small and focused**
   - One component per PR is ideal
   - Make the PR title descriptive

2. **Write clear commit messages**
   - Follow the format above
   - Explain why the change was made

3. **Test thoroughly**
   - All tests must pass: `cargo test`
   - Test the component in a real Minecraft plugin

4. **Update documentation**
   - Update README if adding a new category
   - Add examples if functionality is complex

## Style Guide

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo clippy` and address all warnings
- Run `cargo fmt` before committing

### Java Code

- Follow [Google Java Style Guide](https://google.github.io/styleguide/javaguide.html)
- Use modern Java features (records, pattern matching, lambdas)
- Limit line length to 100 characters

### Commit Messages

- Use imperative mood: "Add" not "Added"
- Capitalize the subject line
- Keep the body at 72 characters or less

## Getting Help

If you need help or have questions:

1. **Check the issues** - Look for similar questions or issues
2. **Start a discussion** - Ask questions in GitHub Discussions
3. **Create an issue** - If you've found a bug or want to request a feature

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors graph

Thank you for making CraftCN better! 🚀
