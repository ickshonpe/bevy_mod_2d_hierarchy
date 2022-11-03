# bevy_mod_2d_hierarchy

Alternative bevy transform plugin for more ergonomic 2d.

* Full 2d transform hierarchy.
* Independent of the Bevy 3d transform hierarcy, can use both in the same project.
* Compatible with existing plugins as long as they only query for `GlobalTransform` and not `Transform`.
* Control 2d transform propagation behaviour.
* Supports Bevy 0.8

## Usage

Add the dependency to your `Cargo.toml`
```toml
[dependencies]
bevy_mod_2d_hierarchy = "0.1"
```

Add the plugin to your Bevy App:
```rust
use bevy::prelude::*;
use bevy_mod_2d_hierarachy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(Hierarchy2dPlugin)
    // .. rest of systems etc
    .run();
}
```

Then you can do some 2d

```rust
pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle2::default());
    commands.spawn_bundle(SpriteBundle2 {
        texture: asset_server.load("sprite.png"),
        transform2: Transform2::from_rotation(0.5 * std::f32::consts::PI).with_scale(3.),
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn_bundle(SpriteBundle2 {
            sprite: Sprite { color: Color::YELLOW, ..Default::default() },
            texture: asset_server.load("sprite.png"),
            transform2: Transform2::from_xy(0., 32.),
            propagate: Propagate::TRANSLATION,
            ..Default::default()
        });
    });
}
```
