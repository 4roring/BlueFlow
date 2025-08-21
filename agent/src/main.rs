use rustautogui;
use std::fmt::Debug;

fn main() {
    let mut rustautogui = rustautogui::RustAutoGui::new(false).unwrap();
    let screen_size = rustautogui.get_screen_size();
    rustautogui
        .prepare_template_from_file(
            "D:/kwang1Projects/BlueFlow/agent/src/images/edge_icon.png",
            Some((
                0,
                0,
                screen_size.0.cast_unsigned(),
                screen_size.1.cast_unsigned(),
            )),
            rustautogui::MatchMode::Segmented,
        )
        .unwrap();

    let found_location = rustautogui
        .find_image_on_screen_and_move_mouse(0.9, 0.0)
        .unwrap()
        .unwrap();

    if found_location.len() == 0 {
        return;
    }

    for (x, y, z) in found_location.iter() {
        println!("{x} {y} {z}");
    }
}
