package com.craftcn.ui.chat;

import com.craftcn.ui.UITheme;
import com.craftcn.ui.util.Tasks;
import io.papermc.paper.event.player.AsyncChatEvent;
import io.papermc.paper.threadedregions.scheduler.ScheduledTask;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import net.kyori.adventure.text.serializer.plain.PlainTextComponentSerializer;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.EventPriority;
import org.bukkit.event.HandlerList;
import org.bukkit.event.Listener;
import org.bukkit.plugin.Plugin;

import java.time.Duration;
import java.util.concurrent.CompletableFuture;
import java.util.function.Consumer;

/**
 * Captures the next chat message a player sends. The message is hidden from chat, and the callback runs on the
 * player's thread, so it is safe to touch the player and their inventory. Typing {@code cancel} ends the prompt.
 * <p>
 * Chat events are asynchronous, so the message is handed back to the player's thread before anything else runs.
 */
public class ChatPrompt implements Listener {

    public static final String CANCEL_WORD = "cancel";
    public static final String CANCELLED_MESSAGE = "Prompt cancelled";
    public static final String TIMED_OUT_MESSAGE = "Prompt timed out";

    private final Plugin plugin;
    private final Player player;
    private final Component prompt;
    private final Consumer<String> callback;
    private final Duration timeout;

    private final CompletableFuture<String> future = new CompletableFuture<>();
    private boolean active;
    private ScheduledTask timeoutTask;

    public ChatPrompt(Plugin plugin, Player player, String prompt, Consumer<String> callback) {
        this(plugin, player, Component.text(prompt), callback, Duration.ofSeconds(30));
    }

    public ChatPrompt(Plugin plugin, Player player, Component prompt, Consumer<String> callback, Duration timeout) {
        this.plugin = plugin;
        this.player = player;
        this.prompt = prompt;
        this.callback = callback;
        this.timeout = timeout;
    }

    /** Starts listening for the player's next message. Call from the player's thread. */
    public void start() {
        if (active) {
            return;
        }
        active = true;

        Bukkit.getPluginManager().registerEvents(this, plugin);

        player.sendMessage(prompt.color(NamedTextColor.YELLOW));
        player.sendMessage(Component.text("Type your answer in chat, or '" + CANCEL_WORD + "' to cancel", NamedTextColor.GRAY));

        timeoutTask = Tasks.later(plugin, player, timeout.toMillis() / 50, this::timeOut);
    }

    /** Completes with the message, or is cancelled when the prompt is cancelled or times out. */
    public CompletableFuture<String> response() {
        return future;
    }

    public boolean isActive() {
        return active;
    }

    /** Cancels the prompt. Call from the player's thread. */
    public void cancel() {
        if (!active) {
            return;
        }
        stop();
        player.sendMessage(Component.text(CANCELLED_MESSAGE, NamedTextColor.RED));
        player.playSound(player.getLocation(), UITheme.ERROR, 1.0f, 1.0f);
        future.cancel(false);
    }

    @EventHandler(priority = EventPriority.LOWEST)
    public void onChat(AsyncChatEvent event) {
        if (!active || !event.getPlayer().equals(player)) {
            return;
        }

        event.setCancelled(true);
        String message = PlainTextComponentSerializer.plainText().serialize(event.message());

        Tasks.run(plugin, player, () -> finish(message));
    }

    private void finish(String message) {
        if (!active) {
            return;
        }
        if (message.trim().equalsIgnoreCase(CANCEL_WORD)) {
            cancel();
            return;
        }

        stop();
        player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
        callback.accept(message);
        future.complete(message);
    }

    private void timeOut() {
        if (!active) {
            return;
        }
        stop();
        player.sendMessage(Component.text(TIMED_OUT_MESSAGE, NamedTextColor.RED));
        future.cancel(false);
    }

    private void stop() {
        active = false;
        HandlerList.unregisterAll(this);
        if (timeoutTask != null) {
            timeoutTask.cancel();
        }
    }
}
