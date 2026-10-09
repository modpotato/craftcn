package com.craftcn.ui.textures;

import com.craftcn.ui.core.BaseMenu;
import net.kyori.adventure.text.Component;
import org.bukkit.entity.Player;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.Plugin;

/**
 * A {@link BaseMenu} with a title plate and textured filler from the CraftCN resource pack.
 * <p>
 * Use it like any menu: override {@link #build()}, place buttons with {@code setButton}, and call
 * {@link #fillFrame()} for the empty space. Players without the pack get the plain title and vanilla fillers.
 */
public abstract class TexturedMenu extends BaseMenu {

    private final GuiFont.Plate plate;

    /**
     * @param rows  the menu height, from 1 to 6
     * @param plate the title plate to draw behind the title
     */
    protected TexturedMenu(Plugin plugin, Player player, Component title, int rows, GuiFont.Plate plate) {
        super(plugin, player, titleFor(player, title, plate), rows * 9);
        this.plate = plate;
    }

    /** The plate this menu draws. */
    protected final GuiFont.Plate plate() {
        return plate;
    }

    /** Fills every empty slot with the textured filler (or the vanilla one for players without the pack). */
    protected final void fillFrame() {
        fillEmptySlots(textureFiller());
    }

    /** The textured filler item for this viewer. */
    protected final ItemStack textureFiller() {
        return GuiIcons.filler(player).name(Component.empty()).build();
    }

    private static Component titleFor(Player player, Component title, GuiFont.Plate plate) {
        return GuiIcons.hasPack(player) ? GuiFont.title(plate, title) : GuiFont.plainTitle(title);
    }
}
