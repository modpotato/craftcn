package com.craftcn.ui.pack;

import com.craftcn.ui.util.Tasks;
import net.kyori.adventure.resource.ResourcePackInfo;
import net.kyori.adventure.resource.ResourcePackRequest;
import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.event.player.PlayerQuitEvent;
import org.bukkit.event.player.PlayerResourcePackStatusEvent;
import org.bukkit.event.player.PlayerResourcePackStatusEvent.Status;
import org.bukkit.plugin.Plugin;

import java.net.URI;
import java.util.Locale;
import java.util.Set;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.BiConsumer;

/**
 * Sends a resource pack to players when they join, and tracks who has loaded it.
 * <p>
 * Host the zip made by {@code craftcn pack --zip}, then pass its URL and SHA-1 here. The client checks the hash,
 * so a file that was changed on the way is rejected. The pack id is derived from the URL, so a new upload to the
 * same URL replaces the old pack.
 */
public final class ResourcePackService implements Listener {

    private static final long SEND_DELAY_TICKS = 10;

    private final Plugin plugin;
    private final ResourcePackInfo info;
    private final Component prompt;
    private final boolean required;
    private final Set<UUID> loaded = ConcurrentHashMap.newKeySet();
    private volatile BiConsumer<Player, Status> statusHandler = (player, status) -> { };

    /**
     * @param url      where the zip is hosted
     * @param sha1Hex  SHA-1 of the zip, 40 hex characters
     * @param required if true, players who decline are disconnected
     * @param prompt   message shown on the download prompt
     * @throws IllegalArgumentException if {@code sha1Hex} is not a SHA-1 hash
     */
    public ResourcePackService(Plugin plugin, URI url, String sha1Hex, boolean required, Component prompt) {
        if (sha1Hex == null || !sha1Hex.matches("[0-9a-fA-F]{40}")) {
            throw new IllegalArgumentException("sha1Hex must be 40 hexadecimal characters");
        }

        this.plugin = plugin;
        this.info = ResourcePackInfo.resourcePackInfo()
                .id(UUID.nameUUIDFromBytes(url.toString().getBytes()))
                .uri(url)
                .hash(sha1Hex.toLowerCase(Locale.ROOT))
                .build();
        this.prompt = prompt;
        this.required = required;
    }

    /** Registers the join and status listeners. Call once from {@code onEnable}. */
    public void register() {
        Bukkit.getPluginManager().registerEvents(this, plugin);
    }

    /** Receives every status change, such as accepted, declined or loaded. Runs on the player's thread. */
    public void onStatus(BiConsumer<Player, Status> handler) {
        this.statusHandler = handler;
    }

    /** Sends the pack to {@code player} now. Call from the player's thread. */
    public void send(Player player) {
        player.sendResourcePacks(ResourcePackRequest.resourcePackRequest()
                .packs(info)
                .prompt(prompt)
                .required(required)
                .replace(true)
                .build());
    }

    /** True once the player has loaded the pack. Use it to pick pack or vanilla visuals. */
    public boolean hasPack(Player player) {
        return loaded.contains(player.getUniqueId());
    }

    @EventHandler
    public void onJoin(PlayerJoinEvent event) {
        Player player = event.getPlayer();
        Tasks.later(plugin, player, SEND_DELAY_TICKS, () -> send(player));
    }

    @EventHandler
    public void onPackStatus(PlayerResourcePackStatusEvent event) {
        Player player = event.getPlayer();
        Status status = event.getStatus();

        if (status == Status.SUCCESSFULLY_LOADED) {
            loaded.add(player.getUniqueId());
        } else if (status == Status.DECLINED || status == Status.FAILED_DOWNLOAD
                || status == Status.INVALID_URL || status == Status.DISCARDED) {
            loaded.remove(player.getUniqueId());
        }

        statusHandler.accept(player, status);
    }

    @EventHandler
    public void onQuit(PlayerQuitEvent event) {
        loaded.remove(event.getPlayer().getUniqueId());
    }
}
