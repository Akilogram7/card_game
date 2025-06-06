/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use std::vec;

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
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "card_game".to_owned(),
        window_width: 3000,
        window_height: 1500,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut loca_box = vec![[980.0, 950.0], [1460.0, 950.0], [1940.0, 950.0]];

    let mut image_x = "assets/grey.png";

    let mut lbl_info = Label::new("", 50.0, 500.0, 30);

    let board = StillImage::new("assets/board_2.png", 3000.0, 1500.0, 0.0, 0.0, true, 1.0).await;

    let mut btn_start = TextButton::new(3000.0 / 2.0, 1500.0 / 2.0, 200.0, 60.0, "Start Game", BLUE, GREEN, 30);

    let mut btn_image = ImageButton::new(200.0, 1100.0, 300.0, 500.0, "assets/char_1.png", "assets/char_1_hover.png").await;
    let mut btn_image_2 = ImageButton::new(400.0, 1100.0, 300.0, 500.0, "assets/char_2.png", "assets/char_2_hover.png").await;

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
    let mut ready_game = false;
    let mut stored_char_a = "";
    let mut stored_char_b = "";
    let mut stored_char_c = "";

    let mut char_list = vec!["char_1", "char_2"];

    let mut enemy_chars: Option<Vec<&str>> = None;
    let mut enemy_displayed = false;

    // persistent enemy buttons
    let mut enemy_buttons: Vec<ImageButton> = vec![];

    loop {
        use_virtual_resolution(3000.0, 1500.0);
        clear_background(LIGHTGRAY);

        board.draw();
        draw_grid(50.0, BROWN);
        lbl_info.with_colors(WHITE, Some(DARKGRAY));

        if btn_start.click() && !start_game {
            start_game = true;
            btn_start.visible = false;
        }

        if start_game {
            if btn_image.click() {
                char_click = !char_click;
                char_type = "char_1";
                show_stat_text = !show_stat_text;
            } else if btn_image_2.click() {
                char_click = !char_click;
                char_type = "char_2";
                show_stat_text = !show_stat_text;
            }

            btn_place_a.click();
            btn_place_b.click();
            btn_place_c.click();

            if btn_place_a.click() {
                if stored_char_a == "char_1" {
                    show_stat_text = !show_stat_text;
                    char_type = "char_1";
                } else if stored_char_a == "char_2" {
                    show_stat_text = !show_stat_text;
                    char_type = "char_2";
                }
            } else if btn_place_b.click() {
                if stored_char_b == "char_1" {
                    show_stat_text = !show_stat_text;
                    char_type = "char_1";
                } else if stored_char_b == "char_2" {
                    show_stat_text = !show_stat_text;
                    char_type = "char_2";
                }
            } else if btn_place_c.click() {
                if stored_char_c == "char_1" {
                    show_stat_text = !show_stat_text;
                    char_type = "char_1";
                } else if stored_char_c == "char_2" {
                    show_stat_text = !show_stat_text;
                    char_type = "char_2";
                }
            }

            if show_stat_text && char_type == "char_1" {
                lbl_info
                    .with_colors(RED, Some(DARKGRAY))
                    .set_text("RED: \nAttack: 12\nHealth: 10")
                    .with_fixed_size(350.0, 200.0)
                    .set_font_size(50)
                    .with_border(BLACK, 2.0);
                lbl_info.draw();
            } else if show_stat_text && char_type == "char_2" {
                lbl_info
                    .with_colors(BLUE, Some(DARKGRAY))
                    .set_text("BLUE: \nAttack: 8\nHealth: 17")
                    .with_fixed_size(350.0, 200.0)
                    .set_font_size(50)
                    .with_border(BLACK, 2.0);
                lbl_info.draw();
            }

            if char_click {
                if btn_place_a.click() {
                    char_place = 1;
                } else if btn_place_b.click() {
                    char_place = 2;
                } else if btn_place_c.click() {
                    char_place = 3;
                }

                x.draw();
                x2.draw();
                x3.draw();
            }

            if char_place == 1 {
                let image_path = format!("assets/{}.png", char_type);
                btn_place_a.set_image(image_path.as_str(), &image_path.as_str()).await;
                btn_place_a.y = 800.0;
                btn_place_a.height = 500.0;
                x.visible = false;
                stored_char_a = char_type;
                char_place = 0;
                ready_btn_show = true;
            } else if char_place == 2 {
                let image_path = format!("assets/{}.png", char_type);
                btn_place_b.set_image(image_path.as_str(), &image_path.as_str()).await;
                btn_place_b.y = 800.0;
                btn_place_b.height = 500.0;
                x2.visible = false;
                stored_char_b = char_type;
                char_place = 0;
                ready_btn_show = true;
            } else if char_place == 3 {
                let image_path = format!("assets/{}.png", char_type);
                btn_place_c.set_image(image_path.as_str(), &image_path.as_str()).await;
                btn_place_c.y = 800.0;
                btn_place_c.height = 500.0;
                x3.visible = false;
                stored_char_c = char_type;
                char_place = 0;
                ready_btn_show = true;
            }

            if ready_btn_show {
                if btn_ready.click() {
                    btn_image.visible = false;
                    btn_image_2.visible = false;
                    btn_ready.visible = false;
                    char_click = false;
                    char_place = 0;
                    ready_game = true;
                }

                if ready_game && enemy_chars.is_none() {
                    let mut chosen = vec![];
                    while chosen.len() < 3 {
                        let rand_char = char_list.choose().unwrap();
                        chosen.push(*rand_char);
                    }
                    enemy_chars = Some(chosen);
                }
                //CODE FROM CHATGPT: MUST LEARN AND UNDERSTAND THIS
                if ready_game && !enemy_displayed {
                    if let Some(chars) = &enemy_chars {
                        let enemy_char_spot = vec![[940.0, 250.0], [1420.0, 250.0], [1900.0, 250.0]];

                        for (i, ch) in chars.iter().enumerate() {
                            let img_path = format!("assets/{}.png", ch);
                            let btn = ImageButton::new(enemy_char_spot[i][0], enemy_char_spot[i][1], 300.0, 500.0, &img_path, &img_path).await;
                            enemy_buttons.push(btn);
                        }

                        enemy_displayed = true;
                    }
                }

                for btn in &mut enemy_buttons {
                    btn.click();
                }
            }
        }

        next_frame().await;
    }
}
