# CraftCN

A "shadcn/ui" for Minecraft plugins. CraftCN copies source-available UI building blocks into your Paper project
instead of shading a library, so you own the code and can change it.

It targets **Paper 26.2** (latest stable) and works back to Minecraft **1.21.4**. Components are built against the
real Paper API in CI, so a registry change that doesn't compile is caught before release.

## What you get

- **Menus** that route clicks through one listener, with clickable buttons, pagination, selection, confirm dialogs
  and an anvil text input.
- **Native dialogs** (Minecraft 1.21.6+): notices, yes/no confirmations, option grids and forms with text, toggle and
  number inputs.
- **Resource pack GUIs**: `craftcn pack` generates a pack with title plates, item icons and a textured filler,
  themed to match your `craftcn.json`. A `ResourcePackService` sends it to players and checks the SHA-1.
- **HUD and effects**: scoreboard sidebar, per-viewer boss bar countdown, advancement-based toasts, holograms,
  frame animations and text effects (gradient, rainbow, shimmer, typewriter).
- **Folia-safe scheduling** via Paper's region schedulers, so the same code runs on Paper and Folia.
- **Modern item data**: item models, tooltip styles, glint overrides and non-italic names by default.
- **An LLM-friendly CLI**: `craftcn context` prints a compact API summary of any component.

## Installation

```bash
cargo install craftcn
```

Or download a binary from the [releases](https://github.com/modpotato/craftcn/releases) page.

## Quick start

Run these in a Maven or Gradle plugin project (the one containing `pom.xml` or `build.gradle[.kts]`):

```bash
craftcn init                         # detects package and Minecraft version, asks for a theme
craftcn add paginated-menu           # installs it, along with base-menu and item-builder
craftcn doctor                       # checks the project for problems
```

## Commands

| Command | Purpose |
| --- | --- |
| `craftcn init [-p pkg] [-t theme] [-m 26.2] [-y]` | Set up CraftCN. Detects the package and `paper-api` version. `-y` accepts the defaults. |
| `craftcn add <component> [--force]` | Install a component and its dependencies. Existing files are kept unless `--force`. |
| `craftcn remove <component> [--force]` | Delete a component's files. Refuses if other components depend on it. |
| `craftcn list [-c A] [-i]` | List components, grouped by category. `-i` shows installed ones only. |
| `craftcn context <component> [-v]` | Print the public API of a component, for LLM context. |
| `craftcn theme list \| info <name> \| apply <name>` | Browse themes. `apply` switches the project and regenerates `UITheme.java`. |
| `craftcn pack [-o dir] [-z]` | Generate the resource pack. `-z` also writes `build/craftcn-pack.zip` and prints its SHA-1. |
| `craftcn doctor` | Check installed files, dependencies, theme and Minecraft version. Exits non-zero on errors. |
| `craftcn update` | Refresh the cached registry index and themes. |

### Where the registry comes from

By default, CraftCN fetches the registry from the `develop` branch of this repository and caches it. To work from a
local checkout (for example, while developing components), set:

```bash
export CRAFTCN_REGISTRY=/path/to/craftcn/registry
```

To use a fork or staging registry, set `CRAFTCN_REGISTRY_URL` to its base URL.

## Components

Components are grouped by category. Each one lists the lowest Minecraft version it needs.

### A. Inventory GUIs

| Component | Minecraft | Description |
| --- | --- | --- |
| `base-menu` | 1.21.4+ | Menu base class. One listener per plugin, clickable `Button`s, and cleanup that can't leak. |
| `paginated-menu` | 1.21.4+ | Framed grid with previous/next buttons and an empty state. |
| `confirmation-menu` | 1.21.4+ | Confirm and cancel dialog for destructive actions. |
| `selection-menu` | 1.21.4+ | Single or multi-select list with a confirm button. |
| `form-menu` | 1.21.4+ | Anvil text input. The player types, clicks the result, and gets a `CompletableFuture`. |
| `dialog-menu` | 1.21.6+ | Native dialogs: notices, confirmations, option grids, and text/toggle/number forms. |

### B. Chat widgets

| Component | Minecraft | Description |
| --- | --- | --- |
| `chat-prompt` | 1.21.4+ | Captures the next chat message on the player's thread, with timeout and cancel. |
| `interactive-message` | 1.21.4+ | Chat builder with hover text, commands, copy, and server-side click callbacks. |

### C. HUD & visuals

| Component | Minecraft | Description |
| --- | --- | --- |
| `scoreboard-sidebar` | 1.21.4+ | Sidebar where each line updates on its own, with blank score numbers. |
| `bossbar-timer` | 1.21.4+ | Countdown boss bar with one bar per viewer, safe on Folia. |
| `toast-notification` | 1.21.4+ | Advancement-based popups with item model icons and frame styles. |
| `hologram` | 1.21.4+ | Floating `TextDisplay` text that can be updated and moved. |
| `animation` | 1.21.4+ | Frame animator for titles, actionbars and icons. |
| `text-fx` | any | Gradient, rainbow, shimmer, pulse and typewriter effects. |

### D. Utilities

| Component | Minecraft | Description |
| --- | --- | --- |
| `item-builder` | 1.21.4+ | Fluent `ItemStack` builder with item models, tooltip styles and glint overrides. |
| `head-util` | 1.21.4+ | Player heads from names or skin URLs, using `PlayerProfile` textures. |
| `scheduler` | 1.21.4+ | Folia-safe `Tasks` helpers for player, region, global and async work. |

### E. Resource pack GUIs

| Component | Minecraft | Description |
| --- | --- | --- |
| `resource-pack` | 1.21.4+ | Sends the generated pack on join, verifies its SHA-1, and tracks who loaded it. |
| `texture-gui` | 1.21.4+ | Title plates, textured icons and filler, plus a `TexturedMenu` base class. Falls back to vanilla without the pack. |

## Resource pack

`craftcn pack` generates a pack in the colours of your theme:

- `font/gui.json`: a `craftcn:gui` font. Title plates are bitmap glyphs, and text is moved with invisible shift glyphs.
- `textures/item/icon_*.png` and `items/icon_*.json`: item models for next, back, confirm, cancel, info, close and
  filler, which work with `ItemBuilder.model(...)`.
- `pack.mcmeta`: the `min_format`/`max_format` range covering Minecraft 1.21.4 through your target version.

Upload the zip, then configure `ResourcePackService` with its URL and SHA-1:

```java
var packs = new ResourcePackService(this, URI.create(url), sha1, false, Component.text("GUI pack"));
packs.register();
GuiIcons.setPackCheck(packs::hasPack); // players without the pack get vanilla icons
```

Players without the pack see the vanilla fallbacks, so nothing breaks when the pack is missing.

## Themes

| Theme | Look |
| --- | --- |
| `default` | Gold, grey and cyan with clear contrast. |
| `dark` | Near-black with purple accents. |
| `ocean` | Navy and teal. |
| `neon` | Magenta and cyan on a near-black base. |
| `parchment` | Warm brown and gold. |

A theme sets the palette, the filler and button materials, and the sounds. `UITheme.java` is generated from it, and
`craftcn pack` uses the same palette for the textures.

## Architecture

- **The CLI (Rust).** A single binary. Registry source resolution, dependency resolution, install and remove,
  pack generation, and diagnostics.
- **The registry.** `registry/index.json` lists components, with their dependencies, Minecraft minimum, and whether
  they use the resource pack. `registry/themes.json` lists themes. Component sources live in
  `registry/components/<name>/`, under the `com.craftcn` namespace, and are rewritten to your package on install.
- **The primitives (Java).** Paper API and Adventure only. No NMS, no shaded libraries.

## Verifying changes

```bash
cargo test                       # CLI and registry integrity tests
bash scripts/verify-components.sh  # installs every component into a Paper 26.2 project and compiles it
```

The verification script needs a JDK 25 and Maven. CI runs both checks on every push to `develop`.

For 0.2.0, the Java was also run on a headless Paper 26.2 server with a throwaway self-test: menu layouts, toast JSON
loading, scheduling, holograms and boss bar countdowns all worked there. That self-test isn't in the repository yet.
Player-facing flows (clicks, dialogs, toasts on screen, resource pack rendering) still need a check in a game client.
See [CHANGELOG.md](CHANGELOG.md).

## Development

```bash
cargo build --release
cargo test
CRAFTCN_REGISTRY=$PWD/registry cargo run -- list
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add a component.

## License

GNU GPLv3

## Support

- GitHub Issues: https://github.com/modpotato/craftcn/issues

## Acknowledgments

- Inspired by [shadcn/ui](https://ui.shadcn.com/)
- Built with [Rust](https://www.rust-lang.org/)
- Powered by [PaperMC](https://papermc.io/) and [Adventure](https://docs.advntr.dev/)
- Resource pack GUI ideas from the PaperMC community and projects such as Praeter and Fancy UI
