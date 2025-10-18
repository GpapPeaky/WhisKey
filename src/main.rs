use macroquad::prelude::*;
use std::time::Instant;      // Timers for enter and backspace when is_key_down(...)

mod editor;
use editor::Editor; // Import the struct

mod console;
use console::Console;

// TODO: Add scrollable screen 
// TODO: Add console system for goto_line, save_file, new_file, goto_dir etc commands
// TODO: Add file handling system
// TODO: Add palletes!
// TODO: Add instant cursor movement with Ctrl
// TODO: Add basic highlighting

#[macroquad::main("whiskey")]
async fn main() {
    set_fullscreen(true); // Window is now fullscreen

    let mut console = Console::new();
    let mut editor = Editor::new();
    
    // Top bar for info display
    let top_bar_margin:f32 = 30.0;

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

        if !console.console_mode { // Text mode 
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
            
            // Handle the cursor movement
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
        } else { // Console mode
            if let Some(c) = get_char_pressed() {
                if !c.is_control() { // If it is a control character, do nothing
                    console.insert_char(c);
                }
            }

            // Backspace
            // Handle first press
            if is_key_pressed(KeyCode::Backspace) {
                console.backspace();
                backspace_timer = Instant::now();
                backspace_held = true;
            }
            
            // Handle held key
            if is_key_down(KeyCode::Backspace) {
                let elapsed = backspace_timer.elapsed().as_secs_f32();
                if backspace_held && elapsed > repeat_delay {
                    console.backspace();
                    backspace_timer = Instant::now() - std::time::Duration::from_secs_f32(repeat_rate);
                }
            } else {
                backspace_held = false;
            }
            
            if is_key_pressed(KeyCode::Enter) { 
                console.execute();
            }
        }

        // Switch to console mode with CTRL + `
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::GraveAccent) {
            console.console_mode_switch();
        }

        for (i, line) in editor.text.iter().enumerate() {
            // Draw line number in gutter
            draw_text_ex(
                &format!("{}", i + 1),
                5.0, // left margin for line numbers
                top_bar_margin + 20.0 + i as f32 * font_size as f32, // same y as the text
                TextParams {
                    font: Some(&font),
                    font_size,
                    color: WHITE,
                    ..Default::default()
                },
            );

            // Text/line seperator
            draw_line(
            60.0,                                                 // x1: gutter separator
            top_bar_margin,                                       // y1: top of line
            60.0,                                                 // x2: same x for vertical line
            screen_height(),                                      // y2: bottom of line
            1.0,
            WHITE
        );
        
            // Draw the actual text
            draw_text_ex(
                line.as_str(),
                65.0,                       // Shift text to the right to leave space for numbers
                top_bar_margin + 20.0 + i as f32 * font_size as f32,
                TextParams {
                    font: Some(&font),
                    font_size,
                    color: WHITE,
                    ..Default::default()
                },
            );

            // Top bar line, display info on top of it 
            draw_line(0.0, top_bar_margin, screen_width(), top_bar_margin, 1.0, WHITE);
        }

        // Cursor blink timer
        let elapsed = cursor_timer.elapsed().as_secs_f32();
        if elapsed > cursor_rate {
            cursor_visible = !cursor_visible; // toggle visibility
            cursor_timer = Instant::now();    // reset timer
        }
        
        console.render_console();

        // Render cursor
        if cursor_visible && !console.console_mode { // Text mode
            let cursor_x = 60.0
                + measure_text(&editor.text[editor.cursor_y][..editor.cursor_x], Some(&font), font_size, 1.0).width +
                 5.0;

            let cursor_y = top_bar_margin + 
                25.0 +                          // File lines margin 
                editor.cursor_y as f32 *
                font_size as f32;

            draw_rectangle(cursor_x, cursor_y - font_size as f32, font_size as f32 / 6.0, font_size as f32, WHITE);
        } else if cursor_visible {              // Console mode
            let cursor_x = 5.0
                + measure_text(&editor.text[editor.cursor_y][..editor.cursor_x], Some(&font), font_size, 1.0).width + 
                5.0; // Padding

            let cursor_y = 
                screen_height() - 
                console::CONSOLE_HEIGHT + 
                font_size as f32 + 
                5.0;                    // Padding

            draw_rectangle(cursor_x, cursor_y - font_size as f32, font_size as f32 / 6.0, font_size as f32, WHITE);
        }

        next_frame().await;
    }

}

