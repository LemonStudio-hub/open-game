use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Request, RequestInit, RequestMode, Response};

pub async fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let window = web_sys::window().ok_or("No window")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Response is not a Response")?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let array_buffer = JsFuture::from(
        resp.array_buffer()
            .map_err(|e| format!("Failed to get array buffer: {:?}", e))?,
    )
    .await
    .map_err(|e| format!("Failed to read array buffer: {:?}", e))?;

    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    let mut bytes = vec![0u8; uint8_array.length() as usize];
    uint8_array.copy_to(&mut bytes);

    Ok(bytes)
}

pub async fn load_image(url: &str) -> Result<HtmlImageElement, String> {
    let img = HtmlImageElement::new().map_err(|e| format!("Failed to create image: {:?}", e))?;

    JsFuture::from(js_sys::Promise::new(&mut |resolve, reject| {
        let img_clone = img.clone();
        let onload = Closure::once(move || {
            resolve.call0(&JsValue::UNDEFINED).ok();
        });
        img_clone.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        let onerror = Closure::once(move || {
            let err = JsValue::from_str("Image load failed");
            reject.call1(&JsValue::UNDEFINED, &err).ok();
        });
        img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        img.set_src(url);
    }))
    .await
    .map_err(|e| format!("Image load failed: {:?}", e))?;

    Ok(img)
}
