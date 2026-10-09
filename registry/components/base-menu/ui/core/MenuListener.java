package com.craftcn.ui.core;

import org.bukkit.Bukkit;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.event.inventory.InventoryDragEvent;
import org.bukkit.event.inventory.PrepareAnvilEvent;
import org.bukkit.plugin.Plugin;

import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

/**
 * One listener per plugin that routes inventory events to the {@link BaseMenu} owning the inventory.
 * Menus never register listeners of their own, so opening many menus costs nothing and nothing leaks.
 */
public final class MenuListener implements Listener {

    private static final Set<Plugin> REGISTERED = ConcurrentHashMap.newKeySet();

    private MenuListener() {
    }

    static void register(Plugin plugin) {
        if (REGISTERED.add(plugin)) {
            Bukkit.getPluginManager().registerEvents(new MenuListener(), plugin);
        }
    }

    @EventHandler
    public void onClick(InventoryClickEvent event) {
        if (event.getView().getTopInventory().getHolder() instanceof BaseMenu menu) {
            menu.handleClick(event);
        }
    }

    @EventHandler
    public void onDrag(InventoryDragEvent event) {
        if (event.getView().getTopInventory().getHolder() instanceof BaseMenu menu) {
            menu.handleDrag(event);
        }
    }

    @EventHandler
    public void onClose(InventoryCloseEvent event) {
        if (event.getInventory().getHolder() instanceof BaseMenu menu) {
            menu.handleClose();
        }
    }

    @EventHandler
    public void onPrepareAnvil(PrepareAnvilEvent event) {
        if (event.getInventory().getHolder() instanceof BaseMenu menu) {
            menu.handlePrepareAnvil(event);
        }
    }
}
