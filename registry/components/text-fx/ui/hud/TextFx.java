package com.craftcn.ui.hud;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.format.TextColor;
import net.kyori.adventure.text.format.TextDecoration;
import net.kyori.adventure.util.HSVLike;

import java.util.ArrayList;
import java.util.List;

/**
 * Text effects built on Adventure: gradients, rainbows, shimmers, pulses and typewriter reveals.
 * <p>
 * Static effects return one {@link Component}. Animated effects return a list of frames, which you play with
 * {@code Animator} (see the {@code animation} component).
 */
public final class TextFx {

    public static final String CURSOR = "▌";

    private static final float RAINBOW_SPREAD = 0.6f;
    private static final float SHIMMER_WIDTH = 2.5f;

    private TextFx() {
    }

    /** Colours each character from {@code from} to {@code to}. */
    public static Component gradient(String text, TextColor from, TextColor to) {
        Component result = Component.empty();
        int length = text.length();
        for (int index = 0; index < length; index++) {
            float t = length == 1 ? 0f : (float) index / (length - 1);
            result = result.append(styled(text.charAt(index), TextColor.lerp(t, from, to)));
        }
        return result;
    }

    /**
     * A rainbow across the text. {@code phase} is in [0, 1): step it between frames to make the colours flow.
     */
    public static Component rainbow(String text, float phase) {
        Component result = Component.empty();
        int length = Math.max(1, text.length());
        for (int index = 0; index < text.length(); index++) {
            float hue = ((float) index / length * RAINBOW_SPREAD + phase) % 1f;
            result = result.append(styled(text.charAt(index), TextColor.color(HSVLike.hsvLike(hue, 0.9f, 1f))));
        }
        return result;
    }

    /**
     * One frame of a highlight sweeping across the text. {@code frame} runs from 0 to {@code frames - 1}.
     * Characters fade from {@code base} to {@code highlight} near the sweep.
     */
    public static Component shimmer(String text, int frame, int frames, TextColor base, TextColor highlight) {
        float travel = text.length() + 2 * SHIMMER_WIDTH;
        float position = frames <= 1 ? 0f : (float) frame / (frames - 1) * travel - SHIMMER_WIDTH;

        Component result = Component.empty();
        for (int index = 0; index < text.length(); index++) {
            float distance = Math.abs(index - position);
            float strength = Math.max(0f, 1f - distance / SHIMMER_WIDTH);
            result = result.append(styled(text.charAt(index), TextColor.lerp(strength, base, highlight)));
        }
        return result;
    }

    /** All frames of {@link #shimmer} for one full sweep. */
    public static List<Component> shimmerFrames(String text, int frames, TextColor base, TextColor highlight) {
        List<Component> result = new ArrayList<>();
        for (int frame = 0; frame < frames; frame++) {
            result.add(shimmer(text, frame, frames, base, highlight));
        }
        return result;
    }

    /**
     * Fades the text between two colours. {@code t} is in [0, 1]; animate it up and down (a triangle wave) for a
     * smooth pulse.
     */
    public static Component pulse(String text, float t, TextColor from, TextColor to) {
        return Component.text(text, TextColor.lerp(Math.max(0f, Math.min(1f, t)), from, to))
                .decoration(TextDecoration.ITALIC, false);
    }

    /** Frames that reveal {@code text} one character at a time, with a cursor until the last character. */
    public static List<Component> typewriterFrames(String text, TextColor color) {
        List<Component> frames = new ArrayList<>();
        for (int length = 0; length <= text.length(); length++) {
            Component frame = Component.text(text.substring(0, length), color);
            if (length < text.length()) {
                frame = frame.append(Component.text(CURSOR, color));
            }
            frames.add(frame.decoration(TextDecoration.ITALIC, false));
        }
        return frames;
    }

    private static Component styled(char character, TextColor color) {
        return Component.text(String.valueOf(character), color).decoration(TextDecoration.ITALIC, false);
    }
}
