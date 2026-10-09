package com.craftcn.ui.util;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.TextDecoration;
import org.bukkit.Material;
import org.bukkit.NamespacedKey;
import org.bukkit.enchantments.Enchantment;
import org.bukkit.inventory.ItemFlag;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.Damageable;
import org.bukkit.inventory.meta.ItemMeta;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * Fluent builder for {@link ItemStack}s using modern item data components.
 * <p>
 * Names and lore are non-italic by default, which is what most custom items want. Use
 * {@link #model(NamespacedKey)} to give an item a resource pack model (the {@code texture-gui}
 * component ships several), and {@link #tooltipStyle(NamespacedKey)} for a custom tooltip frame.
 */
public class ItemBuilder {

    private final ItemStack item;
    private final ItemMeta meta;

    private ItemBuilder(ItemStack item) {
        this.item = item;
        this.meta = item.getItemMeta();
    }

    public static ItemBuilder from(Material material) {
        return new ItemBuilder(new ItemStack(material));
    }

    public static ItemBuilder from(ItemStack item) {
        return new ItemBuilder(item.clone());
    }

    public ItemBuilder name(String name) {
        return name(Component.text(name));
    }

    public ItemBuilder name(Component name) {
        meta.displayName(nonItalic(name));
        return this;
    }

    public ItemBuilder lore(String... lines) {
        List<Component> components = new ArrayList<>();
        for (String line : lines) {
            components.add(nonItalic(Component.text(line)));
        }
        meta.lore(components);
        return this;
    }

    public ItemBuilder lore(Component... lines) {
        return lore(Arrays.asList(lines));
    }

    public ItemBuilder lore(List<? extends Component> lines) {
        List<Component> components = new ArrayList<>();
        for (Component line : lines) {
            components.add(nonItalic(line));
        }
        meta.lore(components);
        return this;
    }

    public ItemBuilder addLore(String line) {
        return addLore(Component.text(line));
    }

    public ItemBuilder addLore(Component line) {
        List<Component> lore = meta.lore() == null ? new ArrayList<>() : new ArrayList<>(meta.lore());
        lore.add(nonItalic(line));
        meta.lore(lore);
        return this;
    }

    public ItemBuilder enchant(Enchantment enchantment, int level) {
        meta.addEnchant(enchantment, level, true);
        return this;
    }

    public ItemBuilder enchant(Enchantment enchantment) {
        return enchant(enchantment, 1);
    }

    public ItemBuilder removeEnchantment(Enchantment enchantment) {
        meta.removeEnchant(enchantment);
        return this;
    }

    /** Shows the enchantment glint without an enchantment, or hides it on an enchanted item. */
    public ItemBuilder glint(boolean glint) {
        meta.setEnchantmentGlintOverride(glint);
        return this;
    }

    /** Shorthand for {@code glint(true)}. */
    public ItemBuilder glow() {
        return glint(true);
    }

    public ItemBuilder unbreakable() {
        meta.setUnbreakable(true);
        meta.addItemFlags(ItemFlag.HIDE_UNBREAKABLE);
        return this;
    }

    public ItemBuilder flags(ItemFlag... flags) {
        meta.addItemFlags(flags);
        return this;
    }

    public ItemBuilder amount(int amount) {
        item.setAmount(Math.max(1, Math.min(amount, item.getMaxStackSize())));
        return this;
    }

    public ItemBuilder durability(int damage) {
        if (meta instanceof Damageable damageable) {
            damageable.setDamage(Math.max(0, damage));
        }
        return this;
    }

    /** Sets the item model used by the resource pack, e.g. {@code craftcn:icon_next}. */
    public ItemBuilder model(NamespacedKey model) {
        meta.setItemModel(model);
        return this;
    }

    /** Sets the item model from a string key such as {@code craftcn:icon_next}. Invalid keys are ignored. */
    public ItemBuilder model(String key) {
        NamespacedKey parsed = NamespacedKey.fromString(key);
        if (parsed != null) {
            meta.setItemModel(parsed);
        }
        return this;
    }

    /** Sets a tooltip frame from the resource pack, e.g. {@code craftcn:panel}. */
    public ItemBuilder tooltipStyle(NamespacedKey style) {
        meta.setTooltipStyle(style);
        return this;
    }

    /** Legacy numeric custom model data. Prefer {@link #model(NamespacedKey)} on Paper 1.21.2+. */
    public ItemBuilder customModelData(int data) {
        meta.setCustomModelData(data);
        return this;
    }

    public ItemStack build() {
        item.setItemMeta(meta);
        return item;
    }

    private static Component nonItalic(Component component) {
        return component.decorationIfAbsent(TextDecoration.ITALIC, TextDecoration.State.FALSE);
    }
}
