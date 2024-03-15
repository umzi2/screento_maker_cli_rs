use std::path::Path;
use image::{ImageBuffer, Luma};
use ndarray::Array2;

fn array2image(array:Array2<f32>)->ImageBuffer<Luma<u8>,Vec<u8>>{
    let(w,h)=(array.shape()[0] as u32,array.shape()[1]as u32);
    ImageBuffer::from_fn(h, w, |x, y| {
        let value = (array[[y as usize, x as usize]]*255.0) as u8;
        Luma([value])
    })
}
pub fn read(path:&Path)->Array2<f32>{
    let img = image::open(path).unwrap().into_luma8();
    let (width, height) = img.dimensions();
    let input = img.iter().map(|&x| x as f32 / 255.0).collect();
    Array2::from_shape_vec((height as usize, width as usize), input).unwrap()
}
pub fn save(array:Array2<f32>,path: &Path){
    let image = array2image(array);
    image.save(path).expect("Failed to save image.");

}
