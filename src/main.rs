use std::time::Duration;

use axum::{routing::get, Router};
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector;
use rand::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Metrix");
    // Define prometheus exporter

    let exporter = PrometheusBuilder::new();

    let handle = exporter
        .install_recorder()
        .expect("Failed to build prometheus exporter");

    let collector = Collector::default();
    collector.describe();

    tokio::spawn(emitter());

    let app = Router::new().route(
        "/metrics",
        get(move || {
            // Collect information just before handling '/metrics'
            collector.collect();
            std::future::ready(handle.render())
        }),
    );
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn emitter() {
    loop {
        let cpu_freq = gauge!("cpu_freq_mhz");

        let randomnumber: f64 = rand::thread_rng().gen();
        let cpu_mhz = 5000_f64 * randomnumber; // Placeholder value in MHz
        cpu_freq.set(cpu_mhz);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
