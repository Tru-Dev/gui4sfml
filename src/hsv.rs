
use sfml::graphics::Color;

pub fn hsv(hue: i16, sat: u8, val: u8) -> Color {
    let hue = hue.rem_euclid(360);
    let c = (val as f64 / 100f64) * (sat as f64 / 100f64);
    let x = c * (1f64 - ((hue as f64 / 60f64) % 2f64 - 1f64).abs());
    let m = val as f64 / 100f64 - c;

    let (r, g, b) = if hue < 60 {
        (c, x, 0f64)
    } else if 60 <= hue && hue < 120 {
        (x, c, 0f64)
    } else if 120 <= hue && hue < 180 {
        (0f64, c, x)
    } else if 180 <= hue && hue < 240 {
        (0f64, x, c)
    } else if 240 <= hue && hue < 300 {
        (x, 0f64, c)
    } else if 300 <= hue {
        (c, 0f64, x)
    } else {
        (0f64, 0f64, 0f64)
    };

    Color::rgb(
        ((r + m) * 255f64) as u8,
        ((g + m) * 255f64) as u8,
        ((b + m) * 255f64) as u8
    )
}

pub fn to_hsv(rgb: Color) -> (i16, u8, u8) {
    let r = rgb.r as f64 / 255f64;
    let g = rgb.g as f64 / 255f64;
    let b = rgb.b as f64 / 255f64;
    
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let delta = c_max - c_min;

    let hue = (60f64 * if delta == 0f64 {
        0f64
    } else if c_max == r {
        ((g - b) / delta).rem_euclid(6f64)
    } else if c_max == g {
        (b - r) / delta + 2f64
    } else if c_max == b {
        (r - g) / delta + 4f64
    } else {
        0f64
    }) as i16;

    let sat = (100f64 * if c_max == 0f64 {
        0f64
    } else {
        delta / c_max
    }) as u8;

    let val = (c_max * 100f64) as u8;

    (hue, sat, val)
}
