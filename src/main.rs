/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use macroquad::miniquad::window;
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
use crate::modules::image_button::ImageButton;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::animated_image::AnimatedImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions; // If you want to customize the loading screen


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "card_game".to_owned(),
        window_width: 3000,
        window_height: 1500,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut loca_box = vec![[980.0, 950.0], [1460.0, 950.0], [1940.0, 950.0]];

    let mut image_x = "assets/grey.png";

    let tm = TextureManager::new();
    tm.preload_all(&["assets/char_1.png", "assets/char_2.png"]).await;
   

    let board = StillImage::new(
        "assets/board_2.png",
        3000.0,  // width
        1500.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut btn_start = TextButton::new(
        3000.0 / 2.0,
        1500.0 / 2.0,
        200.0,
        60.0,
        "Start Game",
        BLUE,
        GREEN,
        30
    );

    let btn_image = ImageButton::new(
        200.0,
        1100.0,
        300.0,
        500.0,
        "assets/char_1.png",
        "assets/char_1_hover.png",
    ).await;

    let btn_image_2 = ImageButton::new(
        400.0,
        1100.0,
        300.0,
        500.0,
        "assets/char_2.png",
        "assets/char_2_hover.png",
    ).await;

    let mut btn_place_a = ImageButton::new(
        870.0,
        900.0,
        329.0,
        243.0,
        image_x,
        image_x,
    ).await;

    let mut btn_place_b = ImageButton::new(
        1350.0,
        900.0,
        329.0,
        243.0,
        image_x,
        image_x,
    ).await;

    let mut btn_place_c = ImageButton::new(
        1830.0,
        900.0,
        329.0,
        243.0,
        image_x,
        image_x,
    ).await;

    let mut x = AnimatedImage::from_gif(
        "assets/x.gif", 
        loca_box[0][0],
        loca_box[0][1],          
        128.0, 128.0,          
        true        
    ).await;

    let mut x2 = AnimatedImage::from_gif(
        "assets/x.gif", 
        loca_box[1][0],
        loca_box[1][1],          
        128.0, 128.0,          
        true                   
    ).await;

    let mut x3 = AnimatedImage::from_gif(
        "assets/x.gif", 
        loca_box[2][0],
        loca_box[2][1],          
        128.0, 128.0,          
        true                   
    ).await;

    let mut start_game = false;
    let mut char_click = false;
    let mut char_type = "";
    let mut char_place = 0;

    


    loop {
         use_virtual_resolution(3000.0, 1500.0);
        clear_background(LIGHTGRAY);
        
        board.draw();
        draw_grid(50.0, BROWN);
        if btn_start.click() && !start_game {
            start_game = true;
            btn_start.visible = false;
            //rand = rng.gen_range(0..2);
        }

        if start_game {
            if btn_image.click() {
                char_click = true;
                char_type = "char_1";
            }
            if btn_image_2.click() {
                char_click = true;
                char_type = "char_2";
            }

            if char_click{
                
                if btn_place_a.click() {
                char_place = 1;
                }
                else if btn_place_b.click() {
                    char_place = 2;
                }
                else if btn_place_c.click() {
                    char_place = 3;
                }

                if char_place == 1 {
                    let image_x_string = format!("assets/{}.png", char_type);
                    image_x = Box::leak(image_x_string.into_boxed_str());
                    btn_place_a = ImageButton::new(
                        870.0,
                        900.0,
                        329.0,
                        243.0,
                        image_x,
                        image_x,
                    ).await;
                    x.visible = false;
                    char_place = 0;
                }
                else if char_place == 2 {
                    let image_x_string = format!("assets/{}.png", char_type);
                    image_x = Box::leak(image_x_string.into_boxed_str());
                    btn_place_b = ImageButton::new(
                        1350.0,
                        900.0,
                        329.0,
                        243.0,
                        image_x,
                        image_x,
                    ).await;
                    x2.visible = false;
                    char_place = 0;
                }
                else if char_place == 3 {
                    let image_x_string = format!("assets/{}.png", char_type);
                    image_x = Box::leak(image_x_string.into_boxed_str());
                    btn_place_c = ImageButton::new(
                        1830.0,
                        900.0,
                        329.0,
                        243.0,
                        image_x,
                        image_x,
                    ).await;
                    x3.visible = false;
                    char_place = 0;
                }
                
                    x.draw();
                    x2.draw();
                    x3.draw();
                
            }

            

        
        
    }
        next_frame().await;
    
}
}