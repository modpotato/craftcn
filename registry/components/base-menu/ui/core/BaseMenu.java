package com.craftcn.ui.core;

import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.HandlerList;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.event.inventory.InventoryDragEvent;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.Plugin;

public abstract class BaseMenu implements Listener {
    
    protected final Plugin plugin;
    protected final Player player;
    protected final String title;
    protected final int size;
    protected Inventory inventory;
    
    private boolean cancelled;
    
    public BaseMenu(Plugin plugin, Player player, String title, int size) {
        this.plugin = plugin;
        this.player = player;
        this.title = title;
        this.size = size;
        this.cancelled = false;
    }
    
    public void open() {
        inventory = Bukkit.createInventory(null, size, Component.text(title));
        build();
        
        Bukkit.getPluginManager().registerEvents(this, plugin);
        player.openInventory(inventory);
    }
    
    public void close() {
        if (!cancelled) {
            cancelled = true;
            HandlerList.unregisterAll(this);
            if (player.getOpenInventory().getTopInventory() == inventory) {
                player.closeInventory();
            }
        }
    }
    
    protected abstract void build();
    
    protected abstract void onClick(InventoryClickEvent event);
    
    protected void onClose(InventoryCloseEvent event) {
    }
    
    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (event.getInventory() != inventory) {
            return;
        }
        
        if (cancelled) {
            return;
        }
        
        event.setCancelled(true);
        onClick(event);
    }
    
    @EventHandler
    public void onInventoryDrag(InventoryDragEvent event) {
        if (event.getInventory() != inventory) {
            return;
        }
        
        if (cancelled) {
            return;
        }
        
        event.setCancelled(true);
    }
    
    @EventHandler
    public void onInventoryClose(InventoryCloseEvent event) {
        if (event.getInventory() != inventory) {
            return;
        }
        
        if (cancelled) {
            return;
        }
        
        cancelled = true;
        HandlerList.unregisterAll(this);
        onClose(event);
    }
    
    public Player getPlayer() {
        return player;
    }
    
    public Inventory getInventory() {
        return inventory;
    }
    
    protected ItemStack createFiller() {
        return new ItemStack(UITheme.FILLER_GLASS);
    }
    
    protected void fillEmptySlots() {
        ItemStack filler = createFiller();
        for (int i = 0; i < inventory.getSize(); i++) {
            if (inventory.getItem(i) == null || inventory.getItem(i).getType() == Material.AIR) {
                inventory.setItem(i, filler);
            }
        }
    }
    
    protected void fillRange(int start, int end) {
        ItemStack filler = createFiller();
        for (int i = start; i < end; i++) {
            if (i >= 0 && i < inventory.getSize()) {
                inventory.setItem(i, filler);
            }
        }
    }
}
