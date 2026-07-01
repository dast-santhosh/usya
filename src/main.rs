#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use bootloader::boot_info::PixelFormat;
use core::panic::PanicInfo;
use tinybmp::Bmp;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        
        let width = info.horizontal_resolution;
        let height = info.vertical_resolution;
        let bytes_per_pixel = info.bytes_per_pixel;
        let stride = info.stride;

        // Clear the screen to pure white
        for y in 0..height {
            for x in 0..width {
                let offset = (y * stride + x) * bytes_per_pixel;
                buffer[offset] = 255;
                if bytes_per_pixel > 1 {
                    buffer[offset + 1] = 255;
                }
                if bytes_per_pixel > 2 {
                    buffer[offset + 2] = 255;
                }
            }
        }

        // Draw logo
        let logo_bytes = include_bytes!("assets/logo.bmp");
        if let Ok(bmp) = Bmp::<Rgb888>::from_slice(logo_bytes) {
            let logo_width = bmp.bounding_box().size.width as usize;
            let logo_height = bmp.bounding_box().size.height as usize;
            
            let start_x = if width > logo_width { (width - logo_width) / 2 } else { 0 };
            let start_y = if height > logo_height { (height - logo_height) / 2 } else { 0 };

            for Pixel(point, color) in bmp.pixels() {
                let x = start_x + point.x as usize;
                let y = start_y + point.y as usize;

                if x < width && y < height {
                    let offset = (y * stride + x) * bytes_per_pixel;
                    
                    match info.pixel_format {
                        PixelFormat::RGB => {
                            buffer[offset] = color.r();
                            if bytes_per_pixel > 1 { buffer[offset + 1] = color.g(); }
                            if bytes_per_pixel > 2 { buffer[offset + 2] = color.b(); }
                        }
                        PixelFormat::BGR => {
                            buffer[offset] = color.b();
                            if bytes_per_pixel > 1 { buffer[offset + 1] = color.g(); }
                            if bytes_per_pixel > 2 { buffer[offset + 2] = color.r(); }
                        }
                        PixelFormat::U8 => {
                            let grey = (color.r() as u16 + color.g() as u16 + color.b() as u16) / 3;
                            buffer[offset] = grey as u8;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
