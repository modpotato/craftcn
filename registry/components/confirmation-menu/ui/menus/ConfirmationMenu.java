package com.craftcn.ui.menus;

import com.craftcn.ui.core.BaseMenu;
import com.craftcn.ui.core.UITheme;
import com.craftcn.ui.util.ItemBuilder;
import org.bukkit.entity.Player;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.inventory.ItemStack;
import org.bukkit.Material;
import org.bukkit.plugin.Plugin;

import java.util.function.Runnable;

public class ConfirmationMenu extends BaseMenu {
    
    private Runnable onConfirm;
    private Runnable onDeny;
    private String message;
    
    public ConfirmationMenu(Plugin plugin, Player player, String title, String message) {
        super(plugin, player, title, 27);
        this.message = message;
    }
    
    public void setOnConfirm(Runnable callback) {
        this.onConfirm = callback;
    }
    
    public void setOnDeny(Runnable callback) {
        this.onDeny = callback;
    }
    
    @Override
    protected void build() {
        inventory.clear();
        
        ItemStack messageItem = ItemBuilder.from(Material.PAPER)
            .name("Confirmation")
            .addLore(message)
            .build();
        
        ItemStack confirmButton = ItemBuilder.from(Material.GREEN_WOOL)
            .name("Confirm")
            .addLore("Click to confirm")
            .glow()
            .build();
        
        ItemStack denyButton = ItemBuilder.from(Material.RED_WOOL)
            .name("Cancel")
            .addLore("Click to cancel")
            .build();
        
        inventory.setItem(13, messageItem);
        inventory.setItem(11, confirmButton);
        inventory.setItem(15, denyButton);
        
        fillEmptySlots();
    }
    
    @Override
    protected void onClick(InventoryClickEvent event) {
        int slot = event.getSlot();
        
        if (slot == 11) {
            if (onConfirm != null) {
                onConfirm.run();
            }
            player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
            close();
        } else if (slot == 15) {
            if (onDeny != null) {
                onDeny.run();
            }
            player.playSound(player.getLocation(), UITheme.CLOSE, 1.0f, 1.0f);
            close();
        }
    }
}
