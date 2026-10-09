package com.craftcn.ui.menus;

import io.papermc.paper.dialog.Dialog;
import io.papermc.paper.dialog.DialogResponseView;
import io.papermc.paper.registry.data.dialog.ActionButton;
import io.papermc.paper.registry.data.dialog.DialogBase;
import io.papermc.paper.registry.data.dialog.action.DialogAction;
import io.papermc.paper.registry.data.dialog.body.DialogBody;
import io.papermc.paper.registry.data.dialog.input.DialogInput;
import io.papermc.paper.registry.data.dialog.type.DialogType;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.event.ClickCallback;
import org.bukkit.entity.Player;

import java.time.Duration;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

/**
 * Native Paper dialogs (Minecraft 1.21.6 and newer). The client draws them, so they need no inventory, look
 * the same on every screen size and accept typed input directly.
 * <p>
 * Buttons stay clickable for ten minutes. Futures complete with {@code false} or an empty response after two
 * minutes, because the server is not told when a player dismisses a dialog with Escape.
 */
public final class DialogMenu {

    public static final String CLOSE_LABEL = "Close";
    public static final String SUBMIT_LABEL = "Submit";

    private static final ClickCallback.Options BUTTON_OPTIONS =
            ClickCallback.Options.builder().lifetime(Duration.ofMinutes(10)).build();
    private static final long TIMEOUT_MINUTES = 2;

    private DialogMenu() {
    }

    /** Shows a dialog with one button that closes it. */
    public static void notice(Player player, Component title, Component body, Component buttonLabel) {
        ActionButton button = ActionButton.builder(buttonLabel).build();
        player.showDialog(Dialog.create(builder -> builder.empty()
                .base(base(title, bodyOf(body), List.of()))
                .type(DialogType.notice(button))));
    }

    /**
     * Shows a yes/no dialog.
     *
     * @return {@code true} if the player picks yes, {@code false} for no or on timeout
     */
    public static CompletableFuture<Boolean> confirm(Player player, Component title, Component body,
                                                     Component yesLabel, Component noLabel) {
        CompletableFuture<Boolean> answer = new CompletableFuture<>();

        ActionButton yes = ActionButton.builder(yesLabel)
                .action(DialogAction.customClick((view, audience) -> answer.complete(true), BUTTON_OPTIONS))
                .build();
        ActionButton no = ActionButton.builder(noLabel)
                .action(DialogAction.customClick((view, audience) -> answer.complete(false), BUTTON_OPTIONS))
                .build();

        player.showDialog(Dialog.create(builder -> builder.empty()
                .base(base(title, bodyOf(body), List.of()))
                .type(DialogType.confirmation(yes, no))));

        return answer.completeOnTimeout(false, TIMEOUT_MINUTES, TimeUnit.MINUTES);
    }

    /**
     * Shows a grid of options. Each option runs its action when clicked.
     *
     * @param columns the number of button columns, at least 1
     */
    public static void choose(Player player, Component title, Component body, int columns, List<Option> options) {
        List<ActionButton> buttons = new ArrayList<>();
        for (Option option : options) {
            buttons.add(ActionButton.builder(option.label())
                    .tooltip(option.tooltip())
                    .action(DialogAction.customClick((view, audience) -> option.action().accept(player), BUTTON_OPTIONS))
                    .build());
        }

        ActionButton exit = ActionButton.builder(Component.text(CLOSE_LABEL)).build();
        DialogType type = DialogType.multiAction(buttons).exitAction(exit).columns(Math.max(1, columns)).build();

        player.showDialog(Dialog.create(builder -> builder.empty()
                .base(base(title, bodyOf(body), List.of()))
                .type(type)));
    }

    /**
     * An option in {@link #choose(Player, Component, Component, int, List)}.
     *
     * @param label   the button text
     * @param tooltip shown on hover, may be empty
     * @param action  runs on the viewer's thread when the option is clicked
     */
    public record Option(Component label, Component tooltip, Consumer<Player> action) {
        public Option {
            Objects.requireNonNull(label, "label");
            Objects.requireNonNull(action, "action");
            tooltip = tooltip == null ? Component.empty() : tooltip;
        }

        public static Option of(Component label, Consumer<Player> action) {
            return new Option(label, Component.empty(), action);
        }
    }

    /** Starts a form with text, toggle and number inputs. */
    public static Form form(Component title) {
        return new Form(title);
    }

    /** The values a player submitted in a {@link Form}. */
    public record Response(Map<String, Object> values) {

        public String text(String key) {
            Object value = values.get(key);
            return value == null ? "" : value.toString();
        }

        public boolean toggle(String key) {
            return Boolean.TRUE.equals(values.get(key));
        }

        public float number(String key) {
            Object value = values.get(key);
            return value instanceof Float number ? number : 0f;
        }
    }

    /** A dialog with inputs. Build it with {@link DialogMenu#form(Component)}, then {@link #open(Player)} it. */
    public static final class Form {

        private enum Kind { TEXT, TOGGLE, NUMBER }

        private final Component title;
        private final List<DialogBody> body = new ArrayList<>();
        private final List<DialogInput> inputs = new ArrayList<>();
        private final Map<String, Kind> kinds = new LinkedHashMap<>();
        private Component submitLabel = Component.text(SUBMIT_LABEL);

        private Form(Component title) {
            this.title = title;
        }

        /** Adds a line of text above the inputs. */
        public Form text(Component text) {
            body.add(DialogBody.plainMessage(text));
            return this;
        }

        /** Adds a single-line text field. */
        public Form textField(String key, Component label, String initial) {
            inputs.add(DialogInput.text(key, label).initial(initial == null ? "" : initial).build());
            kinds.put(key, Kind.TEXT);
            return this;
        }

        /** Adds a checkbox. */
        public Form toggle(String key, Component label, boolean initial) {
            inputs.add(DialogInput.bool(key, label).initial(initial).build());
            kinds.put(key, Kind.TOGGLE);
            return this;
        }

        /** Adds a slider between {@code min} and {@code max}. */
        public Form number(String key, Component label, float min, float max, float initial) {
            inputs.add(DialogInput.numberRange(key, label, min, max).initial(initial).build());
            kinds.put(key, Kind.NUMBER);
            return this;
        }

        public Form submitLabel(Component label) {
            this.submitLabel = label;
            return this;
        }

        /**
         * Shows the form and completes with the submitted values, or with an empty response if the
         * player dismisses it (see the class docs).
         */
        public CompletableFuture<Response> open(Player player) {
            CompletableFuture<Response> answer = new CompletableFuture<>();

            ActionButton submit = ActionButton.builder(submitLabel)
                    .action(DialogAction.customClick((view, audience) -> answer.complete(read(view)), BUTTON_OPTIONS))
                    .build();

            player.showDialog(Dialog.create(builder -> builder.empty()
                    .base(base(title, body, inputs))
                    .type(DialogType.notice(submit))));

            return answer.completeOnTimeout(new Response(Map.of()), TIMEOUT_MINUTES, TimeUnit.MINUTES);
        }

        private Response read(DialogResponseView view) {
            Map<String, Object> values = new LinkedHashMap<>();
            kinds.forEach((key, kind) -> values.put(key, switch (kind) {
                case TEXT -> view.getText(key);
                case TOGGLE -> view.getBoolean(key);
                case NUMBER -> view.getFloat(key);
            }));
            return new Response(values);
        }
    }

    private static List<DialogBody> bodyOf(Component text) {
        return List.of(DialogBody.plainMessage(text));
    }

    private static DialogBase base(Component title, List<DialogBody> bodies, List<DialogInput> inputs) {
        return DialogBase.builder(title)
                .canCloseWithEscape(true)
                .body(bodies)
                .inputs(inputs)
                .build();
    }
}
