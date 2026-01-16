package com.craftcn.ui.chat;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.HandlerList;
import org.bukkit.event.Listener;
import org.bukkit.event.player.AsyncPlayerChatEvent;
import org.bukkit.plugin.Plugin;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

public class ChatPrompt implements Listener {
    
    private final Plugin plugin;
    private final Player player;
    private final String prompt;
    private final Consumer<String> callback;
    private final long timeout;
    private final TimeUnit timeoutUnit;
    
    private CompletableFuture<String> future;
    private boolean active;
    private int taskId;
    
    public ChatPrompt(Plugin plugin, Player player, String prompt, Consumer<String> callback) {
        this(plugin, player, prompt, callback, 30, TimeUnit.SECONDS);
    }
    
    public ChatPrompt(Plugin plugin, Player player, String prompt, Consumer<String> callback, 
                      long timeout, TimeUnit timeoutUnit) {
        this.plugin = plugin;
        this.player = player;
        this.prompt = prompt;
        this.callback = callback;
        this.timeout = timeout;
        this.timeoutUnit = timeoutUnit;
        this.active = false;
    }
    
    public void start() {
        active = true;
        
        player.sendMessage(Component.text(prompt, NamedTextColor.YELLOW));
        player.sendMessage(Component.text("Type your response in chat", NamedTextColor.GRAY));
        
        Bukkit.getPluginManager().registerEvents(this, plugin);
        
        future = new CompletableFuture<>();
        future.orTimeout(timeout, timeoutUnit)
            .exceptionally(ex -> {
                cancel();
                return null;
            });
        
        taskId = Bukkit.getScheduler().runTaskLater(plugin, this::cancel, 
            timeoutUnit.toSeconds(timeout) * 20L).getTaskId();
    }
    
    public CompletableFuture<String> getResponse() {
        if (future == null) {
            future = new CompletableFuture<>();
        }
        return future;
    }
    
    @EventHandler
    public void onPlayerChat(AsyncPlayerChatEvent event) {
        if (!active || event.getPlayer() != player) {
            return;
        }
        
        event.setCancelled(true);
        
        String message = event.getMessage();
        
        if (message.equalsIgnoreCase("cancel")) {
            player.sendMessage(Component.text("Prompt cancelled", NamedTextColor.RED));
            cancel();
            return;
        }
        
        complete(message);
    }
    
    private void complete(String message) {
        if (!active) {
            return;
        }
        
        active = false;
        HandlerList.unregisterAll(this);
        Bukkit.getScheduler().cancelTask(taskId);
        
        if (callback != null) {
            callback.accept(message);
        }
        
        if (future != null && !future.isDone()) {
            future.complete(message);
        }
        
        player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
    }
    
    public void cancel() {
        if (!active) {
            return;
        }
        
        active = false;
        HandlerList.unregisterAll(this);
        Bukkit.getScheduler().cancelTask(taskId);
        
        if (future != null && !future.isDone()) {
            future.cancel(true);
        }
        
        player.playSound(player.getLocation(), UITheme.ERROR, 1.0f, 1.0f);
    }
    
    public boolean isActive() {
        return active;
    }
}
