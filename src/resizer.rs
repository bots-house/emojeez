pub fn resize_png(to: (u32, u32), src: &[u8]) -> Option<Vec<u8>> {
    image::load_from_memory_with_format(src, image::ImageFormat::Png).map_or(None, |img| {
        let mut dest = vec![];
        let img = img.resize(to.0, to.1, image::imageops::Lanczos3);
        img.write_to(&mut dest, image::ImageFormat::Png)
            .map_or(None, |_| Some(dest))
    })
}
