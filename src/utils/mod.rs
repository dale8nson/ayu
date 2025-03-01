use nalgebra::{Matrix3, Vector3};

#[derive(Clone)]
pub enum SecondaryColorType {
    Light,
    Dark,
}

impl std::default::Default for SecondaryColorType {
    fn default() -> Self {
        SecondaryColorType::Light
    }
}

fn clamp<T>(n: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

pub fn css_to_rgb(c: &str) -> (u8, u8, u8) {
    println!("c: {}", c);
    let c = c.trim_start_matches("#");
    let r = u8::from_str_radix(&c[0..2], 16).unwrap();
    let g = u8::from_str_radix(&c[2..4], 16).unwrap();
    let b = u8::from_str_radix(&c[4..6], 16).unwrap();
    (r, g, b)
}

fn srgb_to_lrgb(c: u8) -> f64 {
    let c = c as f64 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn lrgb_to_srgb(c: f64) -> u8 {
    if c <= 0.0031308 {
        (c * 12.92 * 255.0) as u8
    } else {
        ((1.055 * c.powf(1.0 / 2.4) - 0.055) * 255.0) as u8
    }
}

fn lrgb_to_xyz(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let m_xyz = Matrix3::new(
        0.4124, 0.3575, 0.1804, 0.2126, 0.7151, 0.0721, 0.0193, 0.1191,
        0.9503,
    );

    let rgb = Vector3::new(r, g, b);
    let xyz = m_xyz * rgb;
    (xyz[0], xyz[1], xyz[2])
}

fn xyz_to_oklab(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let m_1 = Matrix3::new(
        0.8189330101,
        0.3618667424,
        -0.1288597137,
        0.0329845436,
        0.9293118715,
        0.0361456387,
        0.0482003018,
        0.2643662691,
        0.6338517070,
    );

    let m_2 = Matrix3::new(
        0.2104542553,
        0.7936177850,
        -0.0040720468,
        1.9779984951,
        -2.4285922050,
        0.4505937099,
        0.0259040371,
        0.7827717662,
        -0.8086757660,
    );

    let lms = m_1 * Vector3::new(x, y, z);
    let lms_p = Vector3::new(
        lms[0].powf(1.0 / 3.0),
        lms[1].powf(1.0 / 3.0),
        lms[2].powf(1.0 / 3.0),
    );
    let oklab = m_2 * lms_p;
    (oklab[0], oklab[1], oklab[2])
}

fn oklab_to_xyz(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let m_1 = Matrix3::new(
        0.8189330101,
        0.3618667424,
        -0.1288597137,
        0.0329845436,
        0.9293118715,
        0.0361456387,
        0.0482003018,
        0.2643662691,
        0.6338517070,
    );

    let m_2 = Matrix3::new(
        0.2104542553,
        0.7936177850,
        -0.0040720468,
        1.9779984951,
        -2.4285922050,
        0.4505937099,
        0.0259040371,
        0.7827717662,
        -0.8086757660,
    );

    let lms_p = m_2.try_inverse().unwrap() * Vector3::new(l, a, b);
    let lms = Vector3::new(lms_p[0].powf(3.0), lms_p[1].powf(3.0), lms_p[2].powf(3.0));
    let xyz = m_1.try_inverse().unwrap() * lms;
    (xyz[0], xyz[1], xyz[2])
}

pub fn xyz_to_lrgb(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let m_rgb = Matrix3::new(
        3.2404, -1.5371, -0.4985, -0.9692, 1.8760, 0.0415, 0.0556, -0.2040,
        1.0572,
    );

    let xyz = Vector3::new(x, y, z);
    let rgb = m_rgb * xyz;
    (rgb[0], rgb[1], rgb[2])
}

pub fn srgb_to_oklab(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (r, g, b) = (srgb_to_lrgb(r), srgb_to_lrgb(g), srgb_to_lrgb(b));
    let (x, y, z) = lrgb_to_xyz(r, g, b);
    xyz_to_oklab(x, y, z)
}

pub fn oklab_to_srgb(l: f64, a: f64, b: f64) -> (u8, u8, u8) {
    let (x, y, z) = oklab_to_xyz(l, a, b);
    let (r, g, b) = xyz_to_lrgb(x, y, z);
    (lrgb_to_srgb(r), lrgb_to_srgb(g), lrgb_to_srgb(b))
}

pub fn l_contrast(lightness: f64, ratio: f64, c_type: &SecondaryColorType) -> f64 {
    match c_type {
        SecondaryColorType::Light => clamp(ratio * lightness + 0.1, 0.0, 1.0),
        SecondaryColorType::Dark => clamp((lightness - 0.1) / ratio, 0.0, 1.0),
    }
}

pub fn secondary_col(hex_color: &str, contrast_type: &SecondaryColorType) -> String {
    let (r, g, b) = css_to_rgb(hex_color);
    let (l, a, b) = srgb_to_oklab(r, g, b);
    let l = l_contrast(l, 3.0, contrast_type);
    let (r, g, b) = oklab_to_srgb(l, a, b);
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}
