# bevy_mod_2d_hierarchy

[![crates.io](https://img.shields.io/crates/v/bevy_mod_2d_heirarchy)](https://crates.io/crates/bevy_mod_2d_hierarchy)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_mod_2d_hierarchy)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_2d_hierarchy)](https://crates.io/crates/bevy_mod_2d_hierarchy)

Bevy plugin for more ergonomic 2D.

* Specialized 2D transform and propagation systems.
* Independent of the Bevy 3D transform systems, can use both in the same project. 
* Compatible with existing plugins as long as they only query for `GlobalTransform` and not `Transform`.
* Control 2D transform propagation behaviour.
* Performance similar to the 3D Transform. The propagation control has a cost, but some operations cheaper in 2D.
* No quaternions.
* Supports Bevy 0.9

## Usage

Add the dependency to your `Cargo.toml`
```toml
[dependencies]
bevy_mod_2d_hierarchy = "0.3"
```

Add the plugin to your Bevy App:
```rust
use bevy::prelude::*;
use bevy_mod_2d_hierarachy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(Transform2dPlugin)
    // .. rest of systems etc
    .run();
}
```

Then spawn some sprites 

```rust
pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle2::default());
    commands.spawn(SpriteBundle2 {
        texture: asset_server.load("sprite.png"),
        transform2: Transform2::from_rotation(0.5 * PI).with_scale(3.),
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn(SpriteBundle2 {
            sprite: Sprite { 
                color: Color::YELLOW, 
                ..Default::default() 
            },
            texture: asset_server.load("sprite.png"),
            transform2: Transform2::from_xy(0., 32.),
            propagate: Propagate::TRANSLATION,
            ..Default::default()
        });
    });
}
```

## Notes

See also: https://github.com/devil-ira/bevy_mod_transform2d.

Major differences Transform2d vs 2d_hierarchy:

* Transform2d is compatible with systems that query for Transform. No 3rd party physics and collision detection crates will work with 2d_hierarchy; for that reason alone, Transform2d is the better choice for most users.
* 2d_hierarchy is marginally more efficient (but neither library is performance focused, and you probably won't even be to able to measure the difference). Might be room to improve 2d_hierarchy
* 2d_hierarchy uses a single f32 for scale and Transform2d uses a Vec2. 
* Transform2d has a seperate component for Z depth, while 2d_hierarchy keeps it in the transform.
* Because 2d_hierarchy is incompatible with Transform you can't use Bevy's builtin bundles like SpriteBundle and have to use the provided replacement SpriteBundle2 (or make your own bundle). 
* You can have mix entities with Transform2d and regular Transforms in the same transform tree. 2d_hierarchy you can't. Useful if you want to mix 2d and 3d.
* Transform2d there is the worry of transform synchronization problems, but they aren't likely and should be easy to fix etc.
* 2d_hierarchy you can control which properties are propagated down the transform tree (useful for text captions above rotated and scaled sprites).

You could even use Transform2d and 2d_hierarchy together if you really wanted. That would be really silly though.











