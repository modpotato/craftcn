package com.craftcn.ui.menus;

import com.craftcn.ui.UITheme;
import com.craftcn.ui.core.BaseMenu;
import com.craftcn.ui.core.Button;
import com.craftcn.ui.util.ItemBuilder;
import net.kyori.adventure.text.Component;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.Plugin;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.function.BiConsumer;
import java.util.function.Function;

/**
 * A framed, paginated menu. Items fill the inner grid (seven columns by the rows between the top and bottom
 * rows), and the bottom row holds previous, page info and next.
 * <p>
 * Call {@link #setSource(List)} and {@link #setRenderer(Function)}, then {@link #open()}.
 *
 * @param <T> the type of item being paged
 */
public class PaginatedMenu<T> extends BaseMenu {

    public static final String PREVIOUS_LABEL = "Previous page";
    public static final String NEXT_LABEL = "Next page";
    public static final String EMPTY_LABEL = "Nothing here yet";

    private static final int COLUMNS = 7;

    private final List<T> items = new ArrayList<>();
    private final List<Integer> contentSlots = new ArrayList<>();
    private final int lastRow;
    private Function<T, ItemStack> renderer = item -> ItemBuilder.from(Material.PAPER).name(String.valueOf(item)).build();
    private BiConsumer<T, Button.Click> onItemClick = (item, click) -> { };
    private int page;

    /**
     * @param rows the menu height, from 3 to 6
     */
    public PaginatedMenu(Plugin plugin, Player player, String title, int rows) {
        this(plugin, player, Component.text(title), rows);
    }

    public PaginatedMenu(Plugin plugin, Player player, Component title, int rows) {
        super(plugin, player, title, rows * 9);
        if (rows < 3 || rows > 6) {
            throw new IllegalArgumentException("rows must be between 3 and 6: " + rows);
        }

        this.lastRow = rows - 1;
        for (int row = 1; row < lastRow; row++) {
            for (int column = 1; column <= COLUMNS; column++) {
                contentSlots.add(row * 9 + column);
            }
        }
    }

    /** Replaces the items to page through and returns to the first page. */
    public void setSource(List<T> source) {
        items.clear();
        items.addAll(source);
        page = 0;
        refresh();
    }

    /** Maps each item to the icon shown for it. */
    public void setRenderer(Function<T, ItemStack> renderer) {
        this.renderer = renderer;
    }

    /** Runs when the player clicks an item. */
    public void onItemClick(BiConsumer<T, Button.Click> handler) {
        this.onItemClick = handler;
    }

    public void nextPage() {
        if (page < pageCount() - 1) {
            page++;
            refresh();
        }
    }

    public void previousPage() {
        if (page > 0) {
            page--;
            refresh();
        }
    }

    /** Zero-based index of the page being shown. */
    public int page() {
        return page;
    }

    public int pageCount() {
        return Math.max(1, (int) Math.ceil((double) items.size() / contentSlots.size()));
    }

    public List<T> items() {
        return Collections.unmodifiableList(items);
    }

    @Override
    protected void build() {
        fillRange(0, size, filler());

        int perPage = contentSlots.size();
        int from = page * perPage;
        int to = Math.min(from + perPage, items.size());

        if (items.isEmpty()) {
            ItemStack empty = ItemBuilder.from(Material.BARRIER).name(EMPTY_LABEL).build();
            setItem(contentSlots.get(contentSlots.size() / 2), empty);
        }

        for (int index = from; index < to; index++) {
            T item = items.get(index);
            setButton(contentSlots.get(index - from), Button.of(renderer.apply(item), click -> onItemClick.accept(item, click)));
        }

        int previousSlot = lastRow * 9;
        int infoSlot = lastRow * 9 + 4;
        int nextSlot = lastRow * 9 + 8;

        if (page > 0) {
            setButton(previousSlot, Button.of(navigation(false), click -> previousPage()));
        } else {
            setButton(previousSlot, Button.display(navigation(false)));
        }

        setItem(infoSlot, pageInfo());

        if (page < pageCount() - 1) {
            setButton(nextSlot, Button.of(navigation(true), click -> nextPage()));
        } else {
            setButton(nextSlot, Button.display(navigation(true)));
        }
    }

    private ItemStack navigation(boolean next) {
        Material material = next ? UITheme.NEXT_BUTTON : UITheme.BACK_BUTTON;
        return ItemBuilder.from(material)
                .name(next ? NEXT_LABEL : PREVIOUS_LABEL)
                .lore("Page " + (page + (next ? 2 : 0)) + " of " + pageCount())
                .build();
    }

    private ItemStack pageInfo() {
        return ItemBuilder.from(Material.BOOK)
                .name("Page " + (page + 1) + " of " + pageCount())
                .lore(items.size() + " items")
                .build();
    }
}
