use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        math::{Point3, Vector3},
        transform::Transform,
        Parent,
    },
    ecs::prelude::{Component, NullStorage},
    ecs::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        debug_drawing::DebugLinesComponent, transparent::Transparent, Camera, ImageFormat,
        SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
    },
    tiles::{MortonEncoder, Tile, TileMap},
    window::ScreenDimensions,
    winit,
};

#[derive(Default)]
struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CurrentState {
    Wait,
    Gameplay,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Gameplay
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAction {
    OpenMenu,
    Turn,
    Quit,
}

pub struct Game {
    pub user_action: Option<UserAction>,
    current_state: CurrentState,
}
impl Default for Game {
    fn default() -> Self {
        Game {
            user_action: None,
            current_state: CurrentState::default(),
        }
    }
}

pub struct PausedState;
impl SimpleState for PausedState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        Trans::None
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.write_resource::<Game>().current_state = CurrentState::Wait;
    }
}

pub struct Rl;
impl SimpleState for Rl {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Player>();

        let tiles_handle = load_spritesheet(world, "tiles.png", "tiles_manual.ron");
        let player = init_player(world, &tiles_handle);

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = init_camera(
            world,
            player,
            Transform::from(Vector3::new(0.0, 0.0, 0.1)),
            Camera::standard_2d(width, height),
        );

        let _ = world
            .create_entity()
            .with(DebugLinesComponent::with_capacity(1))
            .build();

        let map = TileMap::<ExampleTile, MortonEncoder>::new(
            Vector3::new(48, 48, 1),
            Vector3::new(20, 20, 1),
            Some(tiles_handle),
        );

        let _map_entity = world
            .create_entity()
            .with(map)
            .with(Transform::default())
            .build();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let mut game = data.world.write_resource::<Game>();
        if let Some(UserAction::Turn) = game.user_action.take() {
            println!("shit");
            return Trans::Push(Box::new(PausedState));
        }

        Trans::None
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.write_resource::<Game>().current_state = CurrentState::Gameplay;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

pub fn init_camera(
    world: &mut World,
    parent: Entity,
    transform: Transform,
    camera: Camera,
) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(Parent { entity: parent })
        .with(camera)
        .named("camera")
        .build()
}

fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 32,
    };
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}

fn load_spritesheet(
    world: &mut World,
    spritesheet: &str,
    spritesheet_ron: &str,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}", spritesheet),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        format!("texture/{}", spritesheet_ron),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
