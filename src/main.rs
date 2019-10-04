extern crate image;
extern crate num_complex;
extern crate num_cpus;
extern crate rayon;

use num_complex::Complex;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

static max_iterations: u16 = 100u16;
static img_size: u32 =  8000u32;
static cxmin: f32 = -2f32;
static cxmax: f32 = 1f32;
static cymin: f32 = -1.5f32;
static cymax: f32 = 1.5f32;
static scalex: f32 = (cxmax - cxmin) / img_size as f32;
static scaley: f32 = (cymax - cymin) / img_size as f32;


fn mandel_loop(mut imgbuf: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>>, c: u32, work_items: u32, num_cpus: usize)
    //where F: image::GenericImageView + image::GenericImage  //+ image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>>
{
    (c..(work_items)).step_by(num_cpus).for_each(|i| 
            
        {
            
            let x = (i as u32) % img_size;
            let y = ((i as u32) / img_size) as u32;


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
                let pixel = image::Rgb([0 as u8, color as u8,0 as u8]);
                imgbuf.put_pixel(x as u32,y as u32, pixel);
            }
            else
            {
                let pixel = image::Rgb([0 as u8, color as u8,0 as u8]);
                imgbuf.put_pixel(x as u32,y as u32, pixel);
            }

            
        });    
}

fn main() {
    
 
    // Create a new ImgBuf
    let mut imgbuf = image::ImageBuffer::new(img_size, img_size);

    //parallel mandel calculation
    let work_items = img_size * img_size;

    let num_cpus = num_cpus::get();

    (0..num_cpus).into_par_iter().for_each(|c|{
        mandel_loop(imgbuf, c as u32, work_items, num_cpus);
    });


    
 
    // Save image
    imgbuf.save("fractal.png").unwrap();
}
