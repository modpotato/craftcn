package com.craftcn.ui.hud;

import com.craftcn.ui.util.Tasks;
import io.papermc.paper.threadedregions.scheduler.ScheduledTask;
import org.bukkit.entity.Player;
import org.bukkit.plugin.Plugin;

import java.util.List;
import java.util.function.Consumer;

/**
 * Plays a list of frames at a fixed rate: titles, actionbars, boss bar labels, item icons and so on.
 * <p>
 * Frames are delivered on the viewer's thread by {@link #forPlayer}, or on the global region by {@link #global}.
 * Each call starts its own playback, so one {@code Animator} can drive several viewers. Cancel the returned task
 * to stop.
 *
 * @param <T> the frame type, such as {@code Component} or {@code ItemStack}
 */
public final class Animator<T> {

    private final List<T> frames;
    private final long periodTicks;
    private boolean loop = true;

    private Animator(List<T> frames, long periodTicks) {
        if (frames.isEmpty()) {
            throw new IllegalArgumentException("An animation needs at least one frame");
        }
        this.frames = List.copyOf(frames);
        this.periodTicks = Math.max(1, periodTicks);
    }

    /** An animator that shows {@code frames} one after another, {@code periodTicks} apart (20 ticks = 1 second). */
    @SafeVarargs
    public static <T> Animator<T> of(long periodTicks, T... frames) {
        return new Animator<>(List.of(frames), periodTicks);
    }

    public static <T> Animator<T> of(List<T> frames, long periodTicks) {
        return new Animator<>(frames, periodTicks);
    }

    /** Whether playback restarts after the last frame. Enabled by default. */
    public Animator<T> loop(boolean loop) {
        this.loop = loop;
        return this;
    }

    /** Plays on {@code player}'s thread. Playback ends when the player leaves. */
    public ScheduledTask forPlayer(Plugin plugin, Player player, Consumer<T> onFrame) {
        int[] cursor = {0};
        return Tasks.repeat(plugin, player, 1, periodTicks, task -> advance(task, cursor, onFrame));
    }

    /** Plays on the global region. {@code onFrame} must not touch individual players. */
    public ScheduledTask global(Plugin plugin, Consumer<T> onFrame) {
        int[] cursor = {0};
        return Tasks.globalRepeat(plugin, 1, periodTicks, task -> advance(task, cursor, onFrame));
    }

    private void advance(ScheduledTask task, int[] cursor, Consumer<T> onFrame) {
        if (cursor[0] >= frames.size()) {
            if (!loop) {
                task.cancel();
                return;
            }
            cursor[0] = 0;
        }
        onFrame.accept(frames.get(cursor[0]++));
    }
}
