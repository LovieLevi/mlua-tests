pub mod player;
use player::*;

use mlua::prelude::*;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_WARNING)
        .size(800, 450)
        .title("Lua Intergation Example")
        .build();
    rl.set_target_fps(60);

    let lua_setup_start = std::time::Instant::now();
    let lua = Lua::new();
    let globals = lua.globals();
    globals
        .set(
            "PlayerDef",
            lua.create_proxy::<SharedPlayer>()
                .expect("Failed to create proxy for \"SharedPlayer\""),
        )
        .expect("Failed to set \"PlayerDef\" global");

    lua.load(std::fs::read_to_string("main.lua").expect("Failed to read \"main.lua\""))
        .exec()
        .expect("Failed to execute \"main.lua\"");

    let player = Player::new();
    globals
        .set("Player", player.clone())
        .expect("Failed to set \"Player\" global");
    let lua_setup_time = lua_setup_start.elapsed();

    {
        let mut player = player.0.lock().unwrap();
        player.position = Vector2::new(
            rl.get_screen_width() as f32 / 2.0,
            rl.get_screen_height() as f32 / 2.0,
        );
    }

    // Call the `init` function in Lua
    let lua_init_start = std::time::Instant::now();
    match globals.get::<_, mlua::Function>("Init") {
        Ok(init) => {
            init.call::<_, ()>(())
                .expect("Failed to call \"Init\" function");
        }
        Err(_) => {}
    }
    let lua_init_time = lua_init_start.elapsed();

    let has_update = globals.get::<_, mlua::Function>("Update").is_ok();
    let mut update_times = Vec::new();

    while !rl.window_should_close() {
        // Update

        globals
            .set("DeltaTime", rl.get_frame_time())
            .expect("Failed to set \"delta_time\" global");
        globals
            .set("ScreenWidth", rl.get_screen_width())
            .expect("Failed to set \"screen_width\" global");
        globals
            .set("ScreenHeight", rl.get_screen_height())
            .expect("Failed to set \"screen_height\" global");
        globals
            .set("Time", rl.get_time())
            .expect("Failed to set \"time\" global");

        // Call the `update` function in Lua
        let lua_update_start = std::time::Instant::now();
        if has_update {
            globals
                .get::<_, mlua::Function>("Update")
                .unwrap()
                .call::<_, ()>(())
                .expect("Failed to call \"Update\" function");
        }
        let lua_update_time = lua_update_start.elapsed();
        update_times.push(lua_update_time);
        if update_times.len() > 100 {
            update_times.remove(0);
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut player = player.0.lock().unwrap();
            player.update();
            player.draw(&mut d);
        }

        d.draw_fps(2, 2);

        d.draw_text(
            &format!(
                "Lua Setup Time: {:.3}ms",
                lua_setup_time.as_micros() as f32 / 1000.0
            ),
            2,
            20,
            20,
            Color::BLACK,
        );

        d.draw_text(
            &format!(
                "Lua Init Time: {:.3}ms",
                lua_init_time.as_micros() as f32 / 1000.0
            ),
            2,
            40,
            20,
            Color::BLACK,
        );

        if has_update {
            let avg_update_time = update_times.iter().sum::<std::time::Duration>()
                / update_times.len() as u32;

            d.draw_text(
                &format!(
                    "Lua Update Time Avg: {:.3}ms",
                    avg_update_time.as_micros() as f32 / 1000.0
                ),
                2,
                60,
                20,
                Color::BLACK,
            );
        }
    }
}
