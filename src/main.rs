use macroquad::prelude::*;
use std::time::Instant;      // Timers for enter and backspace when is_key_down(...)

mod editor;
use editor::Editor; // Import the struct

#[macroquad::main("whiskey")]
async fn main() {
    set_fullscreen(true); // Window is now fullscreen

    let mut editor = Editor::new();

    // Enter press timer
    let mut enter_timer = Instant::now();
    let mut enter_held = false;

    // Backspace press timer
    let mut backspace_timer = Instant::now();
    let mut backspace_held = false;
    
    // Timer parameters
    let repeat_delay = 0.1;   // seconds before repeat starts
    let repeat_rate = 0.05;   // seconds per repeat after that

    // Cursor blink
    let mut cursor_timer = Instant::now();
    let mut cursor_visible = true;
    let cursor_rate = 0.45; // seconds per blink

    // Font size
    let font_size = 22;

    loop {
        clear_background(BLACK);

        // Input handle
        if let Some(c) = get_char_pressed() {
            if !c.is_control() { // If it is a control character, do nothing
                editor.insert_char(c);
            }
        }

        // Newline
        // Handle first press
        if is_key_pressed(KeyCode::Enter) {
            editor.new_line();
            enter_timer = Instant::now();
            enter_held = true;
        }
        
        // Handle held key
        if is_key_down(KeyCode::Enter) {
            let elapsed = enter_timer.elapsed().as_secs_f32();
            if enter_held && elapsed > repeat_delay {
                editor.new_line();
                enter_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
            }
        } else {
            enter_held = false;
        }
        
        // Backspace
        // Handle first press
        if is_key_pressed(KeyCode::Backspace) {
            editor.backspace();
            backspace_timer = Instant::now();
            backspace_held = true;
        }
        
        // Handle held key
        if is_key_down(KeyCode::Backspace) {
            let elapsed = backspace_timer.elapsed().as_secs_f32();
            if backspace_held && elapsed > repeat_delay {
                editor.backspace();
                backspace_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
            }
        } else {
            backspace_held = false;
        }
        
        // Handle the cursor movement
        if is_key_pressed(KeyCode::Up) {
            editor.move_cursor(KeyCode::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            editor.move_cursor(KeyCode::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            editor.move_cursor(KeyCode::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            editor.move_cursor(KeyCode::Right);
        }

        // Render text
        for (i, line) in editor.text.iter().enumerate() {
            draw_text(
                line.as_str(),
                15.0,
                20.0 + i as f32 * font_size as f32,
                font_size as f32,
                WHITE,
            );
        }

        // Cursor blink timer
        let elapsed = cursor_timer.elapsed().as_secs_f32();
        if elapsed > cursor_rate {
            cursor_visible = !cursor_visible; // toggle visibility
            cursor_timer = Instant::now();    // reset timer
        }
        
        // Render cursor
        if cursor_visible {
            let cursor_x = 10.0
                + measure_text(&editor.text[editor.cursor_y][..editor.cursor_x], None, font_size, 1.0).width + 5.0;
            let cursor_y = 25.0 + editor.cursor_y as f32 * font_size as f32;
            draw_rectangle(cursor_x, cursor_y - font_size as f32, 2.0, font_size as f32, WHITE);
        }

        next_frame().await;

    }
}
