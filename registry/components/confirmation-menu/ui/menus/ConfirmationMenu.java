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

import java.util.List;

/**
 * A three-row confirm and cancel dialog for destructive actions. The message sits in the centre, confirm
 * is on the left and cancel is on the right.
 */
public class ConfirmationMenu extends BaseMenu {

    public static final String CONFIRM_LABEL = "Confirm";
    public static final String CANCEL_LABEL = "Cancel";

    private static final int CONFIRM_SLOT = 11;
    private static final int MESSAGE_SLOT = 13;
    private static final int CANCEL_SLOT = 15;

    private final Component message;
    private Runnable onConfirm = () -> { };
    private Runnable onDeny = () -> { };

    public ConfirmationMenu(Plugin plugin, Player player, String title, String message) {
        this(plugin, player, Component.text(title), Component.text(message));
    }

    public ConfirmationMenu(Plugin plugin, Player player, Component title, Component message) {
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
        fillRange(0, size, filler());

        setItem(MESSAGE_SLOT, ItemBuilder.from(Material.PAPER)
                .name(Component.text("Are you sure?", NamedTextColor.GOLD))
                .lore(List.of(message))
                .build());

        ItemStack confirm = ItemBuilder.from(Material.LIME_WOOL)
                .name(CONFIRM_LABEL)
                .lore("Click to confirm")
                .glow()
                .build();
        setButton(CONFIRM_SLOT, Button.of(confirm, click -> {
            onConfirm.run();
            close();
        }).withSound(UITheme.SUCCESS));

        ItemStack cancel = ItemBuilder.from(Material.RED_WOOL)
                .name(CANCEL_LABEL)
                .lore("Click to cancel")
                .build();
        setButton(CANCEL_SLOT, Button.of(cancel, click -> {
            onDeny.run();
            close();
        }).withSound(UITheme.CLOSE));
    }
}
