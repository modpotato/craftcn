package com.craftcn.ui.forms;

import com.craftcn.ui.UITheme;
import com.craftcn.ui.core.BaseMenu;
import com.craftcn.ui.core.Button;
import com.craftcn.ui.util.ItemBuilder;
import net.kyori.adventure.text.Component;
import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.inventory.InventoryType;
import org.bukkit.event.inventory.PrepareAnvilEvent;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.view.AnvilView;
import org.bukkit.plugin.Plugin;

import java.util.concurrent.CompletableFuture;
import java.util.function.Consumer;

/**
 * Asks the player to type a line of text in an anvil. The player edits the name in the anvil's text field and
 * clicks the result to submit. Closing the anvil without submitting cancels {@link #future()}.
 * <p>
 * Usage: {@code new FormMenu(plugin, player, "Rename", "Old name", text -> ...).open();}
 */
public class FormMenu extends BaseMenu {

    public static final String SUBMIT_LABEL = "Submit";
    public static final String PLACEHOLDER = "Type here";

    private static final int FIELD_SLOT = 0;
    private static final int SUBMIT_SLOT = 2;

    private final String initialText;
    private final Consumer<String> onSubmit;
    private final CompletableFuture<String> future = new CompletableFuture<>();
    private boolean submitted;

    public FormMenu(Plugin plugin, Player player, String title, String initialText, Consumer<String> onSubmit) {
        this(plugin, player, Component.text(title), initialText, onSubmit);
    }

    /**
     * @param initialText text pre-filled in the input field, may be {@code null}
     * @param onSubmit    receives the submitted text on the viewer's thread
     */
    public FormMenu(Plugin plugin, Player player, Component title, String initialText, Consumer<String> onSubmit) {
        // An anvil has its own inventory type, so the size passed here is unused.
        super(plugin, player, title, 3);
        this.initialText = initialText == null ? "" : initialText;
        this.onSubmit = onSubmit;
    }

    /** Completes with the submitted text, or is cancelled when the anvil closes unsubmitted. */
    public CompletableFuture<String> future() {
        return future;
    }

    @Override
    protected Inventory createInventory() {
        return Bukkit.createInventory(this, InventoryType.ANVIL, title);
    }

    @Override
    protected void build() {
        // The first item seeds the text field: its name is the text the player starts from.
        String seed = initialText.isEmpty() ? PLACEHOLDER : initialText;
        setItem(FIELD_SLOT, ItemBuilder.from(Material.PAPER).name(seed).build());
        setButton(SUBMIT_SLOT, Button.of(submitIcon(), this::submit));
    }

    @Override
    protected void onPrepareAnvil(PrepareAnvilEvent event) {
        // Always offer the submit icon as the result, whatever the text is.
        event.setResult(submitIcon());
    }

    @Override
    protected void onClose() {
        if (!submitted) {
            future.cancel(false);
        }
    }

    private ItemStack submitIcon() {
        return ItemBuilder.from(UITheme.SUCCESS_ICON)
                .name(SUBMIT_LABEL)
                .lore("Click to submit")
                .build();
    }

    private void submit(Button.Click click) {
        if (submitted) {
            return;
        }
        submitted = true;

        String text = click.event().getView() instanceof AnvilView anvil && anvil.getRenameText() != null
                ? anvil.getRenameText()
                : "";

        onSubmit.accept(text);
        future.complete(text);
        close();
    }
}
