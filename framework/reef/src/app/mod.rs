pub mod app;

pub use app::App;

pub mod prelude {
    pub use crate::component_fn;
    pub use crate::core::{color::Color, geometry::*};
    pub use crate::dom::ReefRenderer;
    pub use crate::draw::{DrawBackend, DrawPlan, DrawPrimitive, FrameSubmission};
    pub use crate::vnode::{
        Badge, Button, CodeBlock, Column, Component, Container, Divider, ElementType,
        FunctionComponent, Icon, Image, Label, PropValue, PropsMap, Row, Spacer, Stack, VElement,
        VNode,
    };
    pub use crate::App;
    pub use crate::{props, rsx};
}
