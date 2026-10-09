package com.craftcn.ui.hud;

import com.craftcn.ui.util.Tasks;
import net.kyori.adventure.text.Component;
import org.bukkit.Color;
import org.bukkit.Location;
import org.bukkit.entity.Display;
import org.bukkit.entity.TextDisplay;
import org.bukkit.plugin.Plugin;

import java.util.concurrent.CompletableFuture;

/**
 * Floating text built on a {@link TextDisplay} entity. The text always faces the viewer and can change at any time.
 * <p>
 * Entities belong to a region, so every change is scheduled onto the region that owns the hologram. The entity is
 * not persisted, so it does not survive a restart; spawn holograms again in {@code onEnable}.
 */
public final class Hologram {

    private static final Color TRANSPARENT = Color.fromARGB(0, 0, 0, 0);

    private final Plugin plugin;
    private final TextDisplay display;
    private volatile Location location;

    private Hologram(Plugin plugin, TextDisplay display, Location location) {
        this.plugin = plugin;
        this.display = display;
        this.location = location;
    }

    /** Spawns a hologram at {@code location}. The future completes on that location's region. */
    public static CompletableFuture<Hologram> spawn(Plugin plugin, Location location, Component text) {
        CompletableFuture<Hologram> future = new CompletableFuture<>();

        Tasks.at(plugin, location, () -> {
            TextDisplay display = location.getWorld().spawn(location, TextDisplay.class, entity -> {
                entity.text(text);
                entity.setBillboard(Display.Billboard.CENTER);
                entity.setAlignment(TextDisplay.TextAlignment.CENTER);
                entity.setShadowed(true);
                entity.setSeeThrough(false);
                entity.setDefaultBackground(false);
                entity.setBackgroundColor(TRANSPARENT);
                entity.setPersistent(false);
            });
            future.complete(new Hologram(plugin, display, location));
        });

        return future;
    }

    /** Replaces the text. Safe to call from any thread. */
    public void text(Component text) {
        Tasks.at(plugin, location, () -> display.text(text));
    }

    /** Moves the hologram. Safe to call from any thread. */
    public void move(Location to) {
        this.location = to;
        display.teleportAsync(to);
    }

    /** Removes the hologram from the world. */
    public void remove() {
        Tasks.at(plugin, location, display::remove);
    }
}
