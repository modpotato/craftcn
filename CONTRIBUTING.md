# Contributing to CraftCN

Thanks for contributing! This guide covers adding components, the Rust CLI, and how changes get verified.

## Quick start

1. Fork the repository and branch from `develop`: `git checkout -b feature/my-component`
2. Add your sources under `registry/components/<component-name>/`
3. Add an entry to `registry/index.json`
4. Run the checks (see [Verifying changes](#verifying-changes))
5. Open a pull request against `develop`

## Component layout

```
registry/components/<component-name>/
└── ui/
    ├── <area>/
    │   └── <ClassName>.java
    └── ...
```

Sources are written under the `com.craftcn` namespace. `craftcn add` rewrites every `com.craftcn` reference (package
line, imports and qualified names) to the user's package, and adds the `UITheme` import where it's needed.

### Registry entry

Add the component to `registry/index.json`, under `components`. Categories are derived from this list, so there is no
separate category index to keep in sync.

```json
{
  "name": "your-component",
  "category": "A",
  "description": "One line, shown by craftcn list",
  "dependencies": ["base-menu", "item-builder"],
  "minecraft": "1.21.4",
  "resource_pack": false,
  "files": [
    { "path": "ui/area/YourClass.java" }
  ]
}
```

| Field | Meaning |
| --- | --- |
| `dependencies` | Components installed first. Must exist in the registry. |
| `minecraft` | Lowest Minecraft version the APIs need. Omit it only if the component uses nothing version-specific. |
| `resource_pack` | `true` if the component draws from `craftcn pack` output. `craftcn doctor` then checks that the pack exists. |
| `files` | Source paths relative to the component directory. They must stay inside it: no `..` and no absolute paths. |

### Categories

- **A**: Inventory GUIs and dialogs
- **B**: Chat widgets
- **C**: HUD & visuals
- **D**: Utilities
- **E**: Resource pack GUIs

## Java coding standards

Components target the latest stable Paper API and Adventure. Keep them to public Paper and Adventure APIs. Don't use
NMS, and don't shade dependencies.

1. **Use modern APIs**
   - Build menus with `InventoryHolder`, not inventory identity checks.
   - Use Adventure `Component`s for text. Don't use legacy `ChatColor` strings.
   - Use item data (`ItemMeta.setItemModel`, `setTooltipStyle`, `setEnchantmentGlintOverride`), not
     `setCustomModelData`, where a modern equivalent exists.
   - Don't use deprecated or removal-marked API. `./scripts/verify-components.sh` compiles with `-Xlint:all`, so new
     warnings show up there.

2. **Folia-safe threading**
   - Touch a player only on that player's thread: `Tasks.run(plugin, player, ...)`, not `Bukkit.getScheduler()`.
   - Touch entities and blocks only on their region: `Tasks.at(plugin, location, ...)`.
   - Async-chat, async-network and other off-thread callbacks must hand results back through `Tasks`.

3. **No magic strings**
   - Put user-visible text in named constants (for example `PaginatedMenu.NEXT_LABEL`), or in the theme.

4. **Use base classes**
   - Menus extend `BaseMenu`, and clickable slots use `Button`. Don't register your own inventory listeners.

5. **Documentation**
   - Add Javadoc to every public type and method, and state which thread a method must be called from.

## Theme and resource pack constants

`UITheme` is generated from `registry/themes.json`. Each theme sets a palette, materials and sounds.

- Sounds are referenced as `Sound.<CONSTANT>` and checked at compile time. Each value must match a constant on
  `org.bukkit.Sound`.
- Materials use `Material.valueOf`, so each value must be an exact `Material` enum name.
- The `texture-gui` glyph and shift code points must match `src/pack/font.rs`. Change both together.

## Rust CLI standards

1. **Error handling**: return `anyhow::Result`, and add context with `.context(...)`.
2. **No `unwrap()`** outside tests.
3. **Safety**: registry paths are untrusted. Pass them through `safe_relative_path` before touching the filesystem.
4. **Tests**: write them for each module. Use `tempfile` for filesystem tests.
5. **Style**: run `cargo fmt`, and `cargo clippy` before pushing.

## Verifying changes

```bash
cargo test                          # CLI, registry integrity and generator tests
bash scripts/verify-components.sh   # installs every component into a Paper 26.2 project and compiles it
```

`verify-components.sh` needs Maven and a JDK 25, because Paper 26.x is built for Java 25. CI runs both checks on
pushes and pull requests to `develop`.

For a component, also run it in a server if you can. Headless server checks catch thread and runtime errors that the
compiler can't.

## Commit messages

Use the imperative mood, capitalise the subject, and keep the body at 72 characters or less:

```
add(hud): Add Hologram component

- Category: C
- Dependencies: scheduler
- Minecraft: 1.21.4+
- Files: ui/hud/Hologram.java
```

## Pull request guidelines

1. **Keep it focused.** One component, or one CLI feature, per pull request.
2. **Update the registry** when a component's files or dependencies change.
3. **Update the docs** (README and the component table) when you add a command or category.
4. **Add a CHANGELOG entry** under "Unreleased".

## Getting help

1. Check the [issues](https://github.com/modpotato/craftcn/issues) for similar questions.
2. Open an issue for bugs or feature requests.

Thank you for making CraftCN better!
