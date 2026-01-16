package com.craftcn.ui.hud;

import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.entity.Player;
import org.bukkit.scoreboard.DisplaySlot;
import org.bukkit.scoreboard.Objective;
import org.bukkit.scoreboard.Score;
import org.bukkit.scoreboard.Scoreboard;
import org.bukkit.scoreboard.Team;

import java.util.ArrayList;
import java.util.List;

public class ScoreboardSidebar {
    
    private final Scoreboard scoreboard;
    private final Objective objective;
    private final Player player;
    private final List<Team> teams;
    private final List<String> lines;
    private String title;
    
    public ScoreboardSidebar(Player player, String title) {
        this.player = player;
        this.title = title;
        this.scoreboard = Bukkit.getScoreboardManager().getNewScoreboard();
        this.objective = scoreboard.registerNewObjective("sidebar", "dummy", 
            Component.text(title));
        this.objective.setDisplaySlot(DisplaySlot.SIDEBAR);
        this.teams = new ArrayList<>();
        this.lines = new ArrayList<>();
    }
    
    public void setTitle(String title) {
        this.title = title;
        objective.displayName(Component.text(title));
    }
    
    public void addLine(String line) {
        lines.add(line);
        updateLines();
    }
    
    public void setLine(int index, String line) {
        if (index >= 0 && index < lines.size()) {
            lines.set(index, line);
            updateLines();
        }
    }
    
    public void removeLine(int index) {
        if (index >= 0 && index < lines.size()) {
            lines.remove(index);
            updateLines();
        }
    }
    
    public void clearLines() {
        lines.clear();
        updateLines();
    }
    
    private void updateLines() {
        for (Team team : teams) {
            team.unregister();
        }
        teams.clear();
        
        for (int i = 0; i < lines.size(); i++) {
            String line = lines.get(lines.size() - 1 - i);
            String teamName = "line_" + i;
            
            Team team = scoreboard.registerNewTeam(teamName);
            teams.add(team);
            
            String entry = String.valueOf(net.kyori.adventure.text.format.NamedTextColor.values()[i % 16]);
            team.addEntry(entry);
            team.prefix(Component.text(line));
            
            objective.getScore(entry).setScore(i);
        }
    }
    
    public void show() {
        player.setScoreboard(scoreboard);
    }
    
    public void hide() {
        if (player.getScoreboard() == scoreboard) {
            player.setScoreboard(Bukkit.getScoreboardManager().getMainScoreboard());
        }
    }
    
    public void destroy() {
        hide();
        for (Team team : teams) {
            team.unregister();
        }
        objective.unregister();
    }
    
    public boolean isVisible() {
        return player.getScoreboard() == scoreboard;
    }
    
    public int getLineCount() {
        return lines.size();
    }
    
    public List<String> getLines() {
        return new ArrayList<>(lines);
    }
}
