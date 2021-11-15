extern crate raylib;
use raylib::prelude::*;


struct Block{
    position: Vector3, 
    color: Color,  
}

impl Block {
    fn new() -> Block {
        return Block{position: Vector3::new(0.0, 0.0, 0.0), color: Color::RED}
    }
    fn set_activity(&mut self, active: bool) {
        if active == false {
            self.color = Color::LIME
        }
        else {
            self.color = Color::RED
        }
    }
}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    
    let mut framecounter: i32 = 0;
    let mut num = 0;
    let mut block_vec: Vec<Block> = Vec::new();
    
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 10.0, 10.0), 
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0), 
        45.0);

    block_vec.push(Block::new());
    let mut count = 0;
    
    // rl.set_camera_mode(&camera, CameraMode::CAMERA_FIRST_PERSON);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        
        if framecounter % 60 == 0 {
            num += 1
        }
        
        {
            let mut selected_block = &mut block_vec[count];
            selected_block.set_activity(true);
            if rl.is_key_down(KeyboardKey::KEY_DOWN){
                selected_block.position.z += 0.3;
            }
            else if rl.is_key_down(KeyboardKey::KEY_UP){
                selected_block.position.z -= 0.3;
            }
            if rl.is_key_down(KeyboardKey::KEY_RIGHT){
                selected_block.position.x += 0.3;
            }
            else if rl.is_key_down(KeyboardKey::KEY_LEFT){
                selected_block.position.x -= 0.3;
            }

            if rl.is_key_down(KeyboardKey::KEY_SPACE){
                selected_block.position.y += 0.3;
            }

            else if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
                selected_block.position.y -= 0.3;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_R){
                selected_block.set_activity(false);
                block_vec.push(Block::new());
                count += 1;
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_Q){
                selected_block.set_activity(true);
                block_vec.pop();
                count -= 1;
            }
        }
        framecounter += 1;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        {
            let mut nestedd2 = d.begin_mode3D(&camera);
            for cube in &block_vec {
                nestedd2.draw_cube(cube.position, 2.0, 2.0, 2.0, cube.color);
                nestedd2.draw_cube_wires(cube.position, 2.0, 2.0, 2.0, Color::MAROON);
            }
        }
        d.draw_text("Press R to desactive block", 12, 12, 20, Color::BLACK);
        d.draw_text("Press Q to active block", 12, 32, 20, Color::BLACK);
        let strg = format!("{}", num);
        d.draw_text(&strg, 595, 12, 20, Color::BLACK);
        
    }
}