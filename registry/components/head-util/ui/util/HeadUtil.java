package com.craftcn.ui.util;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.SkullMeta;
import org.bukkit.profile.PlayerProfile;
import org.bukkit.profile.PlayerTextures;

import java.net.URI;
import java.util.Base64;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.CompletableFuture;

public class HeadUtil {
    
    private static final Map<String, ItemStack> textureCache = new HashMap<>();
    private static final Map<String, PlayerProfile> profileCache = new HashMap<>();
    private static final Gson gson = new Gson();
    
    private HeadUtil() {
    }
    
    public static ItemStack createPlayerHead(Player player) {
        return createPlayerHead(player.getName());
    }
    
    public static ItemStack createPlayerHead(String playerName) {
        ItemStack head = new ItemStack(Material.PLAYER_HEAD);
        SkullMeta meta = (SkullMeta) head.getItemMeta();
        meta.setOwningPlayer(Bukkit.getOfflinePlayer(playerName));
        head.setItemMeta(meta);
        return head;
    }
    
    public static CompletableFuture<ItemStack> createTextureHead(String textureUrl) {
        String cacheKey = "texture:" + textureUrl;
        
        if (textureCache.containsKey(cacheKey)) {
            return CompletableFuture.completedFuture(textureCache.get(cacheKey).clone());
        }
        
        return CompletableFuture.supplyAsync(() -> {
            try {
                String base64Texture = encodeTexture(textureUrl);
                return createHeadFromBase64(base64Texture, cacheKey);
            } catch (Exception e) {
                e.printStackTrace();
                return createPlayerHead("MHF_Question");
            }
        });
    }
    
    public static ItemStack createHeadFromBase64(String base64Texture) {
        return createHeadFromBase64(base64Texture, null);
    }
    
    private static ItemStack createHeadFromBase64(String base64Texture, String cacheKey) {
        ItemStack head = new ItemStack(Material.PLAYER_HEAD);
        SkullMeta meta = (SkullMeta) head.getItemMeta();
        
        if (meta != null) {
            try {
                String decoded = new String(Base64.getDecoder().decode(base64Texture));
                JsonObject json = gson.fromJson(decoded, JsonObject.class);
                String texture = json.getAsJsonObject("textures")
                    .getAsJsonObject("SKIN")
                    .get("url")
                    .getAsString();
                
                UUID uuid = UUID.nameUUIDFromBytes(texture.getBytes());
                PlayerProfile profile = Bukkit.createPlayerProfile(uuid, "CraftCN_Head");
                
                PlayerTextures textures = profile.getTextures();
                textures.setSkin(URI.create(texture));
                
                profile.setTextures(textures);
                meta.setOwnerProfile(profile);
                
                if (cacheKey != null) {
                    textureCache.put(cacheKey, head.clone());
                }
            } catch (Exception e) {
                e.printStackTrace();
                meta.setOwningPlayer(Bukkit.getOfflinePlayer("MHF_Question"));
            }
        }
        
        head.setItemMeta(meta);
        return head;
    }
    
    public static CompletableFuture<ItemStack> createMojangHead(String playerName) {
        String cacheKey = "mojang:" + playerName;
        
        if (textureCache.containsKey(cacheKey)) {
            return CompletableFuture.completedFuture(textureCache.get(cacheKey).clone());
        }
        
        return CompletableFuture.supplyAsync(() -> {
            try {
                UUID uuid = fetchMojangUUID(playerName);
                if (uuid == null) {
                    return createPlayerHead("MHF_Question");
                }
                
                ItemStack head = new ItemStack(Material.PLAYER_HEAD);
                SkullMeta meta = (SkullMeta) head.getItemMeta();
                
                if (meta != null) {
                    PlayerProfile profile = Bukkit.createPlayerProfile(uuid, playerName);
                    PlayerTextures textures = profile.getTextures();
                    textures.updateSkin();
                    profile.setTextures(textures);
                    meta.setOwnerProfile(profile);
                }
                
                head.setItemMeta(meta);
                textureCache.put(cacheKey, head.clone());
                return head;
            } catch (Exception e) {
                e.printStackTrace();
                return createPlayerHead("MHF_Question");
            }
        });
    }
    
    private static UUID fetchMojangUUID(String playerName) throws Exception {
        java.net.URL url = new java.net.URL("https://api.mojang.com/users/profiles/minecraft/" + playerName);
        try (java.io.BufferedReader reader = new java.io.BufferedReader(
                new java.io.InputStreamReader(url.openStream()))) {
            StringBuilder response = new StringBuilder();
            String line;
            while ((line = reader.readLine()) != null) {
                response.append(line);
            }
            JsonObject json = gson.fromJson(response.toString(), JsonObject.class);
            String id = json.get("id").getAsString();
            return UUID.fromString(
                id.replaceFirst(
                    "(\\w{8})(\\w{4})(\\w{4})(\\w{4})(\\w{12})",
                    "$1-$2-$3-$4-$5"
                )
            );
        }
    }
    
    private static String encodeTexture(String textureUrl) {
        JsonObject textureJson = new JsonObject();
        JsonObject textures = new JsonObject();
        JsonObject skin = new JsonObject();
        skin.addProperty("url", textureUrl);
        textures.add("SKIN", skin);
        textureJson.add("textures", textures);
        return Base64.getEncoder().encodeToString(gson.toJson(textureJson).getBytes());
    }
    
    public static void clearCache() {
        textureCache.clear();
        profileCache.clear();
    }
    
    public static int getCacheSize() {
        return textureCache.size();
    }
    
    public static ItemStack createSuccessHead() {
        return createPlayerHead("MHF_Yes");
    }
    
    public static ItemStack createErrorHead() {
        return createPlayerHead("MHF_No");
    }
    
    public static ItemStack createInfoHead() {
        return createPlayerHead("MHF_Question");
    }
    
    public static ItemStack createWarningHead() {
        return createPlayerHead("MHF_Exclamation");
    }
    
    public static ItemStack createArrowLeft() {
        return createPlayerHead("MHF_ArrowLeft");
    }
    
    public static ItemStack createArrowRight() {
        return createPlayerHead("MHF_ArrowRight");
    }
    
    public static ItemStack createArrowUp() {
        return createPlayerHead("MHF_ArrowUp");
    }
    
    public static ItemStack createArrowDown() {
        return createPlayerHead("MHF_ArrowDown");
    }
}
