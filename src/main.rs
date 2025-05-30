/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use crate::modules::animated_image::AnimatedImage;
use crate::modules::grid::draw_grid;
use crate::modules::image_button::ImageButton;
use crate::modules::label::Label;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::miniquad::window;
use macroquad::prelude::*; // If you want to customize the loading screen

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

    let mut lbl_info = Label::new("", 50.0, 500.0, 30);
    //let tm = TextureManager::new();
    //tm.preload_all(&["assets/char_1.png", "assets/char_2.png"]).await;

    let board = StillImage::new(
        "assets/board_2.png",
        3000.0, // width
        1500.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    let mut btn_start = TextButton::new(3000.0 / 2.0, 1500.0 / 2.0, 200.0, 60.0, "Start Game", BLUE, GREEN, 30);

    let btn_image = ImageButton::new(200.0, 1100.0, 300.0, 500.0, "assets/char_1.png", "assets/char_1_hover.png").await;

    let btn_image_2 = ImageButton::new(400.0, 1100.0, 300.0, 500.0, "assets/char_2.png", "assets/char_2_hover.png").await;

    let mut btn_place_a = ImageButton::new(870.0, 900.0, 329.0, 243.0, image_x, image_x).await;

    let mut btn_place_b = ImageButton::new(1350.0, 900.0, 329.0, 243.0, image_x, image_x).await;

    let mut btn_place_c = ImageButton::new(1830.0, 900.0, 329.0, 243.0, image_x, image_x).await;

    let mut btn_ready = TextButton::new(3000.0 / 2.0, 1500.0 / 2.0, 200.0, 60.0, "Ready", BLUE, GREEN, 30);

    let mut x = AnimatedImage::from_gif("assets/x.gif", loca_box[0][0], loca_box[0][1], 128.0, 128.0, true).await;

    let mut x2 = AnimatedImage::from_gif("assets/x.gif", loca_box[1][0], loca_box[1][1], 128.0, 128.0, true).await;

    let mut x3 = AnimatedImage::from_gif("assets/x.gif", loca_box[2][0], loca_box[2][1], 128.0, 128.0, true).await;

    let mut start_game = false;
    let mut char_click = false;
    let mut char_type = "";
    let mut char_place = 0;
    let mut show_stat_text = false;
    let mut ready_btn_show = false;

    loop {
        use_virtual_resolution(3000.0, 1500.0);
        clear_background(LIGHTGRAY);

        board.draw();
        draw_grid(50.0, BROWN);
        lbl_info.with_colors(WHITE, Some(DARKGRAY));
        if btn_start.click() && !start_game {
            start_game = true;
            btn_start.visible = false;
            //rand = rng.gen_range(0..2);
        }

        if start_game {
            if btn_image.click() {
                char_click = !char_click;
                char_type = "char_1";
                show_stat_text = true;
            }
            if btn_image_2.click() {
                char_click = !char_click;
                char_type = "char_2";
                show_stat_text = true;
            }
            // Constantly draw the buttons to make sure characters are visible even when not trying to place them
            btn_place_a.click();
            btn_place_b.click();
            btn_place_c.click();

            if show_stat_text && char_type == "char_1" && char_click {
                lbl_info
                    .with_colors(RED, Some(DARKGRAY))
                    .set_text("RED: \nAttack: 12\nHealth: 10")
                    .with_fixed_size(350.0, 200.0)
                    .set_font_size(50)
                    .with_border(BLACK, 2.0);
                lbl_info.draw();
            } else if show_stat_text && char_type == "char_2" && char_click {
                lbl_info
                    .with_colors(BLUE, Some(DARKGRAY))
                    .set_text("BLUE: \nAttack: 8\nHealth: 17")
                    .with_fixed_size(350.0, 200.0)
                    .set_font_size(50)
                    .with_border(BLACK, 2.0);
                lbl_info.draw();
            }
            // If the character is clicked, then figure out where it is placed
            if char_click {
                if btn_place_a.click() {
                    char_place = 1;
                } else if btn_place_b.click() {
                    char_place = 2;
                } else if btn_place_c.click() {
                    char_place = 3;
                }
                //draw the gifs
                x.draw();
                x2.draw();
                x3.draw();
            }
            //Change the character image based on the player clicked and remove the x gif from the screen and then set the character spot placed back to 0
            if char_place == 1 {
                let image_x_string = format!("assets/{}.png", char_type);
                image_x = Box::leak(image_x_string.into_boxed_str());
                btn_place_a = ImageButton::new(870.0, 800.0, 300.0, 500.0, image_x, image_x).await;
                x.visible = false;
                char_place = 0;
                ready_btn_show = true;
            } else if char_place == 2 {
                let image_x_string = format!("assets/{}.png", char_type);
                image_x = Box::leak(image_x_string.into_boxed_str());
                btn_place_b = ImageButton::new(1350.0, 800.0, 300.0, 500.0, image_x, image_x).await;
                x2.visible = false;
                char_place = 0;
                ready_btn_show = true;
            } else if char_place == 3 {
                let image_x_string = format!("assets/{}.png", char_type);
                image_x = Box::leak(image_x_string.into_boxed_str());
                btn_place_c = ImageButton::new(1830.0, 800.0, 300.0, 500.0, image_x, image_x).await;
                x3.visible = false;
                char_place = 0;
                ready_btn_show = true;
            }
            if ready_btn_show {
                btn_ready.click();
            }
        }
        next_frame().await;
    }
}
