use mlua::{Table, UserData, UserDataMethods};
use raylib::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Player {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Player {
    pub fn new() -> SharedPlayer {
        SharedPlayer(Arc::new(Mutex::new(Player {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        })))
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, 10.0, Color::RED);
    }
}

#[derive(Clone, Debug)]
pub struct SharedPlayer(pub Arc<Mutex<Player>>);

impl UserData for SharedPlayer {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("set_position", |_, this, new_position: Table| {
            let x = new_position.get::<_, f32>("x")?;
            let y = new_position.get::<_, f32>("y")?;
            this.0.lock().unwrap().position = Vector2::new(x, y);
            Ok(())
        });

        methods.add_method("set_velocity", |_, this, new_velocity: Table| {
            let x = new_velocity.get::<_, f32>("x")?;
            let y = new_velocity.get::<_, f32>("y")?;
            this.0.lock().unwrap().velocity = Vector2::new(x, y);
            Ok(())
        });

        methods.add_method("get_position", |lua, this, _: ()| {
            let p = this.0.lock().unwrap().position;
            let position = lua.create_table()?;
            position.set("x", p.x)?;
            position.set("y", p.y)?;
            Ok(position)
        });

        methods.add_method("get_velocity", |lua, this, _: ()| {
            let v = this.0.lock().unwrap().velocity;
            let velocity = lua.create_table()?;
            velocity.set("x", v.x)?;
            velocity.set("y", v.y)?;
            Ok(velocity)
        });
    }
}
