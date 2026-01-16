package com.craftcn.ui.menus;

import com.craftcn.ui.core.BaseMenu;
import org.bukkit.entity.Player;
import org.bukkit.Material;

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
        
        ItemStack messageItem = new ItemBuilder(Material.PAPER)
            .name("Confirmation")
            .lore(message)
            .build();
        
        ItemStack confirmButton = new ItemBuilder(Material.GREEN_WOOL)
            .name("Confirm")
            .lore("Click to confirm")
            .glow()
            .build();
        
        ItemStack denyButton = new ItemBuilder(Material.RED_WOOL)
            .name("Cancel")
            .lore("Click to cancel")
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
