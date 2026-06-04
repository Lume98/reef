mod root;
pub mod widget_host;

pub use root::{create_root, WidgetRoot};
pub use widget_host::{
    dispatch_to_child, EventContext, PaintContext, Widget, WidgetHost,
};
