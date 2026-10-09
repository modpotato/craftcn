package com.craftcn.ui.hud;

import com.craftcn.ui.util.Tasks;
import io.papermc.paper.threadedregions.scheduler.ScheduledTask;
import net.kyori.adventure.bossbar.BossBar;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import org.bukkit.entity.Player;
import org.bukkit.plugin.Plugin;

import java.time.Duration;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Consumer;
import java.util.function.Function;

/**
 * A countdown shown as a boss bar.
 * <p>
 * Each viewer has their own bar, and every bar is updated on its viewer's thread. That keeps the timer correct on
 * Folia as well as Paper. The timer itself ticks on the global region.
 */
public class BossBarTimer {

    private static final long TICKS_PER_STEP = 5;
    private static final long MILLIS_PER_TICK = 50;

    private final Plugin plugin;
    private final Component title;
    private final Duration total;
    private final Map<Player, BossBar> bars = new ConcurrentHashMap<>();

    private BossBar.Color color = BossBar.Color.BLUE;
    private BossBar.Overlay overlay = BossBar.Overlay.PROGRESS;
    private Function<Duration, Component> label = BossBarTimer::clock;
    private Consumer<BossBarTimer> onComplete = timer -> { };

    private volatile Duration remaining;
    private volatile ScheduledTask ticker;

    public BossBarTimer(Plugin plugin, Component title, Duration duration) {
        this.plugin = plugin;
        this.title = title;
        this.total = duration;
        this.remaining = duration;
    }

    public static BossBarTimer create(Plugin plugin, String title, Duration duration) {
        return new BossBarTimer(plugin, Component.text(title), duration);
    }

    public BossBarTimer color(BossBar.Color color) {
        this.color = color;
        return this;
    }

    public BossBarTimer overlay(BossBar.Overlay overlay) {
        this.overlay = overlay;
        return this;
    }

    /** Formats the time left. The default shows {@code m:ss}. */
    public BossBarTimer label(Function<Duration, Component> label) {
        this.label = label;
        return this;
    }

    /** Runs on the global region when the countdown reaches zero. Use {@link Tasks} to act on players. */
    public BossBarTimer onComplete(Consumer<BossBarTimer> callback) {
        this.onComplete = callback;
        return this;
    }

    /** Shows the bar to {@code player}. Call from any thread. */
    public void addViewer(Player player) {
        BossBar bar = BossBar.bossBar(name(remaining), progress(remaining), color, overlay);
        if (bars.putIfAbsent(player, bar) == null) {
            Tasks.run(plugin, player, () -> player.showBossBar(bar));
        }
    }

    /** Hides the bar from {@code player}. */
    public void removeViewer(Player player) {
        BossBar bar = bars.remove(player);
        if (bar != null) {
            Tasks.run(plugin, player, () -> player.hideBossBar(bar));
        }
    }

    /** Starts the countdown from the full duration. Does nothing if it is already running. */
    public synchronized void start() {
        if (ticker != null) {
            return;
        }
        remaining = total;
        ticker = Tasks.globalRepeat(plugin, 0, TICKS_PER_STEP, this::tick);
    }

    /** Stops the countdown and keeps the bars visible at their current value. */
    public synchronized void stop() {
        if (ticker != null) {
            ticker.cancel();
            ticker = null;
        }
    }

    public boolean isRunning() {
        return ticker != null;
    }

    /** Time left in the countdown. */
    public Duration remaining() {
        return remaining;
    }

    private void tick(ScheduledTask task) {
        Duration next = remaining.minusMillis(TICKS_PER_STEP * MILLIS_PER_TICK);
        remaining = next.isNegative() ? Duration.ZERO : next;

        for (Map.Entry<Player, BossBar> entry : bars.entrySet()) {
            BossBar bar = entry.getValue();
            Tasks.run(plugin, entry.getKey(), () -> {
                bar.name(name(remaining));
                bar.progress(progress(remaining));
            });
        }

        if (remaining.isZero()) {
            task.cancel();
            ticker = null;
            onComplete.accept(this);
        }
    }

    private Component name(Duration left) {
        return title.append(Component.text("  ")).append(label.apply(left));
    }

    private float progress(Duration left) {
        if (total.isZero()) {
            return 0f;
        }
        float ratio = (float) left.toMillis() / total.toMillis();
        return Math.max(0f, Math.min(1f, ratio));
    }

    private static Component clock(Duration left) {
        long seconds = Math.max(0, left.toSeconds());
        return Component.text(String.format("%d:%02d", seconds / 60, seconds % 60), NamedTextColor.WHITE);
    }
}
