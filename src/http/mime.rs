// MIME type utilities
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types


pub fn mime_from_ext(ext: &str, default: Option<&'static str>) -> &'static str {
    match ext {
        "txt" => "text/plain",

        "html" => "text/html",
        "css" => "text/css",
        "js" | "mjs" => "text/javascript",
        "wasm" => "application/wasm",

        "apng" => "image/apng",
        "avif" => "image/avif",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",

        "ogg" => "application/ogg",
        "wav" => "audio/wave",
        "webm" => "audio/webm", // Could also be "video/webm"

        _ => default.unwrap_or("")
    }
}
