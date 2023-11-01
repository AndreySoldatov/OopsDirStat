use macroquad::{
    shapes::{
        draw_rectangle, 
        draw_rectangle_lines
    }, 
    window::{
        screen_width, screen_height
    }, 
    color, 
    prelude::{GRAY}, 
    text::{draw_text, get_text_center}
};

use rand::prelude::*;

use crate::{fs_tree, utils};

impl fs_tree::FsTreeNode {
    pub fn draw(&self, active_index: usize) {
        if let fs_tree::EntryType::Dir = self.get_type() {
            let mut x = 0.0;

            let mut rng = SmallRng::seed_from_u64(self.get_weight());

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

                let mut y = 5.0;

                for sentry in entry.get_children().iter() {
                    let sprop = (sentry.get_weight() as f32) / (entry.get_weight() as f32);
                    draw_rectangle(
                        x + 5.0, 
                        y, 
                        screen_width() * prop - 10.0, 
                        (screen_height() - 37.0 - 5.0 * entry.get_children().len() as f32) * sprop,
                        color::hsl_to_rgb(col.0, col.1, col.2 + 0.1)
                    );
                    y += (screen_height() - 37.0 - 5.0 * entry.get_children().len() as f32) * sprop + 5.0;
                }

                if i == active_index {
                    draw_rectangle_lines(
                        x, 
                        0.0, 
                        screen_width() * prop, 
                        screen_height() - 32.0, 
                        8.0,
                        GRAY
                    );  
                }
                x += screen_width() * prop;
            }

            let active_subdir_str = format!("{:?}:{}:{}",
                self.get_children()[active_index].get_type(),
                self.get_children()[active_index].get_name(),
                utils::bytes_to_string(self.get_children()[active_index].get_weight())
            );

            draw_text(
                &active_subdir_str, 
                10.0,
                screen_height() - 10.0, 
                32.0, 
                GRAY
            );
        } else {
            let fstr = format!("File: {} | {}", self.get_name(), utils::bytes_to_string(self.get_weight()));
            let tc = get_text_center(&fstr, None, 32, 1.0, 0.0);
            draw_text(
                &fstr, 
                screen_width() / 2.0 - tc.x, 
                (screen_height() - 32.0) / 2.0 - tc.y, 
                32.0, 
                GRAY
            );
        }

        let active_dir_str = format!("{:?}:{}:{}",
            self.get_type(),
            self.get_name(),
            utils::bytes_to_string(self.get_weight())
        );

        draw_text(
            &active_dir_str, 
            screen_width() - get_text_center(&active_dir_str, None, 32, 1.0, 0.0).x * 2.0 - 10.0,
            screen_height() - 10.0, 
            32.0,
            GRAY
        );
    }
}