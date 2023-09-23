//! Utility functions to create projection matrices.
//!
//! You should choose a submodule based on the coordinate system convetion that *your application*
//! is using, *not* which graphics api you are using. Then within that submodule, you can choose
//! a specific projection matrix constructor based on the *output* coordinate system, which will be
//! determined by the graphics api you're using.
//!
//! For example, if your code assumes that +X points right, +Y points up, and +Z points towards the
//! viewer, similar to Godot, Maya, Houdini, Substance, etc.:
//!
//! ```ignore,log
//!     +y
//!     |
//!     |
//!     0----- +x
//!    /
//!   /
//! +z
//! ```
//!
//! then you should use one of the projections listed in the [`rh_yup`] module. Since this is
//! generally considered to be the "most standard" (get out the pitchforks!) computer graphics
//! coordinate space, these projections are also re-exported at the root of the `projection`
//! module.
//!
//! If your code assumes that +X points right, +Y points up, and +Z points away from the viewer,
//! similar to Unity, Cinema4d, or ZBrush:
//!
//! ```ignore,log
//!     +y  +z
//!     |  /
//!     | /
//!     0 ----- +x
//! ```
//!
//! then you should use one of the projections listed in the [`lh_yup`] module.
//!
//! If you're building a 2d application which assumes the +X points right, +Y points down, and
//! +Z points towards the viewer (higher depth means "over" lower depth), then you should use
//! one of the projections listed in the [`lh_ydown`] module.
//!
//! If you're building a 3d application which uses a source Z-up coordinate space (similar to
//! Blender, 3ds max, or Unreal), then we do not currently have a module with projections
//! suitable for your use case. Contributions to add this are welcome!

pub mod lh_ydown;
pub mod lh_yup;
pub mod rh_yup;

pub use rh_yup::*;
