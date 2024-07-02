use image::GenericImageView;
use palette::FromColor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[wasm_bindgen]
pub fn extract(image_buffer: Vec<u8>) -> Color {
    let image = image::load_from_memory(&image_buffer).unwrap();

    let (width, height) = image.dimensions();
    let mut r_values = Vec::<f32>::with_capacity((width * height) as usize);
    let mut g_values = Vec::<f32>::with_capacity((width * height) as usize);
    let mut b_values = Vec::<f32>::with_capacity((width * height) as usize);

    let mut weights = 0.0;

    for (_, _, color) in image.pixels() {
        let rgb: palette::Srgb<f32> =
            palette::rgb::Rgb::new(color[0], color[1], color[2]).into_format();
        let hsl = palette::Hsl::from_color(rgb);

        let factor =
            (color[3] as f32 / 255.0) * (1.0 - (hsl.lightness - 0.5).abs() * 2.0) * hsl.saturation;

        r_values.push(rgb.red * factor);
        g_values.push(rgb.green * factor);
        b_values.push(rgb.blue * factor);

        weights += factor;
    }

    if weights == 0.0 {
        return Color { r: 0, g: 0, b: 0 };
    }

    let r = r_values.iter().map(|v| v / weights).sum::<f32>();
    let g = g_values.iter().map(|v| v / weights).sum::<f32>();
    let b = b_values.iter().map(|v| v / weights).sum::<f32>();

    let rgb = palette::Srgb::new(r, g, b);

    let mut hsl = palette::Hsl::from_color(rgb);

    hsl.lightness = if hsl.lightness < 0.5 {
        hsl.lightness * ease_out_quint(hsl.lightness * 2.0)
    } else {
        1.0 - (1.0 - hsl.lightness) * ease_out_quint((1.0 - hsl.lightness) * 2.0)
    };

    hsl.saturation = lerp(hsl.saturation, 1.0, 0.5);

    let rgb = palette::Srgb::from_color(hsl).into_format::<u8>();

    Color {
        r: rgb.red,
        g: rgb.green,
        b: rgb.blue,
    }
}

fn ease_out_quint(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t * t * t + 1.0
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
