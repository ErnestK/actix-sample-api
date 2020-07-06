pub const URL_TO_SAVE: &str = "store";
const URL_TO_PREVIEW: &str = "preview";

pub fn create_preview(filename: String) {
    let width = 100;
    let height = 100;
    let img = image::open(format!("{}/{}", URL_TO_SAVE, filename)).unwrap();

    let resized = img.resize(width, height, image::imageops::Nearest);
    &resized.save(format!("{}/{}", URL_TO_PREVIEW, filename));
}