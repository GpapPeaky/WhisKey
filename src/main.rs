use macroquad::prelude::*;
use std::time::Instant;      // Timers for enter and backspace when is_key_down(...)

mod editor;
use editor::Editor; // Import the struct

// TODO: Add basic highlighting
// TODO: Add file handling system
// TODO: Add console system for goto_line, save_file, new_file, goto_dir etc commands
// TODO: Add palletes!
// TODO: Add instant cursor movement with Ctrl

#[macroquad::main("whiskey")]
async fn main() {
    set_fullscreen(true); // Window is now fullscreen

    // Camera positions, to move with the cursor
    let mut camera_x: f32 = 0.0;
    let mut camera_y: f32 = 0.0;

    // Pixels from edge before we start scrolling
    let scroll_margin = 100.0;

    let mut editor = Editor::new();

    // Enter press timer
    let mut enter_timer = Instant::now();
    let mut enter_held = false;

    // Backspace press timer
    let mut backspace_timer = Instant::now();
    let mut backspace_held = false;
    
    // Timer parameters
    let repeat_delay = 0.15;   // seconds before repeat starts
    let repeat_rate = 0.05;    // seconds per repeat after that

    // Cursor blink
    let mut cursor_timer = Instant::now();
    let mut cursor_visible = true;
    let cursor_rate = 0.45; // seconds per blink

    // Cursor movement
    let mut cursor_movement_timer = Instant::now();
    let mut cursor_movement_held = false;

    // Font
    let font_size = 22;
    let font: Font = load_ttf_font("assets/fonts/Courier Prime.ttf").await.unwrap();

    loop {
        clear_background(BLACK);

        // Input handle
        if let Some(c) = get_char_pressed() {
            if !c.is_control() { // If it is a control character, do nothing
                editor.insert_char(c);
            }
        }

        // Tab key
        if is_key_pressed(KeyCode::Tab) {
            editor.insert_tab();
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
        
        // Cursor movement
        // Handle first press
        if is_key_pressed(KeyCode::Up) {
            editor.move_cursor(KeyCode::Up);
            cursor_movement_timer = Instant::now();
            cursor_movement_held = true;
        }
        if is_key_pressed(KeyCode::Down) {
            editor.move_cursor(KeyCode::Down);
            cursor_movement_timer = Instant::now();
            cursor_movement_held = true;
        }
        if is_key_pressed(KeyCode::Left) {
            editor.move_cursor(KeyCode::Left);
            cursor_movement_timer = Instant::now();
            cursor_movement_held = true;
        }
        if is_key_pressed(KeyCode::Right) {
            editor.move_cursor(KeyCode::Right);
            cursor_movement_timer = Instant::now();
            cursor_movement_held = true;
        }

        // Handle held key
        if is_key_down(KeyCode::Up) {
            let elapsed = cursor_movement_timer.elapsed().as_secs_f32();
            if cursor_movement_held && elapsed > repeat_delay {
                editor.move_cursor(KeyCode::Up);
                cursor_movement_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
            }
        } else if is_key_down(KeyCode::Down) {
            let elapsed = cursor_movement_timer.elapsed().as_secs_f32();
            if cursor_movement_held && elapsed > repeat_delay {
                editor.move_cursor(KeyCode::Down);
                cursor_movement_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
            }
        } else if is_key_down(KeyCode::Left) {
            let elapsed = cursor_movement_timer.elapsed().as_secs_f32();
            if cursor_movement_held && elapsed > repeat_delay {
                editor.move_cursor(KeyCode::Left);
                cursor_movement_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
            }
        } else if is_key_down(KeyCode::Right) {
            let elapsed = cursor_movement_timer.elapsed().as_secs_f32();
            if cursor_movement_held && elapsed > repeat_delay {
                editor.move_cursor(KeyCode::Right);
                cursor_movement_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
            }
        } else {
            cursor_movement_held = false;
        }

        for (i, line) in editor.text.iter().enumerate() {
            // Draw line number in gutter
            draw_text_ex(
                &format!("{}", i + 1),
                5.0 - camera_x, // left margin for line numbers
                20.0 + i as f32 * font_size as f32 - camera_y, // same y as the text
                TextParams {
                    font: Some(&font),
                    font_size,
                    color: WHITE,
                    ..Default::default()
                },
            );

            // Text/line seperator
            draw_line(
            60.0,                                // x1: gutter separator
            0.0,  // y1: top of line
            60.0,                                // x2: same x for vertical line
            screen_height(), // y2: bottom of line
            1.0,                                 // stroke width
            WHITE                                 // color
        );
        
            // Draw the actual text
            draw_text_ex(
                line.as_str(),
                65.0 - camera_x,                       // Shift text to the right to leave space for numbers
                20.0 + i as f32 * font_size as f32 - camera_y,
                TextParams {
                    font: Some(&font),
                    font_size,
                    color: WHITE,
                    ..Default::default()
                },
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
            let cursor_x = 60.0
                + measure_text(&editor.text[editor.cursor_y][..editor.cursor_x], Some(&font), font_size, 1.0).width + 5.0;
            let cursor_y = 25.0 + editor.cursor_y as f32 * font_size as f32;
        
            draw_rectangle(
                cursor_x - camera_x,
                cursor_y - font_size as f32 - camera_y,
                font_size as f32 / 8.0,
                font_size as f32,
                WHITE,
            );
        }

        // Get screen size
        let (screen_w, screen_h) = (screen_width(), screen_height());
        
        // Calculate cursor position in world space (not offset)
        let cursor_x = 60.0
            + measure_text(&editor.text[editor.cursor_y][..editor.cursor_x], Some(&font), font_size, 1.0).width
            + 5.0;
        let cursor_y = 25.0 + editor.cursor_y as f32 * font_size as f32;
        
        // Horizontal scrolling
        if cursor_x - camera_x > screen_w - scroll_margin {
            camera_x = cursor_x - (screen_w - scroll_margin);
        } else if cursor_x - camera_x < scroll_margin {
            camera_x = (cursor_x - scroll_margin).max(0.0);
        }
        
        // Vertical scrolling
        if cursor_y - camera_y > screen_h - scroll_margin {
            camera_y = cursor_y - (screen_h - scroll_margin);
        } else if cursor_y - camera_y < scroll_margin {
            camera_y = (cursor_y - scroll_margin).max(0.0);
        }
        

        next_frame().await;
    }
}
