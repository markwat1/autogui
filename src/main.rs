use rustautogui;

fn main() {
    let rag = rustautogui::RustAutoGui::new(false); // arg: debug
    let mut rag = match rag {
        Ok(rag) => {
            rag
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    };   
    let screen_size = rag.get_screen_size();
    println!("Screen size: {:?}", screen_size);
    rag.prepare_template_from_file(
       "close_button.png",
       Some((0,0,screen_size.0.try_into().unwrap(),screen_size.1.try_into().unwrap())),
       rustautogui::MatchMode::Segmented
    ).unwrap();

    let right_down = (screen_size.0 - 1, screen_size.1 - 1);
    let mut pre_position = (0,0);
    while rag.get_mouse_position().unwrap() != right_down {
        let new_position = rag.get_mouse_position().unwrap();
        if new_position != pre_position {
            println!("Mouse position: {:?}", new_position);
        }
        pre_position = new_position;
//        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    rag.save_screenshot("screenshot.png").unwrap();
    let found = rag.find_image_on_screen(0.9);
    println!("Found: {:?}", found);
}

