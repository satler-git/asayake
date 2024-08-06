use anyhow::{bail, Context as _, Ok, Result};
use fxhash::FxHashMap;
use image::{DynamicImage, Rgba};

/// pngをbase64_pngに変換する
/// `format!("data:image/png;base64,{}", resp_base64)`
pub fn convert_img_base64(image_from: &DynamicImage) -> Result<String> {
    let mut image_data: Vec<u8> = Vec::new();
    image_from
        .write_to(
            &mut std::io::Cursor::new(&mut image_data),
            image::ImageFormat::Png,
        ).context("Unable to convert an icon to base64 string")?;
    Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_data))
}

/// 一番多く使われている色を`u32`で返す
/// 計算量が多い(アイコンはそこまでサイズがデカくないから大丈夫かも)
pub fn find_most_used_color(image: &DynamicImage) -> Result<Rgba<u8>> {
    if let DynamicImage::ImageRgba8(img) = image {
        // 各ピクセルをイテレートして色情報を取得
        let pixels = img.pixels();
        let mut counts: FxHashMap<&Rgba<u8>, u32> = FxHashMap::default();

        for pi in pixels {
            let count = counts.entry(pi).or_insert(0);
            *count += 1;
        }

        let most_common_color = counts
            .iter()
            .filter(|&(&x, _)| match x {
                Rgba([_, _, _, 0]) => false,
                _ => true
            })
            .max_by_key(|i| i.1)
            .context("Unable to find the most common color")?.0;

        Ok(**most_common_color)
    } else {
        bail!("Unable to cast down to a Rgba image.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use image::{DynamicImage, Rgba};

    fn make_one_colored_image(rgba: &Rgba<u8>) -> Result<DynamicImage> {
        let width = 54;
        let height = 54;

        let image: DynamicImage = image::ImageBuffer::from_pixel(width, height, *rgba).into();

        Ok(image.to_rgba8().into())
    }

    fn check_one_colored_image(rgba: Rgba<u8>) -> Result<()> {
        let image = &make_one_colored_image(&rgba)?;
        assert_eq!(find_most_used_color(image)?, rgba);
        Ok(())
    }

    #[test]
    fn one_colored_images() -> Result<()> {
        check_one_colored_image(Rgba([255, 255, 255, 255]))?;
        check_one_colored_image(Rgba([10, 10, 10, 255]))?;
        Ok(())
    }
}
