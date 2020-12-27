use macroquad::{Window, prelude::*};

fn tree(gl: &mut QuadGl, time: f64, deep: u32, angle: f32, tall: f32) {
    if deep >= 10 {
        return;
    }

    // we can use normal macroquad drawing API here
    draw_rectangle(-0.01 / 2., 0., 0.01, tall, GREEN);

    // and we can also modify internal macroquad render state with "gl" reference
    gl.push_model_matrix(glam::Mat4::from_translation(glam::vec3(0., tall, 0.)));

    // right leaf
    gl.push_model_matrix(glam::Mat4::from_rotation_z(angle + time.sin() as f32 * 0.1));
    tree(gl, time, deep + 1, angle * 0.7, tall * 0.8);
    gl.pop_model_matrix();

    // left leaf
    gl.push_model_matrix(glam::Mat4::from_rotation_z(
        -angle - time.cos() as f32 * 0.1,
    ));
    tree(gl, time, deep + 1, angle * 0.7, tall * 0.8);
    gl.pop_model_matrix();

    gl.pop_model_matrix();
}

//I'm not sure but it seems to me that it should be in some helpers of Macroquad
fn clamp(x: f32, low: f32, high: f32)->f32{
    match x{
        _ if x<low => low,
        _ if x>high => high,
        _ => x
    }
}

#[macroquad::main("Tree")]
async fn main() {
    let mut scale = 0.7; //good on 1920x1080 when fullscreen

    loop {
        let camera = Camera2D {
            zoom: vec2(1.*scale, scale*screen_width()/screen_height()), //fixed screen proportions
            target: vec2(0.0, 0.5),
            ..Default::default()
        };

        set_camera(camera);
        scale*= 1.0 + (mouse_wheel().1 as f32)/10.0; //modifying scale
        scale = clamp(scale, 0.02, 5.0); //making it not to go too far or too close
        //println!("{}", scale); //debug

        clear_background(BLUE);
        
        //draw_circle(0., 0., 0.03, GREEN); //initial position marker
        tree(unsafe { get_internal_gl().quad_gl }, get_time(), 0, 1., 0.3);

        next_frame().await
    }
}
