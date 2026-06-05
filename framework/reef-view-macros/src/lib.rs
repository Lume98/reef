use proc_macro::TokenStream;

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
    let _ = input;
    TokenStream::new()
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
    let _ = input;
    TokenStream::new()
}
