#![no_std]

extern crate alloc;

use alloc::format;
use core::panic::PanicInfo;
use wit_bindgen::rt::string::String;

use widget::widget::logging;

wit_bindgen::generate!({
    path: "../wg_display_widget_wit/wit",
    world: "widget"
});

const WIDGET_NAME: &str = "ESP32 Runtime Test Widget";

#[global_allocator]
static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

struct MyWidget;

impl Guest for MyWidget {
    fn get_name() -> String {
        WIDGET_NAME.into()
    }

    fn run(context: WidgetContext) -> WidgetResult {
        logging::log(logging::Level::Info, WIDGET_NAME, "run() invoked");

        WidgetResult {
            data: format!(
                "TEST_OK: run called, last_invocation={}s, config='{}'",
                context.last_invocation.seconds,
                context.config
            ),
        }
    }

    fn get_config_schema() -> String {
        "{\"type\":\"object\",\"properties\":{},\"additionalProperties\":false}".into()
    }

    fn get_version() -> String {
        "0.0.1-test".into()
    }

    fn get_run_update_cycle_seconds() -> u32 {
        5
    }
}

export!(MyWidget);
