package com.example.showcase;

import com.example.showcase.ui.chat.ChatPrompt;
import com.example.showcase.ui.chat.InteractiveMessage;
import com.example.showcase.ui.hud.Animator;
import com.example.showcase.ui.hud.BossBarTimer;
import com.example.showcase.ui.hud.Hologram;
import com.example.showcase.ui.hud.TextFx;
import com.example.showcase.ui.hud.ToastNotification;
import com.example.showcase.ui.menus.DialogMenu;
import com.example.showcase.ui.menus.PaginatedMenu;
import com.example.showcase.ui.pack.ResourcePackService;
import com.example.showcase.ui.textures.GuiFont;
import com.example.showcase.ui.textures.GuiIcons;
import com.example.showcase.ui.util.ItemBuilder;
import com.example.showcase.ui.util.Tasks;
import io.papermc.paper.threadedregions.scheduler.ScheduledTask;
import net.kyori.adventure.bossbar.BossBar;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;
import net.kyori.adventure.text.format.TextColor;
import org.bukkit.Material;
import org.bukkit.command.Command;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.java.JavaPlugin;

import java.net.URI;
import java.time.Duration;
import java.util.List;

/**
 * Shows the CraftCN components working together. Run {@code /showcase <name>} in game:
 * <ul>
 *     <li>{@code shop}: a paginated menu, with a textured title when the resource pack is loaded</li>
 *     <li>{@code form}: a native dialog with text, toggle and number inputs</li>
 *     <li>{@code toast}: an achievement-style popup</li>
 *     <li>{@code fx}: a shimmering actionbar animation for five seconds</li>
 *     <li>{@code timer}: a 30-second boss bar countdown</li>
 *     <li>{@code prompt}: waits for the next chat message</li>
 *     <li>{@code hologram}: a floating label at your position</li>
 * </ul>
 */
public final class ShowcasePlugin extends JavaPlugin implements Listener {

    private ResourcePackService packs;

    @Override
    public void onEnable() {
        saveDefaultConfig();

        String url = getConfig().getString("pack.url", "");
        String sha1 = getConfig().getString("pack.sha1", "");
        if (!url.isBlank() && !sha1.isBlank()) {
            packs = new ResourcePackService(this, URI.create(url), sha1, false,
                    Component.text("CraftCN GUI pack adds custom menus and icons"));
            packs.register();
            GuiIcons.setPackCheck(packs::hasPack);
        }

        getServer().getPluginManager().registerEvents(this, this);
    }

    @EventHandler
    public void onJoin(PlayerJoinEvent event) {
        // Recognised players get a welcome that is clickable without exposing a command.
        InteractiveMessage.from("Try ")
                .append(Component.text("/showcase shop", NamedTextColor.AQUA))
                .clickCallback(audience -> {
                    if (audience instanceof Player player) {
                        openShop(player);
                    }
                })
                .send(event.getPlayer());
    }

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        if (!(sender instanceof Player player)) {
            sender.sendMessage("Run this in game.");
            return true;
        }

        String what = args.length == 0 ? "shop" : args[0].toLowerCase();
        switch (what) {
            case "shop" -> openShop(player);
            case "form" -> openForm(player);
            case "toast" -> ToastNotification.to(player)
                    .title(Component.text("Achievement unlocked", NamedTextColor.GOLD))
                    .description("Showcase toast")
                    .icon(Material.DIAMOND)
                    .frame(ToastNotification.Frame.CHALLENGE)
                    .send(this);
            case "fx" -> playActionBar(player);
            case "timer" -> startTimer(player);
            case "prompt" -> askName(player);
            case "hologram" -> Hologram.spawn(this, player.getLocation().add(0, 2, 0),
                    TextFx.gradient("CraftCN", TextColor.color(0xFFD700), TextColor.color(0x00CED1)));
            default -> player.sendMessage(Component.text("Unknown showcase: " + what, NamedTextColor.RED));
        }
        return true;
    }

    private void openShop(Player player) {
        Tasks.run(this, player, () -> {
            ShopMenu shop = new ShopMenu(this, player, packs != null && packs.hasPack(player));
            shop.open();
        });
    }

    private void openForm(Player player) {
        DialogMenu.form(Component.text("Create a quest"))
                .text(Component.text("Fill in the details below."))
                .textField("name", Component.text("Quest name"), "Dragon slayer")
                .toggle("repeatable", Component.text("Repeatable"), false)
                .number("reward", Component.text("Reward"), 0f, 100f, 25f)
                .submitLabel(Component.text("Create"))
                .open(player)
                .thenAccept(response -> Tasks.run(this, player, () -> player.sendMessage(
                        Component.text("Created '" + response.text("name") + "', reward "
                                + (int) response.number("reward") + ", repeatable: " + response.toggle("repeatable")))));
    }

    private void playActionBar(Player player) {
        List<Component> frames = TextFx.shimmerFrames("Welcome to the showcase", 24,
                TextColor.color(0x7DF9FF), TextColor.color(0xFFFFFF));

        Animator<Component> animator = Animator.of(frames, 4);
        ScheduledTask task = animator.forPlayer(this, player, player::sendActionBar);
        Tasks.later(this, player, 100, task::cancel);
    }

    private void startTimer(Player player) {
        BossBarTimer timer = BossBarTimer.create(this, "Sudden death in", Duration.ofSeconds(30))
                .color(BossBar.Color.RED)
                .onComplete(done -> getServer().broadcast(Component.text("Time is up!")));
        timer.addViewer(player);
        timer.start();
    }

    private void askName(Player player) {
        new ChatPrompt(this, player, Component.text("What is your name?"),
                name -> player.sendMessage(Component.text("Hello, " + name + "!", NamedTextColor.GREEN)),
                Duration.ofSeconds(20))
                .start();
    }

    /** A paginated shop. Viewers with the resource pack get a plate behind the title. */
    private static final class ShopMenu extends PaginatedMenu<String> {

        ShopMenu(Plugin plugin, Player player, boolean textured) {
            super(plugin, player, titleFor(textured), 6);
            setSource(List.of("Iron sword", "Golden apple", "Ender pearl", "Elytra", "Totem", "Shield"));
            setRenderer(item -> ItemBuilder.from(Material.PAPER).name(item).lore("Click to buy").build());
            onItemClick((item, click) -> click.player().sendMessage(Component.text("Bought " + item)));
        }

        private static Component titleFor(boolean textured) {
            Component title = Component.text("Shop");
            return textured ? GuiFont.title(GuiFont.Plate.PRIMARY, title) : title;
        }
    }
}
