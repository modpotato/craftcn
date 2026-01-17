package com.craftcn.ui.menus;

import com.craftcn.ui.core.BaseMenu;
import com.craftcn.ui.core.UITheme;
import com.craftcn.ui.util.ItemBuilder;
import org.bukkit.entity.Player;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.inventory.ItemStack;
import org.bukkit.Material;
import org.bukkit.plugin.Plugin;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.function.BiConsumer;
import java.util.function.Function;

public class SelectionMenu<T> extends BaseMenu {
    
    private List<T> items;
    private Function<T, ItemStack> itemRenderer;
    private Set<T> selectedItems;
    private Set<Integer> selectedSlots;
    private boolean multiSelect;
    private BiConsumer<Set<T>, Player> onSelectionComplete;
    
    public SelectionMenu(Plugin plugin, Player player, String title, int size) {
        super(plugin, player, title, size);
        this.items = new ArrayList<>();
        this.selectedItems = new HashSet<>();
        this.selectedSlots = new HashSet<>();
        this.multiSelect = true;
    }
    
    public void setItems(List<T> items) {
        this.items = items;
    }
    
    public void setItemRenderer(Function<T, ItemStack> renderer) {
        this.itemRenderer = renderer;
    }
    
    public void setMultiSelect(boolean multiSelect) {
        this.multiSelect = multiSelect;
    }
    
    public void setOnSelectionComplete(BiConsumer<Set<T>, Player> callback) {
        this.onSelectionComplete = callback;
    }
    
    @Override
    protected void build() {
        inventory.clear();
        selectedItems.clear();
        selectedSlots.clear();
        
        for (int i = 0; i < items.size() && i < inventory.getSize(); i++) {
            T item = items.get(i);
            ItemStack rendered = renderItem(item, i, false);
            inventory.setItem(i, rendered);
        }
        
        addConfirmButton();
        fillEmptySlots();
    }
    
    private ItemStack renderItem(T item, int slot, boolean selected) {
        ItemStack base = itemRenderer != null ? itemRenderer.apply(item) : 
            new ItemBuilder(Material.PAPER).name(item.toString()).build();
        
        if (selected) {
            return new ItemBuilder(base)
                .glow()
                .lore("Selected")
                .build();
        }
        
        return base;
    }
    
    private void addConfirmButton() {
        ItemStack confirmButton = new ItemBuilder(Material.LIME_WOOL)
            .name("Confirm Selection")
            .lore("Click to confirm")
            .build();
        
        inventory.setItem(inventory.getSize() - 5, confirmButton);
    }
    
    @Override
    protected void onClick(InventoryClickEvent event) {
        int slot = event.getSlot();
        
        if (slot == inventory.getSize() - 5) {
            confirmSelection();
            return;
        }
        
        if (slot < 0 || slot >= items.size()) {
            return;
        }
        
        T item = items.get(slot);
        
        if (selectedSlots.contains(slot)) {
            selectedSlots.remove(slot);
            selectedItems.remove(item);
            
            ItemStack rendered = renderItem(item, slot, false);
            inventory.setItem(slot, rendered);
            
            player.playSound(player.getLocation(), UITheme.CLICK, 1.0f, 1.0f);
        } else {
            if (!multiSelect) {
                for (int s : new ArrayList<>(selectedSlots)) {
                    T prevItem = items.get(s);
                    selectedSlots.remove(s);
                    selectedItems.remove(prevItem);
                    inventory.setItem(s, renderItem(prevItem, s, false));
                }
            }
            
            selectedSlots.add(slot);
            selectedItems.add(item);
            
            ItemStack rendered = renderItem(item, slot, true);
            inventory.setItem(slot, rendered);
            
            player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
        }
    }
    
    private void confirmSelection() {
        if (onSelectionComplete != null) {
            onSelectionComplete.accept(selectedItems, player);
        }
        
        player.playSound(player.getLocation(), UITheme.SUCCESS, 1.0f, 1.0f);
        close();
    }
    
    public Set<T> getSelectedItems() {
        return new HashSet<>(selectedItems);
    }
    
    public void clearSelection() {
        selectedItems.clear();
        selectedSlots.clear();
        build();
    }
}
