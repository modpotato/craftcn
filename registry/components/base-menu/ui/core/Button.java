package com.craftcn.ui.core;

import com.craftcn.ui.UITheme;
import org.bukkit.Sound;
import org.bukkit.entity.Player;
import org.bukkit.event.inventory.ClickType;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.inventory.ItemStack;

import java.util.Objects;
import java.util.function.Consumer;

/**
 * A clickable menu slot: an icon and the action it performs.
 *
 * @param icon   the item shown in the slot
 * @param action runs when the player clicks the slot; never {@code null}
 * @param sound  played on click, or {@code null} for silence
 */
public record Button(ItemStack icon, Consumer<Click> action, Sound sound) {

    public Button {
        Objects.requireNonNull(icon, "icon");
        if (action == null) {
            action = click -> { };
        }
    }

    /** A button that plays the theme's click sound. */
    public static Button of(ItemStack icon, Consumer<Click> action) {
        return new Button(icon, action, UITheme.CLICK);
    }

    /** A slot that only displays an icon. */
    public static Button display(ItemStack icon) {
        return new Button(icon, null, null);
    }

    /** Returns a copy of this button that plays {@code sound} on click. */
    public Button withSound(Sound sound) {
        return new Button(icon, action, sound);
    }

    /**
     * Details of a click on a button.
     *
     * @param player the player who clicked
     * @param type   the click type, such as left, right or shift
     * @param event  the underlying event, already cancelled
     */
    public record Click(Player player, ClickType type, InventoryClickEvent event) {
    }
}
