

//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;
extern crate rayon;

use num_complex::Complex;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    let max_iterations = 100u16;
    let img_size = 8000u32;
    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;
    let scalex = (cxmax - cxmin) / img_size as f32;
    let scaley = (cymax - cymin) / img_size as f32;
 
    // Create a new ImgBuf
    let mut imgbuf = Arc::new(Mutex::new(image::ImageBuffer::new(img_size, img_size)));
    
    imgbuf.lock().unwrap().pixels();

    //parallel mandel calculation
    let workItems = img_size * img_size;
    (0..workItems).into_par_iter().for_each(|i| {
        
        let x = i % img_size;
        let y = (i / img_size) as u32;


        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;
 
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);
 
        let mut i = 0;
        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }
 
        let quotient = (i as f32)/ (max_iterations as f32);

        let color = quotient * (255 as f32);

        if quotient > 0.5
        {
            let pixel = image::Rgb([color as u8, 255 as u8, color as u8]);
            imgbuf.lock().unwrap().put_pixel(x,y,pixel);
        }
        else
        {
            let pixel = image::Rgb([0 as u8, color as u8,0 as u8]);
            imgbuf.lock().unwrap().put_pixel(x,y,pixel);
        }

        


    });    
 
    // Save image
    imgbuf.lock().unwrap().save("fractal.png").unwrap();
}