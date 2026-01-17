package com.craftcn.ui.hud;

import net.kyori.adventure.bossbar.BossBar;
import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.plugin.Plugin;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

public class BossBarTimer {
    
    private final Plugin plugin;
    private final BossBar bossBar;
    private final List<Player> viewers;
    private int taskId = -1;
    private long duration;
    private TimeUnit unit;
    private long remaining;
    private Consumer<BossBarTimer> onComplete;
    private Consumer<BossBarTimer> onTick;
    private boolean running;
    
    public BossBarTimer(Plugin plugin, Component title, float progress, BossBar.Color color, BossBar.Overlay overlay) {
        this.plugin = plugin;
        this.bossBar = BossBar.bossBar(title, progress, color, overlay);
        this.viewers = new ArrayList<>();
        this.running = false;
    }
    
    public static BossBarTimer create(Plugin plugin, String title) {
        return create(plugin, Component.text(title));
    }
    
    public static BossBarTimer create(Plugin plugin, Component title) {
        return new BossBarTimer(plugin, title, 1.0f, BossBar.Color.BLUE, BossBar.Overlay.PROGRESS);
    }
    
    public static BossBarTimer countdown(Plugin plugin, Component title, int seconds) {
        BossBarTimer timer = new BossBarTimer(plugin, title, 1.0f, BossBar.Color.RED, BossBar.Overlay.NOTCHED_10);
        timer.duration = seconds;
        timer.unit = TimeUnit.SECONDS;
        return timer;
    }
    
    public static BossBarTimer countdown(Plugin plugin, String title, int seconds) {
        return countdown(plugin, Component.text(title), seconds);
    }
    
    public BossBarTimer setTitle(String title) {
        return setTitle(Component.text(title));
    }
    
    public BossBarTimer setTitle(Component title) {
        bossBar.name(title);
        return this;
    }
    
    public BossBarTimer setColor(BossBar.Color color) {
        bossBar.color(color);
        return this;
    }
    
    public BossBarTimer setOverlay(BossBar.Overlay overlay) {
        bossBar.overlay(overlay);
        return this;
    }
    
    public BossBarTimer setProgress(float progress) {
        bossBar.progress(progress);
        return this;
    }
    
    public BossBarTimer setDuration(long duration, TimeUnit unit) {
        this.duration = duration;
        this.unit = unit;
        return this;
    }
    
    public BossBarTimer setOnComplete(Consumer<BossBarTimer> callback) {
        this.onComplete = callback;
        return this;
    }
    
    public BossBarTimer setOnTick(Consumer<BossBarTimer> callback) {
        this.onTick = callback;
        return this;
    }
    
    public BossBarTimer addPlayer(Player player) {
        bossBar.viewer(player);
        if (!viewers.contains(player)) {
            viewers.add(player);
        }
        return this;
    }
    
    public BossBarTimer removePlayer(Player player) {
        bossBar.removeViewer(player);
        viewers.remove(player);
        return this;
    }
    
    public BossBarTimer addAllPlayers() {
        for (Player player : Bukkit.getOnlinePlayers()) {
            addPlayer(player);
        }
        return this;
    }
    
    public List<Player> getViewers() {
        return new ArrayList<>(viewers);
    }
    
    public void start() {
        if (running || duration <= 0) {
            return;
        }
        
        running = true;
        remaining = unit.toSeconds(duration);
        
        taskId = Bukkit.getScheduler().runTaskTimer(plugin, () -> {
            if (remaining <= 0) {
                complete();
                return;
            }
            
            remaining--;
            float progress = (float) remaining / unit.toSeconds(duration);
            bossBar.progress(Math.max(0, Math.min(1, progress)));
            
            if (onTick != null) {
                onTick.accept(this);
            }
        }, 0L, 20L).getTaskId();
    }
    
    public void pause() {
        if (running && taskId != -1) {
            Bukkit.getScheduler().cancelTask(taskId);
            running = false;
        }
    }
    
    public void resume() {
        if (!running && remaining > 0) {
            start();
        }
    }
    
    public void stop() {
        pause();
        remaining = 0;
        bossBar.progress(0);
    }
    
    public void complete() {
        stop();
        if (onComplete != null) {
            onComplete.accept(this);
        }
    }
    
    public boolean isRunning() {
        return running;
    }
    
    public long getRemaining(TimeUnit unit) {
        return unit.convert(remaining, TimeUnit.SECONDS);
    }
    
    public float getProgress() {
        return bossBar.progress();
    }
    
    public BossBar getBossBar() {
        return bossBar;
    }
    
    public void destroy() {
        stop();
        for (Player player : viewers) {
            bossBar.removeViewer(player);
        }
        viewers.clear();
    }
}
