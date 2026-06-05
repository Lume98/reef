use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{token, Ident, LitStr, Token};

// The braced! macro is exported from syn::parse — import it at use sites
use syn::braced;

// ── AST types ─────────────────────────────────────────────────────

enum RsxNode {
    Element {
        name: String,
        attrs: Vec<RsxAttr>,
        children: Vec<RsxNode>,
        self_closing: bool,
    },
    Text(String),
    Expr(TokenStream2),
}

struct RsxAttr {
    name: String,
    value: RsxAttrValue,
}

enum RsxAttrValue {
    Str(String),
    Expr(TokenStream2),
    Bool(bool),
}

/// A list of JSX root nodes (implicit fragment).
struct RsxRoot(Vec<RsxNode>);

// ── Parser ────────────────────────────────────────────────────────

impl Parse for RsxRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            nodes.push(input.parse::<RsxNode>()?);
        }
        Ok(RsxRoot(nodes))
    }
}

impl Parse for RsxNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            // ── Parse element: <tagName attrs>children</tagName> ──
            let _: Token![<] = input.parse()?;

            if input.peek(Token![/]) {
                return Err(input.error("unexpected closing tag"));
            }

            let name: Ident = input.parse()?;
            let tag_name = name.to_string();

            // Parse attributes
            let mut attrs = Vec::new();
            while !input.peek(Token![>]) && !input.peek(Token![/]) {
                if input.is_empty() {
                    return Err(input.error("unterminated tag"));
                }
                attrs.push(input.parse::<RsxAttr>()?);
            }

            // Self-closing?
            let self_closing = if input.peek(Token![/]) {
                let _: Token![/] = input.parse()?;
                let _: Token![>] = input.parse()?;
                true
            } else {
                let _: Token![>] = input.parse()?;
                false
            };

            let children = if self_closing {
                Vec::new()
            } else {
                let mut children = Vec::new();
                loop {
                    if input.is_empty() {
                        return Err(input.error(format!("expected closing tag </{}>", tag_name)));
                    }
                    if input.peek(Token![<]) && input.peek2(Token![/]) {
                        let _: Token![<] = input.parse()?;
                        let _: Token![/] = input.parse()?;
                        let close_name: Ident = input.parse()?;
                        if close_name.to_string() != tag_name {
                            return Err(input.error(format!(
                                "mismatched closing tag: </{}> expected </{}>",
                                close_name, tag_name
                            )));
                        }
                        let _: Token![>] = input.parse()?;
                        break;
                    }
                    children.push(input.parse::<RsxNode>()?);
                }
                children
            };

            Ok(RsxNode::Element {
                name: tag_name,
                attrs,
                children,
                self_closing,
            })
        } else if input.peek(token::Brace) {
            // ── Expression interpolation: {expr} ──
            let content;
            braced!(content in input);
            let tokens: TokenStream2 = content.parse()?;
            Ok(RsxNode::Expr(tokens))
        } else if input.peek(LitStr) {
            // ── Text node: "string content" ──
            let s: LitStr = input.parse()?;
            Ok(RsxNode::Text(s.value()))
        } else {
            Err(input.error("expected JSX element `<tag>`, expression `{expr}`, or string `\"text\"`"))
        }
    }
}

impl Parse for RsxAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let attr_name = name.to_string();

        if input.peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            if input.peek(LitStr) {
                let s: LitStr = input.parse()?;
                Ok(RsxAttr {
                    name: attr_name,
                    value: RsxAttrValue::Str(s.value()),
                })
            } else if input.peek(token::Brace) {
                let expr;
                braced!(expr in input);
                let tokens: TokenStream2 = expr.parse()?;
                Ok(RsxAttr {
                    name: attr_name,
                    value: RsxAttrValue::Expr(tokens),
                })
            } else {
                Err(input.error("expected string literal or {expression} after ="))
            }
        } else {
            // Boolean attribute (no value → true)
            Ok(RsxAttr {
                name: attr_name,
                value: RsxAttrValue::Bool(true),
            })
        }
    }
}

// ── Code generation ───────────────────────────────────────────────

fn gen_prop_value(value: &RsxAttrValue) -> TokenStream2 {
    match value {
        RsxAttrValue::Str(s) => {
            quote! { ::reef_vnode::PropValue::String(#s.to_string()) }
        }
        RsxAttrValue::Expr(tokens) => {
            quote! { ::reef_vnode::PropValue::from(#tokens) }
        }
        RsxAttrValue::Bool(b) => {
            quote! { ::reef_vnode::PropValue::Bool(#b) }
        }
    }
}

fn gen_node(node: &RsxNode) -> TokenStream2 {
    match node {
        RsxNode::Element {
            name,
            attrs,
            children,
            self_closing: _,
        } => {
            let child_code: Vec<TokenStream2> = children.iter().map(|c| gen_node(c)).collect();

            if attrs.is_empty() {
                quote! {
                    ::reef_vnode::element(#name, ::reef_vnode::PropsMap::new(), vec![#(#child_code),*])
                }
            } else {
                let attr_stmts: Vec<TokenStream2> = attrs
                    .iter()
                    .map(|a| {
                        let key = &a.name;
                        let val = gen_prop_value(&a.value);
                        quote! { __props.insert(#key, #val); }
                    })
                    .collect();

                quote! {
                    {
                        let mut __props = ::reef_vnode::PropsMap::new();
                        #(#attr_stmts)*
                        ::reef_vnode::element(#name, __props, vec![#(#child_code),*])
                    }
                }
            }
        }
        RsxNode::Text(text) => {
            quote! { ::reef_vnode::VNode::VText(#text.to_string()) }
        }
        RsxNode::Expr(tokens) => {
            quote! { #tokens }
        }
    }
}

fn gen_root(root: &RsxRoot) -> TokenStream2 {
    let nodes: Vec<TokenStream2> = root.0.iter().map(|n| gen_node(n)).collect();
    if nodes.len() == 1 {
        nodes.into_iter().next().unwrap()
    } else {
        quote! { ::reef_vnode::VNode::VFragment(vec![#(#nodes),*]) }
    }
}

// ── Proc-macro entry points ───────────────────────────────────────

/// Build a VNode tree using JSX-like syntax.
///
/// # Example
/// ```ignore
/// rsx! {
///     <container color={Color::rgb(18, 18, 22)} radius={12.0}>
///         <label text={"Hello"} />
///     </container>
/// }
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match syn::parse2::<RsxRoot>(input2) {
        Ok(root) => {
            let expanded = gen_root(&root);
            expanded.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}

/// Build a `PropsMap` literal from key-value pairs.
///
/// # Example
/// ```ignore
/// props! {
///     color: Color::rgb(255, 0, 0),
///     radius: 12.0,
/// }
/// ```
#[proc_macro]
pub fn props(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);

    // Parse key: value, key: value, ...
    let result = syn::parse2::<PropsMacroInput>(input2).map(|parsed| {
        let stmts: Vec<TokenStream2> = parsed
            .pairs
            .iter()
            .map(|p| {
                let key = p.key.to_string();
                let val = gen_prop_value(&p.value);
                quote! { __props.insert(#key, #val); }
            })
            .collect();

        quote! {
            {
                let mut __props = ::reef_vnode::PropsMap::new();
                #(#stmts)*
                __props
            }
        }
    });

    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

struct PropsMacroInput {
    pairs: Vec<PropPair>,
}

struct PropPair {
    key: Ident,
    _colon: Token![:],
    value: RsxAttrValue,
}

impl Parse for PropsMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut pairs = Vec::new();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let _colon: Token![:] = input.parse()?;

            let value = if input.peek(LitStr) {
                let s: LitStr = input.parse()?;
                RsxAttrValue::Str(s.value())
            } else if input.peek(token::Brace) {
                let expr;
                braced!(expr in input);
                let tokens: TokenStream2 = expr.parse()?;
                RsxAttrValue::Expr(tokens)
            } else {
                let expr: syn::Expr = input.parse()?;
                RsxAttrValue::Expr(quote! { #expr })
            };

            pairs.push(PropPair {
                key,
                _colon,
                value,
            });

            // Optional comma
            let _ = input.parse::<Token![,]>();
        }
        Ok(PropsMacroInput { pairs })
    }
}
