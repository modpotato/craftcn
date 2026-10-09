# CraftCN showcase

A small Paper plugin that uses most CraftCN components together. It is also the compile check for the
registry: every component must build against the latest stable Paper API.

## Build it

```bash
craftcn init --package com.example.showcase --minecraft 26.2 --yes
craftcn add paginated-menu
craftcn add dialog-menu toast-notification animation bossbar-timer chat-prompt interactive-message hologram
craftcn add texture-gui resource-pack text-fx
craftcn pack --zip                # optional: generates the resource pack
mvn package
```

Copy `src/main/java/com/example/showcase` and `src/main/resources` from this folder into the project
(`craftcn init` creates the package directory), then run `mvn package` and drop the jar in `plugins/`.

## Try it in game

| Command | What it shows |
| --- | --- |
| `/showcase shop` | Paginated menu. With the resource pack, the title gets a gold plate. |
| `/showcase form` | Native dialog with text, toggle and number inputs (Minecraft 1.21.6+). |
| `/showcase toast` | Advancement-style toast popup. |
| `/showcase fx` | Shimmering actionbar animation for five seconds. |
| `/showcase timer` | 30-second boss bar countdown. |
| `/showcase prompt` | Waits for your next chat message. |
| `/showcase hologram` | Floating gradient text above you. |

Joining the server also sends a clickable hint that opens the shop without running a command.
