package com.craftcn.ui.util;

import com.destroystokyo.paper.profile.PlayerProfile;
import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.OfflinePlayer;
import org.bukkit.entity.Player;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.SkullMeta;

import java.net.MalformedURLException;
import java.net.URI;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.util.UUID;

/**
 * Builds player head items from player names or skin URLs using Paper's {@link PlayerProfile}.
 * <p>
 * Skins are referenced by URL, for example {@code https://textures.minecraft.net/texture/<hash>}, and the
 * client loads them directly, so no Mojang API or Base64 decoding is needed.
 */
public final class HeadUtil {

    private HeadUtil() {
    }

    /** A head showing the skin of {@code player}. */
    public static ItemStack of(Player player) {
        return of(player.getName());
    }

    /** A head showing the skin of the player called {@code name}. */
    public static ItemStack of(String name) {
        OfflinePlayer owner = Bukkit.getOfflinePlayer(name);
        ItemStack head = new ItemStack(Material.PLAYER_HEAD);
        SkullMeta meta = (SkullMeta) head.getItemMeta();
        meta.setOwningPlayer(owner);
        head.setItemMeta(meta);
        return head;
    }

    /**
     * A head that shows the skin at {@code skinUrl}. The profile id is derived from the URL, so the same
     * skin always produces the same profile.
     *
     * @throws IllegalArgumentException if the URL is malformed
     */
    public static ItemStack skin(String skinUrl) {
        URL url = parse(skinUrl);
        UUID id = UUID.nameUUIDFromBytes(skinUrl.getBytes(StandardCharsets.UTF_8));

        PlayerProfile profile = Bukkit.createProfile(id, null);
        profile.getTextures().setSkin(url);

        ItemStack head = new ItemStack(Material.PLAYER_HEAD);
        SkullMeta meta = (SkullMeta) head.getItemMeta();
        meta.setPlayerProfile(profile);
        head.setItemMeta(meta);
        return head;
    }

    private static URL parse(String skinUrl) {
        try {
            return URI.create(skinUrl).toURL();
        } catch (MalformedURLException | IllegalArgumentException e) {
            throw new IllegalArgumentException("Invalid skin URL: " + skinUrl, e);
        }
    }
}
