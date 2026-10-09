package com.craftcn.ui.chat;

import net.kyori.adventure.audience.Audience;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.event.ClickEvent;
import net.kyori.adventure.text.event.HoverEvent;
import net.kyori.adventure.text.format.NamedTextColor;
import net.kyori.adventure.text.format.TextDecoration;

import java.time.Duration;
import java.util.function.Consumer;

/**
 * Immutable builder for chat messages with hover text and clicks.
 * <p>
 * {@link #clickCallback(Consumer)} runs server-side code when a player clicks, without exposing a command.
 * Callback buttons expire after {@code callbackLifetime} (ten minutes by default).
 */
public class InteractiveMessage {

    private static final Duration CALLBACK_LIFETIME = Duration.ofMinutes(10);

    private final Component message;

    private InteractiveMessage(Component message) {
        this.message = message;
    }

    public static InteractiveMessage create() {
        return new InteractiveMessage(Component.empty());
    }

    public static InteractiveMessage from(String text) {
        return new InteractiveMessage(Component.text(text));
    }

    public InteractiveMessage append(String text) {
        return new InteractiveMessage(message.append(Component.text(text)));
    }

    public InteractiveMessage append(Component component) {
        return new InteractiveMessage(message.append(component));
    }

    public InteractiveMessage color(NamedTextColor color) {
        return new InteractiveMessage(message.color(color));
    }

    public InteractiveMessage bold() {
        return new InteractiveMessage(message.decorate(TextDecoration.BOLD));
    }

    public InteractiveMessage italic() {
        return new InteractiveMessage(message.decorate(TextDecoration.ITALIC));
    }

    public InteractiveMessage underline() {
        return new InteractiveMessage(message.decorate(TextDecoration.UNDERLINED));
    }

    public InteractiveMessage strikethrough() {
        return new InteractiveMessage(message.decorate(TextDecoration.STRIKETHROUGH));
    }

    public InteractiveMessage hover(String text) {
        return hover(Component.text(text));
    }

    public InteractiveMessage hover(Component component) {
        return new InteractiveMessage(message.hoverEvent(HoverEvent.showText(component)));
    }

    public InteractiveMessage clickCommand(String command) {
        return new InteractiveMessage(message.clickEvent(ClickEvent.runCommand(command)));
    }

    public InteractiveMessage clickSuggest(String command) {
        return new InteractiveMessage(message.clickEvent(ClickEvent.suggestCommand(command)));
    }

    public InteractiveMessage clickUrl(String url) {
        return new InteractiveMessage(message.clickEvent(ClickEvent.openUrl(url)));
    }

    public InteractiveMessage clickCopy(String text) {
        return new InteractiveMessage(message.clickEvent(ClickEvent.copyToClipboard(text)));
    }

    /**
     * Runs {@code callback} on the server when a player clicks the message. Nothing is exposed to the client, so
     * this is safer than a command for actions that should only come from this message.
     */
    public InteractiveMessage clickCallback(Consumer<Audience> callback) {
        return new InteractiveMessage(message.clickEvent(ClickEvent.callback(
                callback::accept,
                options -> options.lifetime(CALLBACK_LIFETIME))));
    }

    public Component build() {
        return message;
    }

    public void send(Audience audience) {
        audience.sendMessage(message);
    }

    public void sendBroadcast() {
        org.bukkit.Bukkit.broadcast(message);
    }
}
