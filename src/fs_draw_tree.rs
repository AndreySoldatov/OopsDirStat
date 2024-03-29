use macroquad::{
    shapes::{
        draw_rectangle, 
        draw_rectangle_lines
    }, 
    window::{
        screen_width, screen_height
    }, 
    color, 
    prelude::GRAY, 
    text::{draw_text_ex, get_text_center, Font, TextParams}
};

use rand::prelude::*;

use crate::{fs_tree, utils};

impl fs_tree::FsTreeNode {
    pub fn draw(&self, active_index: usize, font: Option<&Font>, font_size: u16) {
        //TODO: This needs refactoring:
        if let fs_tree::EntryType::Dir = self.get_type() {
            if self.get_children().len() > 0 {
                let mut x = 0.0;

                let mut rng = SmallRng::seed_from_u64(self.get_weight());

                let mut outline_x: f32 = 0.0;
                let mut outline_width: f32 = 0.0;

                for (i, entry) in self.get_children().iter().enumerate() {
                    let prop = (entry.get_weight() as f32) / (self.get_weight() as f32);

                    let col: (f32, f32, f32) = (rng.gen_range(0.0..1.0), 0.8, 0.7);

                    draw_rectangle(
                        x, 
                        0.0, 
                        screen_width() * prop, 
                        screen_height() - 32.0, 
                        color::hsl_to_rgb(col.0, col.1, col.2)
                    );

                    let mut y = 0.0;

                    for sentry in entry.get_children().iter() {
                        let sprop = (sentry.get_weight() as f32) / (entry.get_weight() as f32);
                        draw_rectangle(
                            x,
                            y, 
                            screen_width() * prop, 
                            (screen_height() - 32.0)  * sprop,
                            color::hsl_to_rgb(col.0, col.1, col.2 + rng.gen_range(-0.1..0.1))
                        );
                        y += (screen_height() - 32.0) * sprop;
                    }

                    if i == active_index {
                        outline_x = x;
                        outline_width = screen_width() * prop;
                    }
                    x += screen_width() * prop;
                }

                draw_rectangle_lines(
                    outline_x, 
                    0.0, 
                    outline_width, 
                    screen_height() - 32.0, 
                    8.0,
                    GRAY
                );

                let active_subdir_str = format!("{:?} | {} | {}",
                    self.get_children()[active_index].get_type(),
                    self.get_children()[active_index].get_name(),
                    utils::bytes_to_string(self.get_children()[active_index].get_weight())
                );

                draw_text_ex(
                    &active_subdir_str, 
                    10.0,
                    screen_height() - 10.0, 
                    TextParams { 
                        font, 
                        font_size, 
                        color: GRAY,
                        ..Default::default()
                    }
                );
            } else {
                let dstr = format!("Directory \"{}\" is empty", self.get_name());
                let tc = get_text_center(&dstr, font, font_size, 1.0, 0.0);
                draw_text_ex(
                    &dstr, 
                    screen_width() / 2.0 - tc.x, 
                    (screen_height() - 32.0) / 2.0 - tc.y, 
                    TextParams { 
                        font, 
                        font_size, 
                        color: GRAY,
                        ..Default::default()
                    }
                );
            }
        } else {
            let fstr = format!("File: {} | {}", self.get_name(), utils::bytes_to_string(self.get_weight()));
            let tc = get_text_center(&fstr, font, font_size, 1.0, 0.0);
            draw_text_ex(
                &fstr, 
                screen_width() / 2.0 - tc.x, 
                (screen_height() - 32.0) / 2.0 - tc.y, 
                TextParams { 
                    font, 
                    font_size, 
                    color: GRAY,
                    ..Default::default()
                }
            );
        }

        let active_dir_str = format!("{:?} | {} | {}",
            self.get_type(),
            self.get_name(),
            utils::bytes_to_string(self.get_weight())
        );

        draw_text_ex(
            &active_dir_str, 
            screen_width() - get_text_center(&active_dir_str, font, font_size, 1.0, 0.0).x * 2.0 - 10.0,
            screen_height() - 10.0, 
            TextParams { 
                font, 
                font_size, 
                color: GRAY,
                ..Default::default()
            }
        );
    }
}