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
use bevy_mod_2d_transform_hierarachy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(Transform2Plugin)
    // .. rest of systems etc
    .run();
}
```

Then you can spawn 

