mod fs_tree;
mod fs_draw_tree;
mod utils;

use std::{sync::mpsc, thread::{self}, path::Path};

use fs_tree::FsTreeNode;
use macroquad::prelude::*;

use std::env;

#[macroquad::main("OopsDirStat")]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("NotoSans-Regular.ttf")).unwrap();
    let font_size = 18;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || !Path::new(&args[1]).exists() {
        loop {
            clear_background(WHITE);

            draw_text_ex(
                "Please provide a valid path!", 
                screen_width() / 2.0 - get_text_center("Please provide a valid path!", Some(&font), font_size, 1.0, 0.0).x,
                screen_height() / 2.0 - get_text_center("Please provide a valid path!", Some(&font), font_size, 1.0, 0.0).y, 
                TextParams { 
                    font: Some(&font), 
                    font_size, 
                    color: GRAY,
                    ..Default::default()
                }
            );

            next_frame().await
        }
    }

    let (tx, rx) = mpsc::channel();

    let root_thread_path = String::from(&args[1]);
    thread::spawn(move || {
        let tree = fs_tree::FsTreeNode::from(Path::new(&root_thread_path)).unwrap();
        tx.send(tree).unwrap();
    });

    let mut a: f32 = 0.0;

    let tree: FsTreeNode;

    let time_start = get_time();

    loop {
        let text = format!("Loading stats for \"{}\" | {:.1}s", &args[1], get_time() - time_start);
        a += get_frame_time() * 5.0;

        clear_background(WHITE);

        match rx.try_recv() {
            Err(_) => {
                let w = 50.0 + a.sin() * 50.0;
                let x = (screen_width() - w) / 2.0 + (a + 1.0).sin() * 25.0;
                draw_rectangle(x, screen_height() / 2.0 - 16.0, w, 10.0, GRAY);
                draw_text_ex(
                    text.as_str(), 
                    screen_width() / 2.0 - get_text_center(text.as_str(), Some(&font), font_size, 1.0, 0.0).x,
                    screen_height() / 2.0 - get_text_center(text.as_str(), Some(&font), font_size, 1.0, 0.0).y + 16.0, 
                    TextParams { 
                        font: Some(&font), 
                        font_size, 
                        color: GRAY,
                        ..Default::default()
                    }
                );
            }
            Ok(t) => {
                tree = t;
                break;
            }
        }

        next_frame().await
    }

    let mut parent_node: Vec<&FsTreeNode> = Vec::new();
    let mut active_node = &tree;
    let mut active_index: usize = 0;

    loop {
        clear_background(WHITE);

        active_node.draw(active_index, Some(&font), font_size);

        if is_key_pressed(KeyCode::Right) {
            active_index = (active_index + 1) % active_node.get_children().len();
        } else if is_key_pressed(KeyCode::Left) {
            if active_index > 0 {
                active_index -= 1;
            } else {
                active_index = active_node.get_children().len() - 1;
            }
        } else if is_key_pressed(KeyCode::Enter) {
            if let fs_tree::EntryType::Dir = active_node.get_type() {
                parent_node.push(active_node);
                active_node = &active_node.get_children()[active_index];
                active_index = 0;
            }
        } else if is_key_pressed(KeyCode::Escape) && parent_node.len() > 0 {
            active_node = parent_node.pop().unwrap();
            active_index = 0;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mut x = 0.0;
            let mousex = mouse_position().0;
            for (i, entry) in active_node.get_children().iter().enumerate() {
                let prop = (entry.get_weight() as f32) / (active_node.get_weight() as f32);
                let width = screen_width() * prop;

                if mousex >= x && mousex <= x + width {
                    if active_index != i {
                        active_index = i;
                    } else {
                        parent_node.push(active_node);
                        active_node = &active_node.get_children()[active_index];
                        active_index = 0;
                    }
                    break;
                }

                x += width;
            }
        } else if is_mouse_button_pressed(MouseButton::Right) && parent_node.len() > 0 {
            active_node = parent_node.pop().unwrap();
            active_index = 0;
        }

        next_frame().await
    }
}