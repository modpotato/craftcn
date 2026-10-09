# Changelog

## 0.2.0 - Unreleased

Targets Paper 26.2 (latest stable), with support back to Minecraft 1.21.4. Components are verified against the Paper
API in CI.

### Breaking changes

- **Components were rewritten.** Menus now use a holder-based `BaseMenu` with `Button`s and one listener per plugin.
  `ConfirmationMenu`, `PaginatedMenu`, `SelectionMenu`, `FormMenu`, `ChatPrompt` and friends keep their main
  constructors and methods, but a few were renamed or changed type. Reinstall them with `craftcn add <name> --force`
  and check your call sites.
- **`craftcn add` keeps existing files.** Use `--force` to overwrite. Before, files were always overwritten.
- **`craftcn update` takes no flags.** The old `--force` flag only printed a hint.
- **`InteractiveMessage.toString()` is removed.** Serialise the `Component` yourself if you need legacy text.
- **`craftcn.json` has a `minecraft` field**, defaulting to `26.2`. Existing projects are read as-is.

### Fixed

- **`UITheme` failed to load on any hex colour.** The theme parser caught the wrong exception type, so every plugin
  generated from a theme would have crashed on startup. Found by a headless Paper 26.2 server run.
- **Installed files referenced packages that don't exist.** Only the `package` line was rewritten, so imports and
  qualified names kept the `com.craftcn` namespace, and the theme import was not added where it was needed.
- **`craftcn init` could not render themes with sounds.** The template used a filter that was never registered with
  minijinja, and iterating the sounds map was wrong. Both are fixed.
- **`FormMenu` never returned input.** It waited for chat and passed `null` to its callback. It's now a real anvil
  input.
- **`ConfirmationMenu` didn't compile.** It imported `Runnable` from the wrong package.
- **`ChatPrompt` ran player callbacks on the chat thread.** Callbacks now run on the player's thread.
- **`context` missed classes with `extends` or `implements`.** The parser tracks braces and reads public and protected
  members only, unless `--verbose`.
- **`craftcn theme apply` told you to delete the project.** It now regenerates `UITheme.java` in place.
- **The `update` command pointed to a `remove` command that didn't exist.**

### Added

- **Components:** `dialog-menu` (native dialogs, Minecraft 1.21.6+), `hologram`, `animation`, `text-fx`, `scheduler`
  (Folia-safe `Tasks`), `resource-pack` (`ResourcePackService` with SHA-1 verification), `texture-gui` (title plates,
  textured icons, `TexturedMenu`). `toast-notification` is now advancement-based. `head-util` uses `PlayerProfile`
  skins. `bossbar-timer` and `chat-prompt` are Folia-safe.
- **CLI:** `craftcn remove`, `craftcn pack` (procedural resource pack in your theme's colours, `--zip` with SHA-1),
  `craftcn doctor`, `craftcn init --minecraft --yes`.
- **Registry:** `CRAFTCN_REGISTRY` points the CLI at a local checkout. `CRAFTCN_REGISTRY_URL` sets a remote base URL.
  Remote files are cached. Categories are derived from the component list. Registry paths are checked against path
  traversal.
- **Themes:** `neon` and `parchment`.
- **Examples:** `examples/showcase`, a Paper plugin that uses most components.
- **Verification:** `scripts/verify-components.sh` and a CI job that compiles every component against Paper 26.2 with
  `-Xlint:all`.

### Changed

- Components use item models, tooltip styles and glint overrides, not legacy custom model data.
- Sounds are referenced as `Sound.CONSTANT` and checked at compile time, instead of `Sound.valueOf` at runtime.
- The anvil rename text is read through `AnvilView`, replacing the deprecated `AnvilInventory` method.
- The CLI only logs warnings and errors by default.
