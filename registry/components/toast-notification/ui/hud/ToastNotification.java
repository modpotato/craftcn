package com.craftcn.ui.hud;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.inventory.ItemStack;

import java.util.function.Consumer;

public class ToastNotification {
    
    private final Player player;
    private Component title;
    private Component description;
    private Material icon;
    private Consumer<Player> onClick;
    private Consumer<Player> onHover;
    
    public ToastNotification(Player player) {
        this.player = player;
        this.title = Component.empty();
        this.description = Component.empty();
        this.icon = Material.BOOK;
    }
    
    public static ToastNotification create(Player player) {
        return new ToastNotification(player);
    }
    
    public ToastNotification title(String title) {
        return title(Component.text(title));
    }
    
    public ToastNotification title(Component title) {
        this.title = title;
        return this;
    }
    
    public ToastNotification description(String description) {
        return description(Component.text(description));
    }
    
    public ToastNotification description(Component description) {
        this.description = description;
        return this;
    }
    
    public ToastNotification icon(Material icon) {
        this.icon = icon;
        return this;
    }
    
    public ToastNotification onClick(Consumer<Player> callback) {
        this.onClick = callback;
        return this;
    }
    
    public ToastNotification onHover(Consumer<Player> callback) {
        this.onHover = callback;
        return this;
    }
    
    public void send() {
        ItemStack item = new ItemStack(icon);
        
        player.showTitle(
            net.kyori.adventure.title.Title.title(
                title.colorIfAbsent(NamedTextColor.WHITE),
                description.colorIfAbsent(NamedTextColor.GRAY),
                net.kyori.adventure.title.Title.Times.times(
                    java.time.Duration.ofMillis(500),
                    java.time.Duration.ofMillis(3000),
                    java.time.Duration.ofMillis(500)
                )
            )
        );
        
        player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
    }
    
    public void sendWithIcon() {
        ItemStack item = new ItemStack(icon);
        
        Component fullMessage = Component.text()
            .append(Component.text("[", NamedTextColor.GRAY))
            .append(item.displayName())
            .append(Component.text("] ", NamedTextColor.GRAY))
            .append(title)
            .build();
        
        player.sendMessage(fullMessage);
        send();
    }
    
    public static void success(Player player, String message) {
        create(player)
            .title("Success")
            .description(message)
            .icon(Material.GREEN_WOOL)
            .send();
    }
    
    public static void error(Player player, String message) {
        create(player)
            .title("Error")
            .description(message)
            .icon(Material.RED_WOOL)
            .send();
    }
    
    public static void info(Player player, String message) {
        create(player)
            .title("Info")
            .description(message)
            .icon(Material.BLUE_WOOL)
            .send();
    }
    
    public static void warning(Player player, String message) {
        create(player)
            .title("Warning")
            .description(message)
            .icon(Material.YELLOW_WOOL)
            .send();
    }
}
