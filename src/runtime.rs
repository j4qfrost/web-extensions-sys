use crate::{EventTarget, Port};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Runtime;

    // Methods

    #[wasm_bindgen(method)]
    pub fn connect(this: &Runtime, extension_id: Option<&str>, connect_info: &Object) -> Port;

    #[wasm_bindgen(method, js_name = connectNative)]
    pub fn connect_native(this: &Runtime, application: &str) -> Port;

    #[wasm_bindgen(catch, method, js_name = sendMessage)]
    pub async fn send_message(
        this: &Runtime,
        extension_id: Option<&str>,
        message: &JsValue,
        options: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = sendNativeMessage)]
    pub async fn send_native_message(
        this: &Runtime,
        application: &str,
        message: &Object,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = getURL)]
    pub fn get_url(this: &Runtime, path: &str) -> String;

    #[wasm_bindgen(method, js_name = setUninstallURL)]
    pub fn set_uninstall_url(this: &Runtime, url: &str);

    #[wasm_bindgen(method, js_name = openOptionsPage)]
    pub fn open_options_page(this: &Runtime);

    // Events

    #[wasm_bindgen(method, getter, js_name = onMessage)]
    pub fn on_message(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onMessageExternal)]
    pub fn on_message_external(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onConnect)]
    pub fn on_connect(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onConnectExternal)]
    pub fn on_connect_external(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onConnectNative)]
    pub fn on_connect_native(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onInstalled)]
    pub fn on_installed(this: &Runtime) -> EventTarget;

    // Deprecated

    #[deprecated]
    #[wasm_bindgen(method, getter, js_name = lastError)]
    pub fn last_error(this: &Runtime) -> Option<js_sys::Error>;
}
