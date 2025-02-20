use cairo_lang_macro::{inline_macro, ProcMacroResult, TokenStream};

#[inline_macro]
pub fn double(token_stream: TokenStream) -> ProcMacroResult {
    // Convert the TokenStream to a string and trim any whitespace
    let input = token_stream.to_string().trim().to_string();

    // Parse the input as an integer
    let num: i32 = match input.parse() {
        Ok(val) => val,
        Err(_) => {
            return ProcMacroResult::new(TokenStream::empty())
                .with_diagnostics(cairo_lang_macro::Diagnostics::new(vec![
                    cairo_lang_macro::Diagnostic::error("Input must be a valid integer".to_string()),
                ]));
        }
    };

    // Double the number
    let result = num * 2;

    // Convert the result back to a TokenStream
    ProcMacroResult::new(TokenStream::new(result.to_string()))
}