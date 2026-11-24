use std::collections::HashMap;

// Otus hw03 example application
// v 0.3.0
use smart_home::*;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
}
