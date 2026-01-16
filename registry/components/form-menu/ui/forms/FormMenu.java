package com.craftcn.ui.forms;

import com.craftcn.ui.util.ItemBuilder;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.HandlerList;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.Plugin;

import java.util.concurrent.CompletableFuture;
import java.util.function.Consumer;

public class FormMenu implements Listener {
    
    private final Plugin plugin;
    private final Player player;
    private final String title;
    private final String prompt;
    private final Consumer<String> callback;
    
    private Inventory inventory;
    private CompletableFuture<String> future;
    private boolean closed;
    
    public FormMenu(Plugin plugin, Player player, String title, String prompt, Consumer<String> callback) {
        this.plugin = plugin;
        this.player = player;
        this.title = title;
        this.prompt = prompt;
        this.callback = callback;
        this.closed = false;
    }
    
    public void open() {
        inventory = Bukkit.createInventory(null, 45, Component.text(title));
        
        ItemStack promptItem = new ItemBuilder(Material.PAPER)
            .name("Enter your response:")
            .lore(prompt)
            .build();
        
        ItemStack submitItem = new ItemBuilder(Material.GREEN_WOOL)
            .name("Submit")
            .lore("Click to submit your input")
            .build();
        
        ItemStack cancelItem = new ItemBuilder(Material.RED_WOOL)
            .name("Cancel")
            .lore("Click to cancel")
            .build();
        
        inventory.setItem(13, promptItem);
        inventory.setItem(29, submitItem);
        inventory.setItem(33, cancelItem);
        
        for (int i = 0; i < 45; i++) {
            if (inventory.getItem(i) == null || inventory.getItem(i).getType() == Material.AIR) {
                inventory.setItem(i, new ItemBuilder(Material.GRAY_STAINED_GLASS_PANE).name(" ").build());
            }
        }
        
        Bukkit.getPluginManager().registerEvents(this, plugin);
        player.openInventory(inventory);
        
        player.sendMessage(Component.text("Type your response in chat", NamedTextColor.YELLOW));
        player.sendMessage(Component.text("Or click 'Submit' when done", NamedTextColor.GRAY));
    }
    
    public CompletableFuture<String> getInput() {
        if (future == null) {
            future = new CompletableFuture<>();
        }
        return future;
    }
    
    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (event.getInventory() != inventory || closed) {
            return;
        }
        
        event.setCancelled(true);
        
        int slot = event.getSlot();
        if (slot == 29) {
            handleSubmit();
        } else if (slot == 33) {
            handleCancel();
        }
    }
    
    @EventHandler
    public void onInventoryClose(InventoryCloseEvent event) {
        if (event.getInventory() != inventory || closed) {
            return;
        }
        
        closed = true;
        HandlerList.unregisterAll(this);
        
        if (future != null && !future.isDone()) {
            future.cancel(true);
        }
    }
    
    private void handleSubmit() {
        closed = true;
        HandlerList.unregisterAll(this);
        player.closeInventory();
        
        if (callback != null) {
            callback.accept(null);
        }
        
        if (future != null) {
            future.complete(null);
        }
        
        player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
    }
    
    private void handleCancel() {
        closed = true;
        HandlerList.unregisterAll(this);
        player.closeInventory();
        
        if (future != null) {
            future.cancel(true);
        }
        
        player.playSound(player.getLocation(), UITheme.CLOSE, 1.0f, 1.0f);
    }
    
    public void close() {
        if (!closed) {
            closed = true;
            HandlerList.unregisterAll(this);
            if (player.getOpenInventory().getTopInventory() == inventory) {
                player.closeInventory();
            }
        }
    }
}
