use macroquad::prelude::*;

#[macroquad::main("SpaceGame")]
async fn main() {
    let bytes = include_bytes!("../assets/space.png");
    let background: Texture2D = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));

    loop {
        clear_background(WHITE);
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
