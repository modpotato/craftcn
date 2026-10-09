package com.craftcn.ui.textures;

import net.kyori.adventure.key.Key;
import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.NamedTextColor;

/**
 * Helpers for the {@code craftcn:gui} font that {@code craftcn pack} generates.
 * <p>
 * Title plates are bitmap glyphs drawn in the container title, which is the same technique resource pack GUIs use
 * for custom screen art. Text is moved horizontally with invisible shift glyphs. The code points must match
 * {@code src/pack/font.rs} in the CraftCN CLI, so change both together.
 * <p>
 * Without the resource pack, the glyphs are missing and show as boxes. Show {@link #plainTitle(Component)} to
 * players who lack the pack; {@link TexturedMenu} does this for you when you pass a {@link GuiIcons} pack check.
 */
public final class GuiFont {

    public static final Key FONT = Key.key("craftcn", "gui");

    private static final char SHIFT_POSITIVE_BASE = '';
    private static final char SHIFT_NEGATIVE_BASE = '';
    private static final int SHIFT_BITS = 8;
    private static final int MAX_SHIFT_PER_GLYPH = (1 << SHIFT_BITS) - 1;

    /** Container titles start this many pixels from the window's left edge. */
    private static final int TITLE_X = 8;
    /** Distance from the plate's left edge to the title text. */
    private static final int PLATE_INSET = 6;

    private GuiFont() {
    }

    /** A title plate. The width is part of the generated font and must not change without regenerating it. */
    public enum Plate {
        PRIMARY('', 176),
        ACCENT('', 176),
        DANGER('', 176),
        COMPACT('', 96);

        private final char glyph;
        private final int width;

        Plate(char glyph, int width) {
            this.glyph = glyph;
            this.width = width;
        }

        public int width() {
            return width;
        }

        char glyph() {
            return glyph;
        }
    }

    /**
     * A container title drawn on a plate. The plate covers the window's left edge, and {@code text} sits inside
     * it, at the usual title font.
     */
    public static Component title(Plate plate, Component text) {
        return shift(-TITLE_X)
                .append(glyph(plate))
                .append(shift(PLATE_INSET - plate.width()))
                .append(text);
    }

    /** A plain title for players who do not have the resource pack. */
    public static Component plainTitle(Component text) {
        return text;
    }

    /** The plate glyph alone, for custom layouts. */
    public static Component glyph(Plate plate) {
        return Component.text(String.valueOf(plate.glyph()), NamedTextColor.WHITE).font(FONT);
    }

    /**
     * Moves the text that follows by {@code pixels}: positive moves right, negative moves left. Shifts beyond 255
     * pixels are split across several glyphs.
     */
    public static Component shift(int pixels) {
        StringBuilder glyphs = new StringBuilder();
        int remaining = Math.abs(pixels);

        while (remaining > 0) {
            int chunk = Math.min(remaining, MAX_SHIFT_PER_GLYPH);
            for (int bit = 0; bit < SHIFT_BITS; bit++) {
                if ((chunk & (1 << bit)) != 0) {
                    char base = pixels > 0 ? SHIFT_POSITIVE_BASE : SHIFT_NEGATIVE_BASE;
                    glyphs.append((char) (base + bit));
                }
            }
            remaining -= chunk;
        }

        return glyphs.isEmpty() ? Component.empty() : Component.text(glyphs.toString()).font(FONT);
    }
}
