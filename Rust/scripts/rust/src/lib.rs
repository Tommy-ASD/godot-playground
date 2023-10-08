use godot::{
    engine::{CollisionShape2D, RigidBody2D, RigidBody2DVirtual, Sprite2D, Texture2D},
    prelude::{
        gdextension, godot_api, godot_print, load, Base, ExtensionLibrary, Gd, GodotClass, Input,
        Node, Vector2,
    },
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
struct Player {
    speed: f64,

    #[base]
    sprite: Base<RigidBody2D>,
}

#[godot_api]
impl RigidBody2DVirtual for Player {
    fn init(sprite: Base<RigidBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self { speed: 1.0, sprite }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let collisions = self.sprite.get_colliding_bodies();

        let translation;

        match (
            input.is_action_pressed("ui_right".into()),
            input.is_action_pressed("ui_left".into()),
        ) {
            (true, false) => {
                translation = Vector2::RIGHT;
                godot_print!("ui_right pressed")
            }
            (false, true) => {
                translation = Vector2::LEFT;
                godot_print!("ui_left pressed")
            }
            _ => {
                translation = Vector2::ZERO;
            } // if neither or both are pressed
        }
        self.sprite
            .translate(translation * self.speed as f32 * delta as f32);
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
