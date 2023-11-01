mod fs_tree;
mod utils;

use std::{sync::mpsc, thread, path::Path};

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tree = fs_tree::FsTreeNode::from(Path::new("C:\\temp\\")).unwrap();
        tx.send(tree).unwrap();
    });

    let mut a: f32 = 0.0;

    let mut tree = fs_tree::FsTreeNode::new();

    let text = format!("Loading stats of {}", "C:\\Users\\");

    loop {
        a += get_frame_time() * 5.0;
        clear_background(WHITE);

        match rx.try_recv() {
            Err(_) => {
                let w = 50.0 + a.sin() * 50.0;
                let x = (screen_width() - w) / 2.0 + (a + 1.0).sin() * 25.0;
                draw_rectangle(x, screen_height() / 2.0 - 16.0, w, 10.0, GRAY);
                draw_text(
                    text.as_str(), 
                    screen_width() / 2.0 - get_text_center(text.as_str(), None, 32, 1.0, 0.0).x,
                    screen_height() / 2.0 - get_text_center(text.as_str(), None, 32, 1.0, 0.0).y + 16.0, 
                    32.0, 
                    GRAY
                );
            }
            Ok(t) => {
                tree = t;
                break;
            }
        }

        next_frame().await
    }

    let text = format!(
        "{:?}:{}:{}", 
        tree.get_type(), 
        tree.get_name(), 
        utils::bytes_to_string(tree.get_weight()
    ));

    loop {
        clear_background(WHITE);
        draw_text(
            text.as_str(), 
            screen_width() / 2.0 - get_text_center(text.as_str(), None, 32, 1.0, 0.0).x,
            screen_height() / 2.0 - get_text_center(text.as_str(), None, 32, 1.0, 0.0).y, 
            32.0, 
            GRAY
        );

        next_frame().await
    }
}