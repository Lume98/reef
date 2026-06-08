extern crate self as reef;

pub mod app;
pub mod core;
pub mod dom;
pub mod draw;
pub mod hooks;
pub mod layout;
pub mod query;
pub mod reconciler;
pub mod router;
pub mod theme;
pub mod view;
pub mod vnode;
pub mod widgets;

pub use app::App;
pub use draw::draw_backend;
pub use hooks::FiberId;
pub(crate) use hooks::HOOK_REGISTRY;
pub use reef_macros::{props, rsx};
pub use vnode::FunctionComponent;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::component_fn;
    pub use crate::core::{color::Color, geometry::*};
    pub use crate::dom::ReefRenderer;
    pub use crate::draw::{
        DrawBackend, DrawPlan, DrawPrimitive, FrameSubmission, PathSegment, TextAlignment,
        TextWeight,
    };
    pub use crate::props;
    pub use crate::rsx;
    pub use crate::view::widget_host::{PaintContext, Widget};
    pub use crate::vnode::{
        Badge, Button, CodeBlock, Column, Component, Container, Divider, ElementType,
        FunctionComponent, Icon, Image, Label, PropValue, PropsMap, Row, Spacer, Stack, VElement,
        VNode,
    };
    pub use crate::widgets::prelude::*;
}
