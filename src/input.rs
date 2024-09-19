use crate::prelude::*;

/// The set that contains all input
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct Dir4Input {
    pressed: HashMap<Dir4, bool>,
    just_pressed: HashMap<Dir4, bool>,
}
impl Dir4Input {
    pub fn as_vec2(&self) -> Vec2 {
        let mut result = Vec2::ZERO;
        if self[Dir4::Up] {
            result.y += 1.0;
        }
        if self[Dir4::Down] {
            result.y -= 1.0;
        }
        if self[Dir4::Left] {
            result.x -= 1.0;
        }
        if self[Dir4::Right] {
            result.x += 1.0;
        }
        result
    }
}
impl std::ops::Index<Dir4> for Dir4Input {
    type Output = bool;
    fn index(&self, index: Dir4) -> &Self::Output {
        self.pressed.get(&index).unwrap_or(&false)
    }
}
impl Dir4Input {
    pub fn just_pressed(&self, dir: Dir4) -> bool {
        self.just_pressed.get(&dir).unwrap_or(&false).clone()
    }
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum GButton {
    A,
    B,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct GButtonInput {
    // A little cursed but it's fine
    pressed: HashMap<GButton, bool>,
    just_pressed: HashMap<GButton, bool>,
}
impl GButtonInput {
    pub fn pressed(&self, but: GButton) -> bool {
        self.pressed[&but].clone()
    }
    pub fn just_pressed(&self, but: GButton) -> bool {
        self.just_pressed[&but].clone()
    }
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum SpecialButton {
    Start,
    Select,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct SpecialButtonInput {
    // A little cursed but it's fine
    pressed: HashMap<SpecialButton, bool>,
    just_pressed: HashMap<SpecialButton, bool>,
}
impl SpecialButtonInput {
    pub fn pressed(&self, but: SpecialButton) -> bool {
        self.pressed[&but].clone()
    }
    pub fn just_pressed(&self, but: SpecialButton) -> bool {
        self.just_pressed[&but].clone()
    }
}

fn update_input_from_keyboard(
    mut dir4_input: ResMut<Dir4Input>,
    mut gbutton_input: ResMut<GButtonInput>,
    mut special_button_input: ResMut<SpecialButtonInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    dir4_input
        .pressed
        .insert(Dir4::Up, keyboard.pressed(KeyCode::KeyW));
    dir4_input
        .pressed
        .insert(Dir4::Down, keyboard.pressed(KeyCode::KeyS));
    dir4_input
        .pressed
        .insert(Dir4::Left, keyboard.pressed(KeyCode::KeyA));
    dir4_input
        .pressed
        .insert(Dir4::Right, keyboard.pressed(KeyCode::KeyD));
    dir4_input
        .just_pressed
        .insert(Dir4::Up, keyboard.just_pressed(KeyCode::KeyW));
    dir4_input
        .just_pressed
        .insert(Dir4::Down, keyboard.just_pressed(KeyCode::KeyS));
    dir4_input
        .just_pressed
        .insert(Dir4::Left, keyboard.just_pressed(KeyCode::KeyA));
    dir4_input
        .just_pressed
        .insert(Dir4::Right, keyboard.just_pressed(KeyCode::KeyD));
    gbutton_input
        .pressed
        .insert(GButton::A, keyboard.pressed(KeyCode::KeyJ));
    gbutton_input
        .just_pressed
        .insert(GButton::A, keyboard.just_pressed(KeyCode::KeyJ));
    gbutton_input
        .pressed
        .insert(GButton::B, keyboard.pressed(KeyCode::KeyK));
    gbutton_input
        .just_pressed
        .insert(GButton::B, keyboard.just_pressed(KeyCode::KeyK));
    special_button_input
        .pressed
        .insert(SpecialButton::Start, keyboard.pressed(KeyCode::Enter));
    special_button_input
        .just_pressed
        .insert(SpecialButton::Start, keyboard.just_pressed(KeyCode::Enter));
    special_button_input
        .pressed
        .insert(SpecialButton::Select, keyboard.pressed(KeyCode::Escape));
    special_button_input.just_pressed.insert(
        SpecialButton::Select,
        keyboard.just_pressed(KeyCode::Escape),
    );
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dir4Input::default());
        app.insert_resource(GButtonInput::default());
        app.insert_resource(SpecialButtonInput::default());

        app.add_systems(Update, update_input_from_keyboard.in_set(InputSet));
    }
}
