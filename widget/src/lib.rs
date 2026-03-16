#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::panic::PanicInfo;
use schemars::{schema_for, JsonSchema};
use serde::Deserialize;

use widget::widget::{clocks, http, logging, random};

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

#[derive(JsonSchema, Deserialize)]
struct WidgetConfig {
    test: String,
}

struct MyWidget;

impl Guest for MyWidget {
    fn get_name() -> String {
        WIDGET_NAME.into()
    }

    fn run(context: WidgetContext) -> WidgetResult {
        logging::log(logging::Level::Info, "TEST", "=== Starting WIT Interface Tests ===");
        
        // Log context information
        logging::log(
            logging::Level::Info,
            "TEST",
            &format!(
                "Context: last_invocation={}s, config='{}'",
                context.last_invocation.seconds, context.config
            )
        );
        
        // Test 1: Logging (all levels)
        logging::log(logging::Level::Debug, "TEST_LOG", "Testing DEBUG level");
        logging::log(logging::Level::Info, "TEST_LOG", "Testing INFO level");
        logging::log(logging::Level::Warn, "TEST_LOG", "Testing WARN level");
        logging::log(logging::Level::Error, "TEST_LOG", "Testing ERROR level");
        logging::log(logging::Level::Info, "TEST", "✓ Logging: All levels tested");
        
        // Test 2: Clocks Interface
        logging::log(logging::Level::Info, "TEST", "Testing Clocks interface...");
        let now = clocks::now();
        logging::log(
            logging::Level::Info,
            "TEST",
            &format!("✓ Clocks: now={}s + {}ns", now.seconds, now.nanoseconds)
        );
        
        // Test 3: Random Interface
        logging::log(logging::Level::Info, "TEST", "Testing Random interface...");
        let rand1 = random::get_random();
        let rand2 = random::get_random();
        let rand3 = random::get_random();
        let all_unique = rand1 != rand2 && rand2 != rand3 && rand1 != rand3;
        logging::log(
            logging::Level::Info,
            "TEST",
            &format!("✓ Random: {} / {} / {} (unique={})", rand1, rand2, rand3, all_unique)
        );
        
        // Test 4: HTTP GET Interface
        logging::log(logging::Level::Info, "TEST", "Testing HTTP GET request...");
        match http::request(http::Method::Get, "http://httpbin.org/get", None) {
            Ok(response) => {
                logging::log(
                    logging::Level::Info,
                    "TEST",
                    &format!("✓ HTTP GET: status={}, bytes={}", response.status, response.bytes.len())
                );
            },
            Err(_) => {
                logging::log(logging::Level::Error, "TEST", "✗ HTTP GET: Failed");
            }
        }
        
        // Test 5: HTTP POST Interface
        logging::log(logging::Level::Info, "TEST", "Testing HTTP POST request...");
        let test_body = b"test_data_from_widget";
        match http::request(
            http::Method::Post,
            "http://httpbin.org/post",
            Some(test_body)
        ) {
            Ok(response) => {
                logging::log(
                    logging::Level::Info,
                    "TEST",
                    &format!("✓ HTTP POST: status={}, bytes={}", response.status, response.bytes.len())
                );
            },
            Err(_) => {
                logging::log(logging::Level::Error, "TEST", "✗ HTTP POST: Failed");
            }
        }
        
        logging::log(logging::Level::Info, "TEST", "=== Tests Complete ===");
        
        WidgetResult {
            data: format!("Interface tests complete - check logs for results").into(),
        }
    }

    fn get_config_schema() -> String  {
        let schema = schema_for!(WidgetConfig);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    fn get_version() -> String {
        "0.1.0-interface-test".into()
    }

    fn get_run_update_cycle_seconds() -> u32 {
        10
    }
}

export!(MyWidget);
