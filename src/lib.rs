use base64::{
    encode,
    decode
};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    img = img.grayscale();
    log(&"greyscale effect applied".into());

    let mut buffer = vec![] ; //Vec::new();
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );
    data_url
    
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
