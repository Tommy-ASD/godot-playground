use godot::{
    engine::{Sprite2D, Sprite2DVirtual},
    prelude::{
        gdextension, godot_api, godot_print, Base, ExtensionLibrary, GodotClass, Input, Vector2,
    },
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,

    #[base]
    sprite: Base<Sprite2D>,
}

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            sprite,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        match (
            input.is_action_pressed("ui_right".into()),
            input.is_action_pressed("ui_left".into()),
        ) {
            (true, false) => {
                self.sprite
                    .translate(Vector2::RIGHT * self.speed as f32 * delta as f32);
                godot_print!("ui_right pressed")
            }
            (false, true) => {
                godot_print!("ui_left pressed")
            }
            _ => {} // if neither or both are pressed
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.sprite.emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}
