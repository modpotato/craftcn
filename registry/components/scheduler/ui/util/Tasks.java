package com.craftcn.ui.util;

import io.papermc.paper.threadedregions.scheduler.ScheduledTask;
import org.bukkit.Bukkit;
import org.bukkit.Location;
import org.bukkit.entity.Player;
import org.bukkit.plugin.Plugin;

import java.util.function.Consumer;

/**
 * Folia-safe scheduling helpers built on Paper's region schedulers.
 * <p>
 * Work that touches a player (inventories, sounds, scoreboards, packets) must run on that player's
 * thread, so prefer the {@code player} methods over {@code Bukkit.getScheduler()}. On plain Paper all
 * of these run on the main thread, so the same code works on both.
 * <p>
 * The {@code player} methods return {@code null} when the player has already been removed.
 */
public final class Tasks {

    private Tasks() {
    }

    /** Runs {@code task} on the player's thread as soon as possible. */
    public static ScheduledTask run(Plugin plugin, Player player, Runnable task) {
        return player.getScheduler().run(plugin, scheduled -> task.run(), () -> { });
    }

    /** Runs {@code task} on the player's thread after {@code delayTicks} (20 ticks = 1 second). */
    public static ScheduledTask later(Plugin plugin, Player player, long delayTicks, Runnable task) {
        return player.getScheduler().runDelayed(plugin, scheduled -> task.run(), () -> { }, Math.max(1, delayTicks));
    }

    /** Runs {@code task} on the player's thread every {@code periodTicks}. Cancel via the returned task. */
    public static ScheduledTask repeat(Plugin plugin, Player player, long delayTicks, long periodTicks,
                                       Consumer<ScheduledTask> task) {
        return player.getScheduler().runAtFixedRate(plugin, task, () -> { }, Math.max(1, delayTicks), Math.max(1, periodTicks));
    }

    /** Runs {@code task} on the region that owns {@code location}. */
    public static ScheduledTask at(Plugin plugin, Location location, Runnable task) {
        return Bukkit.getRegionScheduler().run(plugin, location, scheduled -> task.run());
    }

    /** Runs {@code task} on the global region, for logic that is not tied to a world position. */
    public static ScheduledTask global(Plugin plugin, Runnable task) {
        return Bukkit.getGlobalRegionScheduler().run(plugin, scheduled -> task.run());
    }

    /** Runs {@code task} on the global region every {@code periodTicks}. Cancel via the returned task. */
    public static ScheduledTask globalRepeat(Plugin plugin, long delayTicks, long periodTicks,
                                             Consumer<ScheduledTask> task) {
        return Bukkit.getGlobalRegionScheduler().runAtFixedRate(plugin, task, Math.max(1, delayTicks), Math.max(1, periodTicks));
    }

    /** Runs {@code task} off the server thread. Do not touch Bukkit state from here. */
    public static ScheduledTask async(Plugin plugin, Runnable task) {
        return Bukkit.getAsyncScheduler().runNow(plugin, scheduled -> task.run());
    }
}
