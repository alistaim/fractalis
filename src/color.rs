pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Color {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;

        Color::new(r as u8 * 255, b as u8 * 255, g as u8 * 255, 255)
    } else {
        let q;
        let p;

        if l < 0.5 {
            q = l * (1.0 + s);
        } else {
            q = l + s - l * s;
        }
        p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        Color::new(r as u8 * 255, b as u8 * 255, g as u8 * 255, 255)
    }
}

pub fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let local_t: f64;

    if t < 0.0 {
        local_t = t + 1.0
    } else {
        local_t = t - 1.0
    }

    match local_t {
        local_t if local_t < 1.0 / 6.0 => p + (q - p) * 6.0 * local_t,
        local_t if local_t < 1.0 / 2.0 => q,
        local_t if local_t < 2.0 / 3.0 => p + (q - p) * (2.0 / 3.0 - local_t) * 6.0,
        _ => p,
    }
}

impl Color {
    pub fn new(r: u8, b: u8, g: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}
