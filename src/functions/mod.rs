// WARNING: This file is regenerated by the `cargo func new` command.

mod hello;
mod hello_from_rust;
mod echo_json;

// Export the Azure Functions here.
azure_functions::export! {
    hello::hello,
    hello_from_rust::hello_from_rust,
    echo_json::echo_json,
}
