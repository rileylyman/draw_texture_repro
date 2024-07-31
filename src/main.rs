use macroquad::{conf::Conf, input, prelude::*};

fn window_conf() -> Conf {
    Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "".into(),
            window_width: 2048,
            window_height: 2048,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let atlas_texture = load_texture("spritesheet.png").await.unwrap();
    atlas_texture.set_filter(FilterMode::Nearest);

    loop {
        if input::is_key_pressed(KeyCode::Escape) {
            break;
        }

        set_camera(&Camera2D {
            target: vec2(64.0, 64.0),
            zoom: vec2(8.0 / screen_width(), 8.0 / screen_height()),
            ..Default::default()
        });

        clear_background(BLACK);
        draw_rectangle(
            0.0,
            0.0,
            atlas_texture.width() as f32,
            atlas_texture.height() as f32,
            RED,
        );
        draw_texture_ex(
            &atlas_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
