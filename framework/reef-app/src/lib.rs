pub mod app;

pub use app::App;

pub mod prelude {
    pub use crate::App;
    pub use reef_core::{color::Color, geometry::*};
    pub use reef_dom::ReefRenderer;
    pub use reef_draw::{DrawBackend, DrawPlan, DrawPrimitive, FrameSubmission};
    pub use reef_view_macros::{props, rsx};
    pub use reef_vnode::{
        component_fn, Badge, Button, CodeBlock, Column, Component, Container, Divider, ElementType,
        FunctionComponent, Icon, Image, Label, PropValue, PropsMap, Row, Spacer, Stack, VElement,
        VNode,
    };
}
