package com.craftcn.ui.textures;

import com.craftcn.ui.UITheme;
import com.craftcn.ui.util.ItemBuilder;
import org.bukkit.Material;
import org.bukkit.NamespacedKey;
import org.bukkit.entity.Player;

import java.util.function.Predicate;

/**
 * Icons from the CraftCN resource pack. Each icon is an item model, so it only looks custom for players who have
 * the pack. Everyone else sees the fallback material.
 * <p>
 * Connect this to the {@code resource-pack} component once, at startup:
 * {@code GuiIcons.setPackCheck(resourcePack::hasPack)}. Until then, every player gets the fallbacks.
 */
public final class GuiIcons {

    public static final NamespacedKey NEXT = new NamespacedKey("craftcn", "icon_next");
    public static final NamespacedKey BACK = new NamespacedKey("craftcn", "icon_back");
    public static final NamespacedKey CONFIRM = new NamespacedKey("craftcn", "icon_confirm");
    public static final NamespacedKey CANCEL = new NamespacedKey("craftcn", "icon_cancel");
    public static final NamespacedKey INFO = new NamespacedKey("craftcn", "icon_info");
    public static final NamespacedKey CLOSE = new NamespacedKey("craftcn", "icon_close");
    public static final NamespacedKey FILLER = new NamespacedKey("craftcn", "icon_filler");

    private static volatile Predicate<Player> packCheck = player -> false;

    private GuiIcons() {
    }

    /** Decides per player whether the resource pack is loaded. */
    public static void setPackCheck(Predicate<Player> check) {
        packCheck = check;
    }

    /** True when {@code player} has the resource pack, according to the check set with {@link #setPackCheck}. */
    public static boolean hasPack(Player player) {
        return packCheck.test(player);
    }

    /** An icon for {@code player}: the pack model if they have the pack, otherwise {@code fallback}. */
    public static ItemBuilder icon(Player player, NamespacedKey model, Material fallback) {
        ItemBuilder builder = ItemBuilder.from(fallback);
        if (hasPack(player)) {
            builder.model(model);
        }
        return builder;
    }

    public static ItemBuilder next(Player player) {
        return icon(player, NEXT, UITheme.NEXT_BUTTON);
    }

    public static ItemBuilder back(Player player) {
        return icon(player, BACK, UITheme.BACK_BUTTON);
    }

    public static ItemBuilder confirm(Player player) {
        return icon(player, CONFIRM, Material.LIME_WOOL);
    }

    public static ItemBuilder cancel(Player player) {
        return icon(player, CANCEL, Material.RED_WOOL);
    }

    public static ItemBuilder info(Player player) {
        return icon(player, INFO, Material.BOOK);
    }

    public static ItemBuilder close(Player player) {
        return icon(player, CLOSE, Material.BARRIER);
    }

    /** The textured filler for empty slots. */
    public static ItemBuilder filler(Player player) {
        return icon(player, FILLER, UITheme.FILLER_GLASS);
    }
}
