use image::DynamicImage;

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
  base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_data)
}

/// 一番多く使われている色を`u8`で返す
pub fn find_most_used_color(image: &DynamicImage) -> u8 {
  todo!()
}