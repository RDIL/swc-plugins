mod minify;
mod pass;
mod transform;

pub mod types;
pub use minify::minify;
pub use plugin_import;
pub use plugin_modularize_imports;
pub use plugin_react_utils;
pub use plugin_lock_corejs_version;
pub use transform::transform;
pub use swc_emotion as plugin_emotion;
pub use styled_components as plugin_styled_components;
pub use styled_jsx as plugin_styled_jsx;