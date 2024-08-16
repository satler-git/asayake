use anyhow::{bail, Context as _, Ok, Result};
use fxhash::FxHashMap;
use image::{DynamicImage, Pixel, Rgba};

/// pngをbase64_pngに変換する
/// `format!("data:image/png;base64,{}", resp_base64)`
pub fn convert_img_base64(image_from: &DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    image_from
        .write_to(
            &mut std::io::Cursor::new(&mut image_data),
            image::ImageFormat::Png,
        )
        .unwrap();
    let resp_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_data);
    format!("data:image/png;base64,{}", resp_base64)
}

fn cast_rgbau8_to_string(rgba: &Rgba<u8>) -> String {
    let rgb = rgba.to_rgb();
    format!("{:02X}{:02X}{:02X}", rgb.0[0], rgb.0[1],rgb.0[2])
}

/// 一番多く使われている色を`u32`で返す
/// 計算量が多い(アイコンはそこまでサイズがデカくないから大丈夫かも)
pub fn find_most_used_color(image: &DynamicImage) -> Result<String> {
    if let DynamicImage::ImageRgba8(img) = image {
        // 各ピクセルをイテレートして色情報を取得
        let pixels = img.pixels();
        let mut counts: FxHashMap<String, u32> = FxHashMap::default();

        for pi in pixels {
            let count = counts.entry(cast_rgbau8_to_string(pi)).or_insert(0);
            *count += 1;
        }

        let most_common_color = counts
            .iter()
            .max_by_key(|i| i.1)
            .context("Unable to find the most common color")?;

        Ok(most_common_color.0.clone())
    } else {
        bail!("Unable to cast down to a Rgba image.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use image::{DynamicImage, Rgb};

    fn make_one_colored_image(rgb: &Rgb<u8>) -> Result<DynamicImage> {
        let width = 54;
        let height = 54;

        let image: DynamicImage = image::ImageBuffer::from_pixel(width, height, *rgb).into();

        Ok(image.to_rgba8().into())
    }

    #[test]
    fn test_cast_rgbau8_to_string() {
        assert_eq!("FFFFFF", cast_rgbau8_to_string(&Rgba([0xFF, 0xFF, 0xFF, 0xFF])));
        assert_eq!("000000", cast_rgbau8_to_string(&Rgba([0x0, 0x0, 0x0, 0xFF])));
        assert_eq!("010203", cast_rgbau8_to_string(&Rgba([0x1, 0x2, 0x3, 0xFF])));
    }

    fn check_one_colored_image(rgb: Rgb<u8>) -> Result<()> {
        let image = &make_one_colored_image(&rgb)?;
        let rgb_u32 = cast_rgbau8_to_string(&Rgba([rgb.0[0], rgb.0[1], rgb.0[2], 0xFF]));
        assert_eq!(find_most_used_color(image)?, rgb_u32);
        Ok(())
    }

    #[test]
    fn one_colored_images() -> Result<()> {
        check_one_colored_image(Rgb([255, 255, 255]))?;
        check_one_colored_image(Rgb([0, 0, 0]))?;
        Ok(())
    }
}
