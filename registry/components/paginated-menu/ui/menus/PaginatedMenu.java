package com.craftcn.ui.menus;

import com.craftcn.ui.core.BaseMenu;
import com.craftcn.ui.core.UITheme;
import com.craftcn.ui.util.ItemBuilder;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.Plugin;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Function;

public class PaginatedMenu<T> extends BaseMenu {
    
    private List<T> items;
    private Function<T, ItemStack> renderer;
    private int currentPage = 0;
    private int itemsPerPage = 28;
    private int startIndex = 9;
    private int endIndex = 36;
    
    public PaginatedMenu(Plugin plugin, Player player, String title) {
        super(plugin, player, title, 54);
        this.items = new ArrayList<>();
    }
    
    public void setSource(List<T> items) {
        this.items = items;
        this.currentPage = 0;
    }
    
    public void setRenderer(Function<T, ItemStack> renderer) {
        this.renderer = renderer;
    }
    
    public void setItemsPerPage(int count) {
        this.itemsPerPage = count;
    }
    
    public void setSlotRange(int start, int end) {
        this.startIndex = start;
        this.endIndex = end;
    }
    
    @Override
    protected void build() {
        inventory.clear();
        
        renderPagination();
        renderItems();
        fillEmptySlots();
    }
    
    private void renderItems() {
        if (renderer == null || items.isEmpty()) {
            return;
        }
        
        int start = currentPage * itemsPerPage;
        int end = Math.min(start + itemsPerPage, items.size());
        
        for (int i = start; i < end; i++) {
            int slot = startIndex + (i - start);
            if (slot < endIndex) {
                T item = items.get(i);
                ItemStack rendered = render(item, slot);
                inventory.setItem(slot, rendered);
            }
        }
    }
    
    protected ItemStack render(T item, int slot) {
        if (renderer != null) {
            return renderer.apply(item);
        }
        // Default rendering if no custom renderer
        return ItemBuilder.from(Material.PAPER)
            .name(item.toString())
            .build();
    }
    
    private void renderPagination() {
        inventory.setItem(0, createFiller());
        inventory.setItem(1, createFiller());
        inventory.setItem(2, createFiller());
        inventory.setItem(3, createFiller());
        inventory.setItem(5, createFiller());
        inventory.setItem(6, createFiller());
        inventory.setItem(7, createFiller());
        inventory.setItem(8, createFiller());
        
        ItemStack prevButton = createNavigationButton(false);
        ItemStack nextButton = createNavigationButton(true);
        
        inventory.setItem(45, prevButton);
        inventory.setItem(53, nextButton);
        
        fillRange(36, 45);
        
        ItemStack pageInfo = createPageInfo();
        inventory.setItem(49, pageInfo);
    }
    
    private ItemStack createNavigationButton(boolean isNext) {
        String displayName = isNext ? "Next Page" : "Previous Page";
        Material material = isNext ? UITheme.NEXT_BUTTON : UITheme.BACK_BUTTON;
        
        return ItemBuilder.from(material)
            .name(displayName)
            .lore("Page " + (currentPage + (isNext ? 2 : 1)))
            .build();
    }
    
    private ItemStack createPageInfo() {
        int totalPages = (int) Math.ceil((double) items.size() / itemsPerPage);
        return ItemBuilder.from(Material.BOOK)
            .name("Page Info")
            .lore("Page: " + (currentPage + 1) + " / " + Math.max(1, totalPages))
            .lore("Items: " + items.size())
            .build();
    }
    
    @Override
    protected void onClick(InventoryClickEvent event) {
        int slot = event.getSlot();
        
        if (slot == 45 && currentPage > 0) {
            currentPage--;
            build();
            player.playSound(player.getLocation(), UITheme.CLICK, 1.0f, 1.0f);
        } else if (slot == 53 && (currentPage + 1) * itemsPerPage < items.size()) {
            currentPage++;
            build();
            player.playSound(player.getLocation(), UITheme.CLICK, 1.0f, 1.0f);
        } else if (slot >= startIndex && slot < endIndex) {
            int itemIndex = currentPage * itemsPerPage + (slot - startIndex);
            if (itemIndex >= 0 && itemIndex < items.size()) {
                T item = items.get(itemIndex);
                onItemClick(item, slot);
            }
        }
    }
    
    protected void onItemClick(T item, int slot) {
    }
    
    public void nextPage() {
        int totalPages = (int) Math.ceil((double) items.size() / itemsPerPage);
        if (currentPage < totalPages - 1) {
            currentPage++;
            build();
        }
    }
    
    public void previousPage() {
        if (currentPage > 0) {
            currentPage--;
            build();
        }
    }
    
    public int getCurrentPage() {
        return currentPage;
    }
    
    public int getTotalPages() {
        return (int) Math.ceil((double) items.size() / itemsPerPage);
    }
    
    public List<T> getItems() {
        return items;
    }
}
