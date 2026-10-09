package com.craftcn.ui.core;

import com.craftcn.ui.UITheme;
import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryDragEvent;
import org.bukkit.event.inventory.PrepareAnvilEvent;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.InventoryHolder;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;
import org.bukkit.plugin.Plugin;

import java.util.HashMap;
import java.util.Map;

/**
 * Base class for chest-style inventory menus.
 * <p>
 * Each open menu is its own {@link InventoryHolder}, and a single {@link MenuListener} per plugin routes
 * events to it. Override {@link #build()} to lay out the menu, place clickable {@link Button}s with
 * {@link #setButton(int, Button)}, and call {@link #refresh()} when state changes. Every click inside the
 * menu is cancelled, so items can never leave it.
 * <p>
 * Call {@link #open()} from the viewer's thread (see the {@code scheduler} component on Folia).
 */
public abstract class BaseMenu implements InventoryHolder {

    protected final Plugin plugin;
    protected final Player player;
    protected final int size;
    protected Component title;

    private final Map<Integer, Button> buttons = new HashMap<>();
    private Inventory inventory;
    private boolean open;

    /**
     * @param plugin the owning plugin
     * @param player the viewer
     * @param title  the title shown at the top of the inventory
     * @param size   the number of slots, a multiple of 9 between 9 and 54
     */
    public BaseMenu(Plugin plugin, Player player, Component title, int size) {
        this.plugin = plugin;
        this.player = player;
        this.title = title;
        this.size = size;
    }

    /** Convenience constructor for plain-text titles. */
    public BaseMenu(Plugin plugin, Player player, String title, int size) {
        this(plugin, player, Component.text(title), size);
    }

    /** Creates the menu's inventory and shows it to the viewer. */
    public final void open() {
        MenuListener.register(plugin);

        inventory = createInventory();
        buttons.clear();
        open = true;

        build();
        player.openInventory(inventory);
        onOpen();
    }

    /** Rebuilds the contents of an open menu, typically after its state changed. */
    public final void refresh() {
        if (!open) {
            return;
        }
        inventory.clear();
        buttons.clear();
        build();
    }

    /** Closes the menu for the viewer. {@link #onClose()} runs once the menu is closed. */
    public final void close() {
        if (!open) {
            return;
        }
        if (player.getOpenInventory().getTopInventory().getHolder() == this) {
            // Fires InventoryCloseEvent, which ends up in handleClose().
            player.closeInventory();
        } else {
            handleClose();
        }
    }

    /** True while the menu is shown to the viewer. */
    public final boolean isOpen() {
        return open;
    }

    public final Player getPlayer() {
        return player;
    }

    @Override
    public final Inventory getInventory() {
        return inventory;
    }

    /**
     * Creates the backing inventory. Override to use another inventory type, such as an anvil.
     *
     * @return a new inventory whose holder is this menu
     */
    protected Inventory createInventory() {
        return Bukkit.createInventory(this, size, title);
    }

    /** Populates the inventory. Called on {@link #open()} and {@link #refresh()}. */
    protected abstract void build();

    /** Called after the menu is shown to the viewer. */
    protected void onOpen() {
    }

    /** Called once the menu is closed, by the viewer or by {@link #close()}. */
    protected void onClose() {
    }

    /** Called for every click in the menu, after any {@link Button} at that slot has run. */
    protected void onClick(InventoryClickEvent event) {
    }

    /** Called when an anvil-backed menu recalculates its result. */
    protected void onPrepareAnvil(PrepareAnvilEvent event) {
    }

    /** Places a clickable button in a slot. */
    protected final void setButton(int slot, Button button) {
        buttons.put(slot, button);
        inventory.setItem(slot, button.icon());
    }

    /** Places a display-only item in a slot. */
    protected final void setItem(int slot, ItemStack item) {
        buttons.remove(slot);
        inventory.setItem(slot, item);
    }

    /** The button at a slot, or {@code null} when the slot is not interactive. */
    protected final Button buttonAt(int slot) {
        return buttons.get(slot);
    }

    /** Fills every empty slot with {@code filler}. */
    protected final void fillEmptySlots(ItemStack filler) {
        for (int slot = 0; slot < inventory.getSize(); slot++) {
            ItemStack current = inventory.getItem(slot);
            if (current == null || current.getType().isAir()) {
                inventory.setItem(slot, filler);
            }
        }
    }

    /** Fills the slots from {@code from} (inclusive) to {@code to} (exclusive) with {@code filler}. */
    protected final void fillRange(int from, int to, ItemStack filler) {
        for (int slot = Math.max(0, from); slot < Math.min(to, inventory.getSize()); slot++) {
            inventory.setItem(slot, filler);
        }
    }

    /** A blank, unnamed glass pane from the theme, used to fill empty space. */
    protected final ItemStack filler() {
        ItemStack filler = new ItemStack(UITheme.FILLER_GLASS);
        ItemMeta meta = filler.getItemMeta();
        meta.itemName(Component.empty());
        filler.setItemMeta(meta);
        return filler;
    }

    void handleClick(InventoryClickEvent event) {
        // Cancel first: nothing may leave or enter the menu, even if a button throws.
        event.setCancelled(true);

        if (event.getClickedInventory() == inventory) {
            Button button = buttons.get(event.getSlot());
            if (button != null) {
                if (button.sound() != null) {
                    player.playSound(player.getLocation(), button.sound(), 1.0f, 1.0f);
                }
                button.action().accept(new Button.Click(player, event.getClick(), event));
            }
        }

        onClick(event);
    }

    void handleDrag(InventoryDragEvent event) {
        event.setCancelled(true);
    }

    void handlePrepareAnvil(PrepareAnvilEvent event) {
        onPrepareAnvil(event);
    }

    void handleClose() {
        if (!open) {
            return;
        }
        open = false;
        buttons.clear();
        onClose();
    }
}
