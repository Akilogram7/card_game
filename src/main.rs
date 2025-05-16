/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
use crate::modules::image_button::ImageButton;

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
    //let mut loca_list = vec![[550.0, 500.0], [850.0, 500.0]];
    //let mut rng_loca: ThreadRng = thread_rng();
    let mut i = 0.0;
    let mut image_x = "assets/x.png";
    //let mut rand_loca = 0;
   // while i < 2.0 {
    //    rand_loca = rng_loca.gen_range(0..2);
   //     i += 1.0;
   // }
    let board = StillImage::new(
        "assets/board_2.png",
        2050.0,  // width
        1080.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut btn_start = TextButton::new(
        950.0,
        500.0,
        200.0,
        60.0,
        "Start Game",
        BLUE,
        GREEN,
        30
    );
    let char_1 = StillImage::new(
        "assets/char_1.png",
        300.0,  // width
        500.0,  // height
        10.0,        //loca_list[rand_loca][0],  // x position
        10.0,    //loca_list[rand_loca][1],   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut x = StillImage::new(
        image_x,
        300.0,  // width
        500.0,  // height
        50.0,        //loca_list[rand_loca][0],  // x position
        50.0,    //loca_list[rand_loca][1],   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let btn_image = ImageButton::new(
        200.0,
        740.0,
        300.0,
        500.0,
        "assets/char_1.png",
        "assets/char_1_hover.png",
    ).await;

    let char_2 = StillImage::new(
        "assets/char_2.png",
        300.0,  // width
        500.0,  // height
        10.0, //loca_list[rand_loca][0],  // x position
        10.0, //loca_list[rand_loca][1],   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut start_game = false;
    let mut char_click = false;
    //let mut char_list = vec![char_1, char_2];
    //let mut rng: ThreadRng = thread_rng();
    //let mut rand = rng.gen_range(0..2);
    


    loop {
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
            }

            if char_click {
            x.draw();
            i += 1.0;
            if i >= 10.0 {
                i = 0.0;
                image_x = "assets/x_glow.png";
                x.draw();
            }
            else if i < 10.0 {
                image_x = "assets/x.png";
                x.draw();
            }
            }
            //char_list[rand].draw();
            //char_list[rand].draw();
        } 
        
        

        next_frame().await;
    }
}
