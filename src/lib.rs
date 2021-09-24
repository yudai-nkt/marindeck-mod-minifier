use minifier::css::minify;
use wasm_bindgen::prelude::*;

/// Represents a result of CSS minification.
/// This struct bypasses the restriction of `Result` not implemnting `IntoWasmAbi`
/// and converts the result into a TypeScript-friendly structure.
#[wasm_bindgen(getter_with_clone)]
pub struct MinificationResult {
    /// Set to `true` if minification succeeds, `false` otherwise.
    pub success: bool,
    /// Contains the minified CSS if `success` is `true`, the error message otherwise.
    pub content: String,
}

/// Minifies the given CSS code using the `minifier` crate and converts the returned value
/// to a `MinificationResult`.
#[wasm_bindgen]
pub fn minify_css(code: &str) -> MinificationResult {
    let minified = minify(code);
    match minified {
        Ok(val) => MinificationResult {
            success: true,
            content: val,
        },
        Err(err) => MinificationResult {
            success: false,
            content: err.to_string(),
        },
    }
}
