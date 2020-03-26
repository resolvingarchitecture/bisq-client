extern crate log;
extern crate simple_logger;

use log::{trace,info};
use bisq_client::BisqClient;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting Bisq Client Daemon...");
    let mut bisq_client = BisqClient::new();
    bisq_client.init();
    trace!("Bisq Client Daemon Stopped.");
}
