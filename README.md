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
        transform2: Transform2::from_rotation(0.5 * std::f32::consts::PI).with_scale(3.),
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn(SpriteBundle2 {
            sprite: Sprite { color: Color::YELLOW, ..Default::default() },
            texture: asset_server.load("sprite.png"),
            transform2: Transform2::from_xy(0., 32.),
            propagate: Propagate::TRANSLATION,
            ..Default::default()
        });
    });
}
```

## Notes

See also https://github.com/devil-ira/bevy_mod_transform2d

bevy_mod_transform2d has better compatibility, you can use the existing sprite and text bundles and it works with Rapier2d. You can also include 2d and 3d transforms in the same hierarchy. 

bevy_mod_2d_hierarchy main advantages are that is has maybe marginally better performance (you won't notice the difference), no synchronisation worries, and transform propagation control.

At the moment it doesn't seem possible to combine both sets of features in a third-party plugin.

Other minor differences are that bevy_mod_transform2d has seperate values for x and y scaling, and stores z depths in a seperate component. 





