use axum::{response::IntoResponse, routing::get, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use image::{codecs::png::PngDecoder, AnimationDecoder, ImageFormat};
use once_cell::sync::Lazy;
use std::io::Cursor;

static IMAGE: Lazy<Vec<&'static [u8]>> = Lazy::new(|| {
    PngDecoder::new(Cursor::new(include_bytes!("image.png")))
        .unwrap()
        .apng()
        .into_frames()
        .map(|frame| {
            let mut output = Vec::new();
            frame
                .unwrap()
                .into_buffer()
                .write_to(&mut Cursor::new(&mut output), ImageFormat::Png)
                .unwrap();
            &*output.leak()
        })
        .collect()
});

async fn serve(jar: CookieJar) -> impl IntoResponse {
    let mut index = jar
        .get("frame")
        .and_then(|frame| frame.value().parse().ok())
        .unwrap_or(0);
    if index >= IMAGE.len() {
        index = 0;
    }
    (
        jar.add(Cookie::new("frame", (index + 1).to_string())),
        [(axum::http::header::CONTENT_TYPE, "image/png")],
        IMAGE[index],
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(serve));

    Ok(router.into())
}
