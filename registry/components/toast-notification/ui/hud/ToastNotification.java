package com.craftcn.ui.hud;

import com.craftcn.ui.util.Tasks;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.serializer.gson.GsonComponentSerializer;
import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.NamespacedKey;
import org.bukkit.advancement.Advancement;
import org.bukkit.entity.Player;
import org.bukkit.plugin.Plugin;

import java.util.Locale;
import java.util.UUID;

/**
 * Achievement-style popups, the same kind the game shows for advancements. Each toast is a temporary advancement
 * that is awarded once and then removed, so it appears without a chat message.
 * <p>
 * Icons can be a vanilla {@link Material} or a resource pack item model (see the {@code texture-gui} component).
 * The advancement API is Paper's {@code UnsafeValues}, which is stable on Paper but not part of the public
 * Bukkit API.
 */
public final class ToastNotification {

    public enum Frame { TASK, GOAL, CHALLENGE }

    private static final String NAMESPACE = "craftcn";
    private static final String TRIGGER = "trigger";
    private static final long REMOVE_AFTER_TICKS = 60;

    private final Player player;
    private Component title = Component.empty();
    private Component description = Component.empty();
    private Material icon = Material.PAPER;
    private String iconModel;
    private Frame frame = Frame.TASK;

    private ToastNotification(Player player) {
        this.player = player;
    }

    public static ToastNotification to(Player player) {
        return new ToastNotification(player);
    }

    public ToastNotification title(String title) {
        return title(Component.text(title));
    }

    public ToastNotification title(Component title) {
        this.title = title;
        return this;
    }

    public ToastNotification description(String description) {
        return description(Component.text(description));
    }

    public ToastNotification description(Component description) {
        this.description = description;
        return this;
    }

    public ToastNotification icon(Material icon) {
        this.icon = icon;
        this.iconModel = null;
        return this;
    }

    /** Uses a resource pack item model for the icon, e.g. {@code craftcn:icon_info}. */
    public ToastNotification iconModel(NamespacedKey model) {
        this.iconModel = model.toString();
        return this;
    }

    public ToastNotification frame(Frame frame) {
        this.frame = frame;
        return this;
    }

    /** Shows the toast to the player. Safe to call from any thread. */
    public void send(Plugin plugin) {
        Tasks.run(plugin, player, () -> show(plugin));
    }

    private void show(Plugin plugin) {
        NamespacedKey key = new NamespacedKey(NAMESPACE, "toast/" + UUID.randomUUID());
        Advancement advancement = Bukkit.getUnsafe().loadAdvancement(key, toJson());

        player.getAdvancementProgress(advancement).awardCriteria(TRIGGER);

        Tasks.later(plugin, player, REMOVE_AFTER_TICKS, () -> Bukkit.getUnsafe().removeAdvancement(key));
    }

    private String toJson() {
        GsonComponentSerializer gson = GsonComponentSerializer.gson();

        String iconJson = "{\"id\":\"" + icon.getKey() + "\"" + modelComponent() + "}";
        return "{"
                + "\"criteria\":{\"" + TRIGGER + "\":{\"trigger\":\"minecraft:impossible\"}},"
                + "\"display\":{"
                + "\"icon\":" + iconJson + ","
                + "\"title\":" + gson.serialize(title) + ","
                + "\"description\":" + gson.serialize(description) + ","
                + "\"frame\":\"" + frame.name().toLowerCase(Locale.ROOT) + "\","
                + "\"announce_to_chat\":false,"
                + "\"show_toast\":true,"
                + "\"hidden\":true"
                + "}"
                + "}";
    }

    private String modelComponent() {
        if (iconModel == null) {
            return "";
        }
        return ",\"components\":{\"minecraft:item_model\":\"" + iconModel + "\"}";
    }
}
