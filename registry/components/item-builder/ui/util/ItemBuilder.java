package com.craftcn.ui.util;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.Material;
import org.bukkit.enchantments.Enchantment;
import org.bukkit.inventory.ItemFlag;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

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
        meta.displayName(name);
        return this;
    }
    
    public ItemBuilder lore(String... lore) {
        List<Component> loreComponents = new ArrayList<>();
        for (String line : lore) {
            loreComponents.add(Component.text(line));
        }
        return lore(loreComponents);
    }
    
    public ItemBuilder lore(List<String> lore) {
        List<Component> loreComponents = new ArrayList<>();
        for (String line : lore) {
            loreComponents.add(Component.text(line));
        }
        return lore(loreComponents);
    }
    
    public ItemBuilder lore(Component... lore) {
        return lore(java.util.Arrays.asList(lore));
    }
    
    public ItemBuilder lore(List<Component> lore) {
        meta.lore(lore);
        return this;
    }
    
    public ItemBuilder addLore(String line) {
        return addLore(Component.text(line));
    }
    
    public ItemBuilder addLore(Component line) {
        List<Component> lore = meta.lore();
        if (lore == null) {
            lore = new ArrayList<>();
        }
        lore.add(line);
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
    
    public ItemBuilder glow() {
        meta.addItemFlags(ItemFlag.HIDE_ENCHANTS);
        meta.addEnchant(Enchantment.UNBREAKING, 1, true);
        return this;
    }
    
    public ItemBuilder unbreakable() {
        meta.setUnbreakable(true);
        meta.addItemFlags(ItemFlag.HIDE_UNBREAKABLE);
        return this;
    }
    
    public ItemBuilder amount(int amount) {
        item.setAmount(amount);
        return this;
    }
    
    public ItemBuilder durability(int durability) {
        if (meta instanceof org.bukkit.inventory.meta.Damageable) {
            ((org.bukkit.inventory.meta.Damageable) meta).setDamage(durability);
        }
        return this;
    }
    
    public ItemBuilder customModelData(int data) {
        meta.setCustomModelData(data);
        return this;
    }
    
    public ItemBuilder flag(ItemFlag... flags) {
        meta.addItemFlags(flags);
        return this;
    }
    
    public ItemBuilder removeFlag(ItemFlag... flags) {
        meta.removeItemFlags(flags);
        return this;
    }
    
    public ItemBuilder hideAttributes() {
        return flag(
            ItemFlag.HIDE_ATTRIBUTES,
            ItemFlag.HIDE_DESTROYS,
            ItemFlag.HIDE_PLACED_ON,
            ItemFlag.HIDE_ADDITIONAL_TOOLTIP
        );
    }
    
    public ItemBuilder hideAll() {
        return flag(ItemFlag.values());
    }
    
    public ItemStack build() {
        item.setItemMeta(meta);
        return item;
    }
    
    public ItemStack get() {
        return build();
    }
    
    public Material getMaterial() {
        return item.getType();
    }
    
    public int getAmount() {
        return item.getAmount();
    }
    
    public Component getName() {
        return meta.displayName();
    }
    
    public List<Component> getLore() {
        return meta.lore();
    }
}
