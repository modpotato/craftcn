//! Procedural pixel art for the generated resource pack, drawn in the theme's palette.

use image::{Rgba, RgbaImage};

pub type Color = Rgba<u8>;

pub const TRANSPARENT: Color = Rgba([0, 0, 0, 0]);
const WHITE: Color = Rgba([255, 255, 255, 255]);
const BLACK: Color = Rgba([0, 0, 0, 255]);

/// Parses `#RRGGBB`, falling back to `fallback` for anything else.
pub fn parse_hex(hex: &str, fallback: Color) -> Color {
    let digits = hex.trim().trim_start_matches('#');
    if digits.len() != 6 {
        return fallback;
    }
    match u32::from_str_radix(digits, 16) {
        Ok(value) => Rgba([
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
            255,
        ]),
        Err(_) => fallback,
    }
}

pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Rgba([
        lerp(a[0], b[0]),
        lerp(a[1], b[1]),
        lerp(a[2], b[2]),
        lerp(a[3], b[3]),
    ])
}

pub fn lighten(color: Color, t: f32) -> Color {
    mix(color, WHITE, t)
}

pub fn darken(color: Color, t: f32) -> Color {
    mix(color, BLACK, t)
}

fn put(img: &mut RgbaImage, x: i32, y: i32, color: Color) {
    if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
        img.put_pixel(x as u32, y as u32, color);
    }
}

fn fill_polygon(img: &mut RgbaImage, points: &[(f32, f32)], color: Color) {
    let min_x = points.iter().map(|p| p.0).fold(f32::MAX, f32::min).floor() as i32;
    let max_x = points.iter().map(|p| p.0).fold(f32::MIN, f32::max).ceil() as i32;
    let min_y = points.iter().map(|p| p.1).fold(f32::MAX, f32::min).floor() as i32;
    let max_y = points.iter().map(|p| p.1).fold(f32::MIN, f32::max).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if point_in_polygon(x as f32 + 0.5, y as f32 + 0.5, points) {
                put(img, x, y, color);
            }
        }
    }
}

fn point_in_polygon(px: f32, py: f32, points: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let mut j = points.len() - 1;
    for i in 0..points.len() {
        let (xi, yi) = points[i];
        let (xj, yj) = points[j];
        if (yi > py) != (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn stroke(img: &mut RgbaImage, a: (f32, f32), b: (f32, f32), width: f32, color: Color) {
    let radius = width / 2.0;
    let min_x = (a.0.min(b.0) - radius).floor() as i32;
    let max_x = (a.0.max(b.0) + radius).ceil() as i32;
    let min_y = (a.1.min(b.1) - radius).floor() as i32;
    let max_y = (a.1.max(b.1) + radius).ceil() as i32;

    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length_sq = (dx * dx + dy * dy).max(f32::EPSILON);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let t = (((px - a.0) * dx + (py - a.1) * dy) / length_sq).clamp(0.0, 1.0);
            let (cx, cy) = (a.0 + dx * t, a.1 + dy * t);
            if (px - cx).hypot(py - cy) <= radius {
                put(img, x, y, color);
            }
        }
    }
}

fn disk(img: &mut RgbaImage, center: (f32, f32), radius: f32, color: Color) {
    let min_x = (center.0 - radius).floor() as i32;
    let max_x = (center.0 + radius).ceil() as i32;
    let min_y = (center.1 - radius).floor() as i32;
    let max_y = (center.1 + radius).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            if (px - center.0).hypot(py - center.1) <= radius {
                put(img, x, y, color);
            }
        }
    }
}

fn diamond(img: &mut RgbaImage, center: (f32, f32), radius: f32, color: Color) {
    let (cx, cy) = center;
    fill_polygon(
        img,
        &[
            (cx, cy - radius),
            (cx + radius, cy),
            (cx, cy + radius),
            (cx - radius, cy),
        ],
        color,
    );
}

/// A horizontal title banner: gradient body, accent border, bevel highlight and diamond ornaments.
pub fn plate(width: u32, height: u32, base: Color, accent: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(width, height, TRANSPARENT);
    let (w, h) = (width as i32, height as i32);

    for y in 0..h {
        let t = y as f32 / (h - 1).max(1) as f32;
        let fill = mix(lighten(base, 0.18), darken(base, 0.32), t);
        for x in 0..w {
            put(&mut img, x, y, fill);
        }
    }

    for x in 0..w {
        put(&mut img, x, 0, accent);
        put(&mut img, x, h - 1, accent);
        put(&mut img, x, 1, lighten(base, 0.45));
        put(&mut img, x, h - 2, darken(base, 0.5));
    }
    for y in 0..h {
        put(&mut img, 0, y, accent);
        put(&mut img, w - 1, y, accent);
    }

    // Round the outer corners.
    for (x, y) in [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)] {
        put(&mut img, x, y, TRANSPARENT);
    }

    let mid = h as f32 / 2.0;
    diamond(&mut img, (5.0, mid), 2.5, accent);
    diamond(&mut img, (w as f32 - 6.0, mid), 2.5, accent);

    img
}

/// A 16x16 beveled tile for filling empty inventory slots.
pub fn filler(size: u32, base: Color, accent: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(size, size, base);
    let last = size as i32 - 1;

    for i in 0..size as i32 {
        put(&mut img, i, 0, lighten(base, 0.25));
        put(&mut img, 0, i, lighten(base, 0.25));
        put(&mut img, i, last, darken(base, 0.35));
        put(&mut img, last, i, darken(base, 0.35));
    }

    let dot = mix(base, accent, 0.5);
    for y in (2..last - 1).step_by(4) {
        for x in (2..last - 1).step_by(4) {
            put(&mut img, x, y, dot);
            put(&mut img, x + 1, y, dot);
            put(&mut img, x, y + 1, dot);
            put(&mut img, x + 1, y + 1, dot);
        }
    }

    img
}

/// Double-stroke chevron pointing right; mirrored for "back".
pub fn chevron(mirror: bool, color: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(16, 16, TRANSPARENT);
    let x = |v: f32| if mirror { 16.0 - v } else { v };
    fill_polygon(
        &mut img,
        &[(x(4.0), 3.0), (x(12.5), 8.0), (x(4.0), 13.0)],
        color,
    );
    img
}

pub fn check(color: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(16, 16, TRANSPARENT);
    stroke(&mut img, (3.0, 8.0), (6.5, 11.5), 2.2, color);
    stroke(&mut img, (6.5, 11.5), (13.0, 4.5), 2.2, color);
    img
}

pub fn cross(color: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(16, 16, TRANSPARENT);
    stroke(&mut img, (4.0, 4.0), (12.0, 12.0), 2.2, color);
    stroke(&mut img, (12.0, 4.0), (4.0, 12.0), 2.2, color);
    img
}

pub fn info(color: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(16, 16, TRANSPARENT);
    disk(&mut img, (8.0, 8.0), 6.8, color);
    disk(&mut img, (8.0, 4.9), 1.1, WHITE);
    stroke(&mut img, (8.0, 7.2), (8.0, 11.6), 2.0, WHITE);
    img
}

/// A filled disc with a white X, distinct from the bare cross used for "cancel".
pub fn close(color: Color) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(16, 16, TRANSPARENT);
    disk(&mut img, (8.0, 8.0), 6.8, color);
    stroke(&mut img, (5.2, 5.2), (10.8, 10.8), 1.8, WHITE);
    stroke(&mut img, (10.8, 5.2), (5.2, 10.8), 1.8, WHITE);
    img
}

/// The 128x128 pack thumbnail shown in the server resource pack list.
pub fn pack_icon(base: Color, accent: Color, foreground: Color) -> RgbaImage {
    let size = 128u32;
    let mut img = RgbaImage::from_pixel(size, size, base);

    for y in 0..size {
        for x in 0..size {
            let t = (x + y) as f32 / (2 * size) as f32;
            let mut color = mix(lighten(base, 0.12), darken(base, 0.4), t);
            if (x + y) % 24 < 6 {
                color = mix(color, accent, 0.18);
            }
            put(&mut img, x as i32, y as i32, color);
        }
    }

    for i in 0..4 {
        for k in 0..size as i32 {
            put(&mut img, k, i, accent);
            put(&mut img, k, size as i32 - 1 - i, accent);
            put(&mut img, i, k, accent);
            put(&mut img, size as i32 - 1 - i, k, accent);
        }
    }

    let c = size as f32 / 2.0;
    diamond(&mut img, (c, c), 26.0, foreground);
    diamond(&mut img, (c, c), 18.0, darken(accent, 0.2));
    diamond(&mut img, (c, c), 8.0, accent);

    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_colors_with_fallback() {
        let fallback = Rgba([1, 2, 3, 255]);
        assert_eq!(parse_hex("#FFD700", fallback), Rgba([255, 215, 0, 255]));
        assert_eq!(parse_hex("nope", fallback), fallback);
    }

    #[test]
    fn plate_has_requested_size_and_transparent_corners() {
        let img = plate(176, 12, Rgba([200, 100, 0, 255]), Rgba([0, 200, 200, 255]));
        assert_eq!(img.dimensions(), (176, 12));
        assert_eq!(img.get_pixel(0, 0)[3], 0);
        assert_eq!(img.get_pixel(88, 6)[3], 255);
    }

    #[test]
    fn icons_are_16_pixels_with_transparent_background() {
        for img in [
            chevron(false, WHITE),
            check(WHITE),
            cross(WHITE),
            info(WHITE),
            close(WHITE),
        ] {
            assert_eq!(img.dimensions(), (16, 16));
            assert_eq!(img.get_pixel(0, 0)[3], 0);
        }
    }
}
