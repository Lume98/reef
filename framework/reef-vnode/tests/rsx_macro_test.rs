use reef_vnode::*;

#[test]
fn rsx_simple_element() {
    let vnode = reef_view_macros::rsx! {
        <container>
            <label />
        </container>
    };

    match vnode {
        VNode::VElement(ref el) => {
            assert_eq!(el.ty, ElementType::Native("container"));
            assert_eq!(el.children.len(), 1);
            match &el.children[0] {
                VNode::VElement(child) => {
                    assert_eq!(child.ty, ElementType::Native("label"));
                }
                _ => panic!("expected label child"),
            }
        }
        _ => panic!("expected container element"),
    }
}

#[test]
fn rsx_element_with_attrs() {
    use reef_core::color::Color;

    let vnode = reef_view_macros::rsx! {
        <container color={Color::rgb(18, 18, 22)} radius={12.0} />
    };

    match vnode {
        VNode::VElement(ref el) => {
            assert_eq!(el.ty, ElementType::Native("container"));
            assert!(el.props.get("color").is_some());
            assert!(el.props.get("radius").is_some());
        }
        _ => panic!("expected container element"),
    }
}

#[test]
fn rsx_string_attr() {
    let vnode = reef_view_macros::rsx! {
        <label text={"Hello World"} />
    };

    match vnode {
        VNode::VElement(ref el) => {
            assert_eq!(el.ty, ElementType::Native("label"));
            match el.props.get("text").unwrap() {
                PropValue::String(s) => assert_eq!(s, "Hello World"),
                _ => panic!("expected string prop"),
            }
        }
        _ => panic!("expected label element"),
    }
}

#[test]
fn rsx_nested_elements() {
    let vnode = reef_view_macros::rsx! {
        <container>
            <row>
                <label text={"A"} />
                <label text={"B"} />
            </row>
        </container>
    };

    match vnode {
        VNode::VElement(ref el) => {
            assert_eq!(el.ty, ElementType::Native("container"));
            assert_eq!(el.children.len(), 1);

            let row = &el.children[0];
            if let VNode::VElement(ref row_el) = row {
                assert_eq!(row_el.ty, ElementType::Native("row"));
                assert_eq!(row_el.children.len(), 2);
            } else {
                panic!("expected row element");
            }
        }
        _ => panic!("expected container element"),
    }
}

#[test]
fn rsx_expression_child() {
    let extra = reef_vnode::element("extra", PropsMap::new(), vec![]);

    let vnode = reef_view_macros::rsx! {
        <container>
            {extra}
        </container>
    };

    match vnode {
        VNode::VElement(ref el) => {
            assert_eq!(el.children.len(), 1);
            match &el.children[0] {
                VNode::VElement(ref child) => {
                    assert_eq!(child.ty, ElementType::Native("extra"));
                }
                _ => panic!("expected extra element"),
            }
        }
        _ => panic!("expected container element"),
    }
}

#[test]
fn props_macro() {
    use reef_core::color::Color;

    let map = reef_view_macros::props! {
        color: Color::rgb(255, 0, 0),
        radius: 8.0,
        visible: true,
    };

    assert!(map.get("color").is_some());
    assert!(map.get("radius").is_some());
    assert!(map.get("visible").is_some());
}
