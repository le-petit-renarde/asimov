//! Mascot Image Generator
//!
//! Generates PNG images for the Professor mascot programmatically.
//! Run with: cargo run --example generate_mascot

use image::{ImageBuffer, Rgba, RgbaImage, GenericImageView};
use std::path::Path;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

/// Color palette for the Professor mascot
mod colors {
    pub const BACKGROUND: Rgba<u8> = Rgba([26, 26, 46, 255]);       // Dark blue-purple
    pub const SKIN: Rgba<u8> = Rgba([245, 230, 211, 255]);          // Warm peach
    pub const GLASSES_FRAME: Rgba<u8> = Rgba([139, 115, 85, 255]);  // Antique gold
    pub const GLASSES_LENS: Rgba<u8> = Rgba([200, 200, 220, 100]);  // Translucent blue
    pub const SHIRT: Rgba<u8> = Rgba([245, 245, 245, 255]);         // Crisp white
    pub const VEST: Rgba<u8> = Rgba([139, 69, 19, 255]);           // Warm brown
    pub const PIPE: Rgba<u8> = Rgba([222, 184, 135, 255]);          // Burlywood
    pub const SMOKE: Rgba<u8> = Rgba([192, 192, 192, 180]);        // Soft gray (semi-transparent)
    pub const BOOK: Rgba<u8> = Rgba([101, 67, 33, 255]);            // Dark leather
    pub const HAIR: Rgba<u8> = Rgba([100, 100, 100, 255]);          // Gray hair
    pub const MUSTACHE: Rgba<u8> = Rgba([80, 60, 50, 255]);        // Dark gray-brown
    pub const ACCENT: Rgba<u8> = Rgba([255, 215, 0, 255]);         // Gold accent
}

fn main() {
    let out_dir = Path::new("public");
    std::fs::create_dir_all(out_dir).expect("Failed to create public directory");
    
    // Generate Professor idle
    let img = generate_professor("idle");
    img.save(out_dir.join("Professor.png")).expect("Failed to save Professor.png");
    println!("✓ Generated public/Professor.png");
    
    // Generate Professor thinking
    let img = generate_professor("thinking");
    img.save(out_dir.join("Professor-Think.png")).expect("Failed to save Professor-Think.png");
    println!("✓ Generated public/Professor-Think.png");
    
    // Generate Professor searching
    let img = generate_professor("searching");
    img.save(out_dir.join("Professor-Search.png")).expect("Failed to save Professor-Search.png");
    println!("✓ Generated public/Professor-Search.png");
    
    // Generate sprite sheet
    let sprite = generate_sprite_sheet();
    sprite.save(out_dir.join("Professor-Sprite.png")).expect("Failed to save Professor-Sprite.png");
    println!("✓ Generated public/Professor-Sprite.png");
    
    // Generate simple logo variant
    let logo = generate_simple_logo();
    logo.save(out_dir.join("Asimov-Logo.png")).expect("Failed to save Asimov-Logo.png");
    println!("✓ Generated public/Asimov-Logo.png");
    
    println!("\n🎉 All mascot images generated successfully!");
}

fn generate_professor(pose: &str) -> RgbaImage {
    let mut img: RgbaImage = ImageBuffer::from_pixel(WIDTH, HEIGHT, colors::BACKGROUND);
    
    let cx = WIDTH as f32 / 2.0;
    let cy = HEIGHT as f32 / 2.0;
    
    // Draw body (centered)
    draw_body(&mut img, cx, cy + 50.0, pose);
    
    // Draw head
    draw_head(&mut img, cx, cy - 80.0, pose);
    
    // Draw glasses
    draw_glasses(&mut img, cx, cy - 90.0, pose);
    
    // Draw mustache
    draw_mustache(&mut img, cx, cy - 55.0);
    
    // Draw pipe (if not searching)
    if pose != "searching" {
        draw_pipe(&mut img, cx + 60.0, cy - 20.0, pose);
    }
    
    // Draw book
    draw_book(&mut img, cx - 80.0, cy + 100.0);
    
    // Add smoke particles
    draw_smoke(&mut img, cx + 70.0, cy - 50.0, pose);
    
    img
}

fn draw_body(img: &mut RgbaImage, cx: f32, cy: f32, pose: &str) {
    // White collar
    for y in 0..80u32 {
        for x in 0..120u32 {
            let px = cx as i32 - 60 + x as i32;
            let py = cy as i32 - 40 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dist = ((x as f32 - 60.0).powi(2) + (y as f32 - 40.0).powi(2)).sqrt();
                if dist < 60.0 {
                    img.put_pixel(px as u32, py as u32, colors::SHIRT);
                }
            }
        }
    }
    
    // Brown vest
    for y in 40..120u32 {
        for x in 20..100u32 {
            let px = cx as i32 - 50 + x as i32;
            let py = cy as i32 - 40 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dist = ((x as f32 - 50.0).powi(2) + (y as f32 - 40.0).powi(2)).sqrt();
                if dist < 60.0 && y > 45 && x < 35 || x > 65 {
                    img.put_pixel(px as u32, py as u32, colors::VEST);
                }
            }
        }
    }
}

fn draw_head(img: &mut RgbaImage, cx: f32, cy: f32, _pose: &str) {
    // Face (oval)
    for y in 0..100u32 {
        for x in 0..80u32 {
            let px = cx as i32 - 40 + x as i32;
            let py = cy as i32 - 50 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 40.0) / 35.0;
                let dy = (y as f32 - 50.0) / 45.0;
                if dx * dx + dy * dy < 1.0 {
                    img.put_pixel(px as u32, py as u32, colors::SKIN);
                }
            }
        }
    }
    
    // Gray hair (top)
    for y in 0..30u32 {
        for x in 0..80u32 {
            let px = cx as i32 - 40 + x as i32;
            let py = cy as i32 - 80 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 40.0) / 40.0;
                let dy = (y as f32 - 15.0) / 15.0;
                if dy < 1.0 && dx * dx + (dy - 0.5).powi(2) < 0.5 {
                    img.put_pixel(px as u32, py as u32, colors::HAIR);
                }
            }
        }
    }
}

fn draw_glasses(img: &mut RgbaImage, cx: f32, cy: f32, pose: &str) {
    // Left lens
    for y in 0..25u32 {
        for x in 0..30u32 {
            let px = cx as i32 - 35 + x as i32;
            let py = cy as i32 - 12 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 15.0) / 14.0;
                let dy = (y as f32 - 12.0) / 11.0;
                if dx * dx + dy * dy < 1.0 {
                    // Eye inside lens
                    let eye_x = if pose == "searching" { 0.0 } else { -2.0 };
                    let eye_y = if pose == "searching" { 0.0 } else { -1.0 };
                    let eye_dx = (x as f32 - 15.0 + eye_x) / 5.0;
                    let eye_dy = (y as f32 - 12.0 + eye_y) / 4.0;
                    if eye_dx * eye_dx + eye_dy * eye_dy < 1.0 {
                        img.put_pixel(px as u32, py as u32, colors::GLASSES_FRAME);
                    } else {
                        img.put_pixel(px as u32, py as u32, colors::GLASSES_LENS);
                    }
                }
            }
        }
    }
    
    // Right lens
    for y in 0..25u32 {
        for x in 0..30u32 {
            let px = cx as i32 + 5 + x as i32;
            let py = cy as i32 - 12 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 15.0) / 14.0;
                let dy = (y as f32 - 12.0) / 11.0;
                if dx * dx + dy * dy < 1.0 {
                    let eye_x = if pose == "searching" { 0.0 } else { -2.0 };
                    let eye_y = if pose == "searching" { 0.0 } else { -1.0 };
                    let eye_dx = (x as f32 - 15.0 + eye_x) / 5.0;
                    let eye_dy = (y as f32 - 12.0 + eye_y) / 4.0;
                    if eye_dx * eye_dx + eye_dy * eye_dy < 1.0 {
                        img.put_pixel(px as u32, py as u32, colors::GLASSES_FRAME);
                    } else {
                        img.put_pixel(px as u32, py as u32, colors::GLASSES_LENS);
                    }
                }
            }
        }
    }
    
    // Bridge
    for x in 0..15u32 {
        let px = cx as i32 - 7 + x as i32;
        let py = cy as i32 - 5;
        if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
            img.put_pixel(px as u32, py as u32, colors::GLASSES_FRAME);
        }
    }
    
    // Temples
    for x in 0..20u32 {
        let px = cx as i32 - 42 - x as i32;
        let py = cy as i32 - 5;
        if px >= 0 && py >= 0 {
            img.put_pixel(px as u32, py as u32, colors::GLASSES_FRAME);
        }
        let px = cx as i32 + 35 + x as i32;
        if (px as u32) < WIDTH {
            img.put_pixel(px as u32, py as u32, colors::GLASSES_FRAME);
        }
    }
}

fn draw_mustache(img: &mut RgbaImage, cx: f32, cy: f32) {
    // Handlebar mustache
    for y in 0..10u32 {
        for x in 0..40u32 {
            let px = cx as i32 - 20 + x as i32;
            let py = cy as i32 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 20.0) / 18.0;
                let dy = (y as f32 - 5.0) / 4.0;
                // Curved handlebar shape
                let curve = if x < 20 { -dx * 0.3 } else { dx * 0.3 };
                if dx * dx + (dy + curve).powi(2) < 1.0 {
                    img.put_pixel(px as u32, py as u32, colors::MUSTACHE);
                }
            }
        }
    }
}

fn draw_pipe(img: &mut RgbaImage, cx: f32, cy: f32, pose: &str) {
    // Pipe stem
    for y in 0..40u32 {
        for x in 0..8u32 {
            let px = cx as i32 - 4 + x as i32;
            let py = cy as i32 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 4.0) / 3.0;
                let dy = (y as f32 - 20.0) / 18.0;
                if dx * dx + dy * dy < 1.0 {
                    img.put_pixel(px as u32, py as u32, colors::PIPE);
                }
            }
        }
    }
    
    // Pipe bowl
    for y in 0..20u32 {
        for x in 0..25u32 {
            let px = cx as i32 - 12 + x as i32;
            let py = cy as i32 + 35 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 12.0) / 10.0;
                let dy = (y as f32 - 10.0) / 9.0;
                if dx * dx + dy * dy < 1.0 {
                    img.put_pixel(px as u32, py as u32, colors::PIPE);
                }
            }
        }
    }
}

fn draw_book(img: &mut RgbaImage, cx: f32, cy: f32) {
    // Book cover
    for y in 0..60u32 {
        for x in 0..50u32 {
            let px = cx as i32 - 25 + x as i32;
            let py = cy as i32 - 30 + y as i32;
            if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
                let dx = (x as f32 - 25.0) / 23.0;
                let dy = (y as f32 - 30.0) / 28.0;
                if dx * dx + dy * dy < 1.0 {
                    img.put_pixel(px as u32, py as u32, colors::BOOK);
                }
            }
        }
    }
    
    // Pages (white edge)
    for x in 0..5u32 {
        let px = cx as i32 - 26 + x as i32;
        let py = cy as i32 - 25;
        if px >= 0 && py >= 0 {
            img.put_pixel(px as u32, py as u32, colors::SHIRT);
        }
    }
}

fn draw_smoke(img: &mut RgbaImage, cx: f32, cy: f32, pose: &str) {
    let intensity = match pose {
        "thinking" => 8,
        "searching" => 10,
        _ => 5,
    };
    
    for i in 0..intensity {
        let x_off = (i as f32 * 7.0).sin() * 10.0;
        let y_off = i as f32 * -15.0;
        let px = cx as i32 + x_off as i32;
        let py = cy as i32 + y_off as i32;
        
        if px >= 0 && py >= 0 && (px as u32) < WIDTH && (py as u32) < HEIGHT {
            let mut smoke = colors::SMOKE;
            smoke.0[3] = (150 - i * 15).max(50);
            img.put_pixel(px as u32, py as u32, smoke);
        }
    }
}

fn generate_sprite_sheet() -> RgbaImage {
    let frame_w = 256;
    let frame_h = 256;
    let cols = 4;
    let rows = 1;
    
    let mut sprite: RgbaImage = ImageBuffer::from_pixel(
        (frame_w * cols) as u32,
        (frame_h * rows) as u32,
        colors::BACKGROUND,
    );
    
    let poses = vec!["idle", "thinking", "searching", "happy"];
    
    for (i, pose) in poses.iter().enumerate() {
        let frame = generate_professor(pose);
        let x_offset = (i as u32) * frame_w;
        
        for y in 0..frame_h {
            for x in 0..frame_w {
                let pixel = frame.get_pixel(x, y);
                sprite.put_pixel(x_offset + x, y, *pixel);
            }
        }
    }
    
    sprite
}

fn generate_simple_logo() -> RgbaImage {
    let size = 256;
    let mut img: RgbaImage = ImageBuffer::from_pixel(size, size, colors::BACKGROUND);
    
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    
    // Simple circular background
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < size as f32 * 0.45 {
                // Inner gradient
                let intensity = 1.0 - (dist / (size as f32 * 0.45)) * 0.3;
                let r = (46.0 * intensity) as u8;
                let g = (26.0 * intensity) as u8;
                let b = (76.0 * intensity) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }
    
    // Simple "A" letter shape (stylized)
    for y in 40..180u32 {
        for x in 80..176u32 {
            let px = cx - 48.0 + x as f32;
            let py = 40.0 + y as f32;
            
            // Left leg
            let left_leg = (px - cx + 48.0).abs() < 8.0 + (y as f32 - 40.0) * 0.2;
            // Right leg
            let right_leg = (px - cx - 48.0).abs() < 8.0 + (y as f32 - 40.0) * 0.2;
            // Crossbar
            let crossbar = y > 120 && y < 140 && (px - cx).abs() < 40.0;
            
            if (left_leg || right_leg || crossbar) && py >= 0.0 && py < size as f32 {
                img.put_pixel(x, y, colors::ACCENT);
            }
        }
    }
    
    img
}
