package com.craftcn.ui.menus;

import com.craftcn.ui.UITheme;
import com.craftcn.ui.core.BaseMenu;
import com.craftcn.ui.core.Button;
import com.craftcn.ui.util.ItemBuilder;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.Plugin;

import java.util.ArrayList;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Set;
import java.util.function.BiConsumer;
import java.util.function.Function;

/**
 * Lets the player pick one or several items from a list and confirm the choice.
 * <p>
 * Items fill the menu from the top left. The bottom row is reserved for the confirm button, so a menu of
 * {@code size} slots holds {@code size - 9} choices.
 *
 * @param <T> the type of item to choose from
 */
public class SelectionMenu<T> extends BaseMenu {

    public static final String CONFIRM_LABEL = "Confirm selection";
    public static final String SELECTED_LABEL = "Selected";

    private final List<T> items = new ArrayList<>();
    private final Set<T> selected = new LinkedHashSet<>();
    private final int confirmSlot;
    private Function<T, ItemStack> itemRenderer = item -> ItemBuilder.from(Material.PAPER).name(String.valueOf(item)).build();
    private boolean multiSelect = true;
    private BiConsumer<Set<T>, Player> onSelectionComplete = (chosen, player) -> { };

    public SelectionMenu(Plugin plugin, Player player, String title, int size) {
        super(plugin, player, title, size);
        this.confirmSlot = size - 5;
    }

    public void setItems(List<T> source) {
        items.clear();
        items.addAll(source);
        refresh();
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

    /** A copy of the current selection, in the order it was made. */
    public Set<T> getSelectedItems() {
        return new LinkedHashSet<>(selected);
    }

    public void clearSelection() {
        selected.clear();
        refresh();
    }

    @Override
    protected void build() {
        fillRange(0, size, filler());

        int capacity = size - 9;
        for (int index = 0; index < Math.min(items.size(), capacity); index++) {
            T item = items.get(index);
            setButton(index, Button.of(render(item, selected.contains(item)), click -> toggle(item)));
        }

        ItemStack confirm = ItemBuilder.from(Material.LIME_WOOL)
                .name(CONFIRM_LABEL)
                .lore(selected.size() + " selected")
                .build();
        setButton(confirmSlot, Button.of(confirm, click -> confirm()).withSound(UITheme.SUCCESS));
    }

    private ItemStack render(T item, boolean isSelected) {
        ItemBuilder builder = ItemBuilder.from(itemRenderer.apply(item));
        if (isSelected) {
            builder.glow().addLore(Component.text(SELECTED_LABEL, NamedTextColor.GREEN));
        }
        return builder.build();
    }

    private void toggle(T item) {
        if (selected.contains(item)) {
            selected.remove(item);
        } else {
            if (!multiSelect) {
                selected.clear();
            }
            selected.add(item);
        }
        refresh();
    }

    private void confirm() {
        onSelectionComplete.accept(getSelectedItems(), player);
        close();
    }
}
