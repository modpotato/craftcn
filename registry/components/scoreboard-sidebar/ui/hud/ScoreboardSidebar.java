package com.craftcn.ui.hud;

import io.papermc.paper.scoreboard.numbers.NumberFormat;
import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.scoreboard.Criteria;
import org.bukkit.scoreboard.DisplaySlot;
import org.bukkit.scoreboard.Objective;
import org.bukkit.scoreboard.Scoreboard;
import org.bukkit.scoreboard.Team;

import java.util.ArrayList;
import java.util.List;

/**
 * A per-player sidebar. Each line is rendered through a team prefix holding an Adventure component, so a single
 * line can change without resending the whole board. Score numbers are hidden with a blank number format.
 * <p>
 * Each line is keyed by an invisible entry (a colour code), which limits the sidebar to {@link #MAX_LINES} lines.
 */
public class ScoreboardSidebar {

    public static final int MAX_LINES = 15;

    private static final String ENTRY_CODES = "0123456789abcdef";
    private static final String SECTION = "§";

    private final Player player;
    private final Scoreboard scoreboard;
    private final Objective objective;
    private final List<Component> lines = new ArrayList<>();

    public ScoreboardSidebar(Player player, Component title) {
        this.player = player;
        this.scoreboard = Bukkit.getScoreboardManager().getNewScoreboard();
        this.objective = scoreboard.registerNewObjective("craftcn_sidebar", Criteria.DUMMY, title);
        this.objective.setDisplaySlot(DisplaySlot.SIDEBAR);
    }

    public ScoreboardSidebar(Player player, String title) {
        this(player, Component.text(title));
    }

    public void setTitle(Component title) {
        objective.displayName(title);
    }

    /** Replaces every line. Extra lines beyond {@link #MAX_LINES} are ignored. */
    public void setLines(List<Component> newLines) {
        lines.clear();
        lines.addAll(newLines.subList(0, Math.min(newLines.size(), MAX_LINES)));
        render();
    }

    public void addLine(Component line) {
        if (lines.size() < MAX_LINES) {
            lines.add(line);
            render();
        }
    }

    /** Changes one line. Ignored when {@code index} is out of range. */
    public void setLine(int index, Component line) {
        if (index >= 0 && index < lines.size()) {
            lines.set(index, line);
            Team team = team(index);
            team.prefix(line);
        }
    }

    public void removeLine(int index) {
        if (index >= 0 && index < lines.size()) {
            lines.remove(index);
            render();
        }
    }

    /** Shows the sidebar to the player. Call from the player's thread. */
    public void show() {
        player.setScoreboard(scoreboard);
    }

    /** Restores the server's main scoreboard for the player. */
    public void hide() {
        player.setScoreboard(Bukkit.getScoreboardManager().getMainScoreboard());
    }

    private void render() {
        for (int index = 0; index < MAX_LINES; index++) {
            String entry = entry(index);

            if (index < lines.size()) {
                Team team = team(index);
                team.prefix(lines.get(index));
                if (!team.hasEntry(entry)) {
                    team.addEntry(entry);
                }

                // Higher scores sit higher on the board, so the first line gets the largest score.
                objective.getScore(entry).setScore(MAX_LINES - index);
                objective.getScore(entry).numberFormat(NumberFormat.blank());
            } else {
                Team team = scoreboard.getTeam(teamName(index));
                if (team != null) {
                    team.unregister();
                }
                scoreboard.resetScores(entry);
            }
        }
    }

    private Team team(int index) {
        Team team = scoreboard.getTeam(teamName(index));
        return team != null ? team : scoreboard.registerNewTeam(teamName(index));
    }

    private static String teamName(int index) {
        return "craftcn_line_" + index;
    }

    private static String entry(int index) {
        return SECTION + ENTRY_CODES.charAt(index);
    }
}
