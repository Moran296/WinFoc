use libwmctl;
use libwmctl::Window;
use std::cmp;
use std::env;

use std::process::Command;

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "left" => Direction::LEFT,
            "right" => Direction::RIGHT,
            "up" => Direction::UP,
            "down" => Direction::DOWN,
            _ => panic!("no directon matched"),
        }
    }
}

#[allow(dead_code)]
fn win_debug(wins: &Vec<Window>) {
    for w in wins {
        print!("id: {}\n", w.id);
        print!("name: {}\n", w.name().unwrap());
        print!("g: {:?}\n", w.geometry().unwrap());
        print!("vg: {:?}\n", w.visual_geometry().unwrap());
        print!("state: {:?}\n", w.state().unwrap());
        print!("kind: {:?}\n", w.kind().unwrap());
    }
}

// fn find_next_window(dir: Direction, windows: &Vec<Window>, active_window: &Window) {}

fn calculate_overlap_area(win1: &Window, win2: &Window) -> i32 {
    let (x1, y1, w1, h1) = win1.visual_geometry().unwrap();
    let (x2, y2, w2, h2) = win2.visual_geometry().unwrap();

    let overlap_x1 = cmp::max(x1, x2);
    let overlap_y1 = cmp::max(y1, y2);
    let overlap_x2 = cmp::min(x1 + w1 as i32, x2 + w2 as i32);
    let overlap_y2 = cmp::min(y1 + h1 as i32, y2 + h2 as i32);

    let overlap_width = cmp::max(0, overlap_x2 - overlap_x1);
    let overlap_height = cmp::max(0, overlap_y2 - overlap_y1);

    return overlap_width * overlap_height;
}

fn find_obscured_windows(windows: &Vec<Window>) -> Vec<u32> {
    let mut obscured: Vec<u32> = vec![];

    for (i, win) in windows.iter().enumerate() {
        let (_, _, this_width, this_height) = win.visual_geometry().unwrap();
        let total_area = this_width * this_height;
        let mut obscured_area = 0;

        // for higher_win in windows[i+1:]:  Compare with all windows higher in the stack
        for higher_win in windows.iter().skip(i + 1) {
            let overlap_area = calculate_overlap_area(win, higher_win);
            obscured_area += overlap_area;

            if obscured_area > (0.15 * total_area as f32) as i32 {
                obscured.push(win.id);
                break;
            }
        }
    }

    return obscured;
}

fn get_windows(desktop: i32) -> Vec<Window> {
    let windows = libwmctl::windows_stack_order()
        .unwrap()
        .into_iter()
        .filter(|w| w.desktop().unwrap() == desktop)
        .filter(|w| !(w.state().unwrap().contains(&libwmctl::State::Hidden)))
        .filter(|w| w.mapped().unwrap() == libwmctl::MapState::Viewable)
        .collect();

    let obscured_windows = find_obscured_windows(&windows);
    windows
        .into_iter()
        .filter(|w| !obscured_windows.contains(&w.id))
        .collect()
}

fn get_x(window: &Window) -> i32 {
    window.visual_geometry().unwrap().0
}
fn get_y(window: &Window) -> i32 {
    window.visual_geometry().unwrap().1
}

fn find_next_window(dir: Direction, active_win: &Window, windows: &Vec<Window>) -> Option<u32> {
    let (current_x, current_y, _, _) = active_win.visual_geometry().unwrap();

    match dir {
        Direction::LEFT => windows
            .into_iter()
            .filter(|w| get_x(w) < current_x)
            .max_by(|a, b| get_x(a).cmp(&get_x(b)))
            .and_then(|w| Some(w.id)),
        Direction::RIGHT => windows
            .into_iter()
            .filter(|w| get_x(w) > current_x)
            .min_by(|a, b| get_x(a).cmp(&get_x(b)))
            .and_then(|w| Some(w.id)),
        Direction::UP => windows
            .into_iter()
            .filter(|w| get_y(w) < current_y)
            .max_by(|a, b| get_y(a).cmp(&get_y(b)))
            .and_then(|w| Some(w.id)),
        Direction::DOWN => windows
            .into_iter()
            .filter(|w| get_y(w) > current_y)
            .min_by(|a, b| get_y(a).cmp(&get_y(b)))
            .and_then(|w| Some(w.id)),
    }
}

fn focus_window_with_wmctrl(window_id: u32) {
    // Convert the window ID to a hexadecimal string (which wmctrl expects)
    let window_id_hex = format!("0x{:x}", window_id);

    // Run the wmctrl command to focus the window
    let output = Command::new("wmctrl")
        .arg("-ia")
        .arg(&window_id_hex) // Pass the window ID in hex format
        .output() // Execute the command and capture the output
        .expect("Failed to execute wmctrl command");

    // Check if the command was successful
    if output.status.success() {
        println!("Successfully focused window with ID: {}", window_id_hex);
    } else {
        eprintln!(
            "Failed to focus window. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut dir: Direction = Direction::LEFT;

    if !args.is_empty() {
        dir = Direction::from(args[0].as_str())
    }

    let active_win = libwmctl::active();
    let windows = get_windows(active_win.desktop().unwrap());

    if let Some(next_win) = find_next_window(dir, &active_win, &windows) {
        focus_window_with_wmctrl(next_win);
    } else {
        print!("no next window\n");
    }

    // win_debug(&windows);
}
