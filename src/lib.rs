mod app;
mod file;
mod https;
use anyhow::Result;

#[cfg(target_os = "android")]
use wry::android_binding;
/*
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
        window::WindowBuilder,
    },
    http::Response,
    webview::{WebView, WebViewBuilder},
};
*/

#[cfg(target_os = "android")]
fn init_logging() {
   android_logger::init_once(
      android_logger::Config::default()
         .with_min_level(log::Level::Trace)
         .with_tag("etude-androidapp-rust"),
   );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
   env_logger::init();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
   match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
      Ok(t) => t,
      Err(err) => {
         eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
         std::process::abort()
      }
   }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn _start_app() {
   stop_unwind(|| main().unwrap());
}

#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
   #[cfg(target_os = "android")]
   android_binding!(com_example, etude_androidapp_rust, _start_app);
   #[cfg(target_os = "ios")]
   log::debug!("start_app");
   _start_app()
}

pub fn main() -> Result<()> {
   log::debug!("main");
   init_logging();

   // Right now we're going through dioxus-desktop but we'd like to go through dioxus-mobile
   // That will seed the index.html with some fixes that prevent the page from scrolling/zooming etc
   dioxus_desktop::launch_cfg(
        app::app,
        // Note that we have to disable the viewport goofiness of the browser.
        // Dioxus_mobile should do this for us
        dioxus_desktop::Config::default().with_custom_index(r#"<!DOCTYPE html>
        <html>
          <head>
            <title>Dioxus app</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no" />
            <!-- CUSTOM HEAD -->
          </head>
          <body>
            <div id="main"></div>
            <!-- MODULE LOADER -->
          </body>
        </html>
       "#.into()),
    );

   Ok(())
}
