//! Glyph layout of the generated `craftcn:gui` font.
//!
//! The Java side (`texture-gui` component, `GuiFont.java`) uses the same code points.
//! Change them in both places, or the tests in `templates/` and `GuiFont` will disagree.

use serde_json::{json, Map, Value};

pub const NAMESPACE: &str = "craftcn";
pub const FONT_FILE: &str = "gui";

/// `SHIFT_POSITIVE_BASE + bit` advances the text by `1 << bit` pixels to the right.
pub const SHIFT_POSITIVE_BASE: u32 = 0xE001;
/// `SHIFT_NEGATIVE_BASE + bit` moves the text by `1 << bit` pixels to the left.
pub const SHIFT_NEGATIVE_BASE: u32 = 0xE011;
/// Eight bits cover shifts of up to 255 pixels in either direction.
pub const SHIFT_BITS: u32 = 8;

/// A bitmap glyph. `texture` is relative to `assets/craftcn/textures/`.
pub struct GlyphSpec {
    pub texture: &'static str,
    pub code: u32,
    pub width: u32,
    pub height: u32,
    pub ascent: i32,
}

/// Title plates drawn as font glyphs. Their widths are part of the Java API.
pub const GLYPHS: [GlyphSpec; 4] = [
    GlyphSpec {
        texture: "gui/plate_primary",
        code: 0xE100,
        width: 176,
        height: 12,
        ascent: 9,
    },
    GlyphSpec {
        texture: "gui/plate_accent",
        code: 0xE101,
        width: 176,
        height: 12,
        ascent: 9,
    },
    GlyphSpec {
        texture: "gui/plate_danger",
        code: 0xE102,
        width: 176,
        height: 12,
        ascent: 9,
    },
    GlyphSpec {
        texture: "gui/plate_compact",
        code: 0xE103,
        width: 96,
        height: 12,
        ascent: 9,
    },
];

/// Item icons, exposed as item models `craftcn:<name>`.
pub const ICONS: [&str; 7] = [
    "icon_next",
    "icon_back",
    "icon_confirm",
    "icon_cancel",
    "icon_info",
    "icon_close",
    "icon_filler",
];

fn char_for(code: u32) -> char {
    char::from_u32(code).expect("font code points are valid BMP scalars")
}

/// Every shift glyph with the horizontal advance it applies.
pub fn shift_advances() -> Vec<(char, i32)> {
    (0..SHIFT_BITS)
        .flat_map(|bit| {
            let step = 1i32 << bit;
            [
                (char_for(SHIFT_POSITIVE_BASE + bit), step),
                (char_for(SHIFT_NEGATIVE_BASE + bit), -step),
            ]
        })
        .collect()
}

/// Builds `assets/craftcn/font/gui.json`.
pub fn font_json() -> Value {
    let mut advances = Map::new();
    for (ch, advance) in shift_advances() {
        advances.insert(ch.to_string(), json!(advance));
    }

    let mut providers = vec![json!({ "type": "space", "advances": advances })];

    for glyph in &GLYPHS {
        providers.push(json!({
            "type": "bitmap",
            "file": format!("{NAMESPACE}:{}.png", glyph.texture),
            "ascent": glyph.ascent,
            "height": glyph.height,
            "chars": [char_for(glyph.code).to_string()],
        }));
    }

    json!({ "providers": providers })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn shift_glyphs_are_unique_and_cover_both_directions() {
        let shifts = shift_advances();
        let codes: HashSet<char> = shifts.iter().map(|(c, _)| *c).collect();
        assert_eq!(codes.len(), shifts.len());

        assert!(shifts.contains(&(char_for(SHIFT_POSITIVE_BASE), 1)));
        assert!(shifts.contains(&(char_for(SHIFT_NEGATIVE_BASE + 7), -128)));
    }

    #[test]
    fn glyph_code_points_do_not_collide_with_shifts() {
        let shift_codes: HashSet<u32> = shift_advances().iter().map(|(c, _)| *c as u32).collect();
        for glyph in &GLYPHS {
            assert!(
                !shift_codes.contains(&glyph.code),
                "{:X} collides",
                glyph.code
            );
            assert!(glyph.ascent <= glyph.height as i32);
        }
    }

    #[test]
    fn font_json_has_space_and_bitmap_providers() {
        let font = font_json();
        let providers = font["providers"].as_array().unwrap();

        assert_eq!(providers[0]["type"], "space");
        assert_eq!(providers.len(), 1 + GLYPHS.len());
        assert_eq!(providers[1]["file"], "craftcn:gui/plate_primary.png");
        assert_eq!(providers[1]["chars"][0], "\u{E100}");
    }
}
