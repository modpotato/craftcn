package com.craftcn.ui.chat;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.event.ClickEvent;
import net.kyori.adventure.text.event.HoverEvent;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.entity.Player;

public class InteractiveMessage {
    
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
        return new InteractiveMessage(message.decorate(net.kyori.adventure.text.format.TextDecoration.BOLD));
    }
    
    public InteractiveMessage italic() {
        return new InteractiveMessage(message.decorate(net.kyori.adventure.text.format.TextDecoration.ITALIC));
    }
    
    public InteractiveMessage underline() {
        return new InteractiveMessage(message.decorate(net.kyori.adventure.text.format.TextDecoration.UNDERLINED));
    }
    
    public InteractiveMessage strikethrough() {
        return new InteractiveMessage(message.decorate(net.kyori.adventure.text.format.TextDecoration.STRIKETHROUGH));
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
    
    public Component build() {
        return message;
    }
    
    public void send(Player player) {
        player.sendMessage(message);
    }
    
    public void sendBroadcast() {
        org.bukkit.Bukkit.broadcast(message);
    }
    
    public String toString() {
        return net.kyori.adventure.text.serializer.legacy.LegacyComponentSerializer.legacySection()
            .serialize(message);
    }
}
