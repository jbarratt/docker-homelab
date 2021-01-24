use actix_web::{http::ContentEncoding, HttpRequest, HttpResponse};
use bytes::Bytes;
use config::System;
use prometheus;
use prometheus::{Encoder, GaugeVec, IntGaugeVec, TextEncoder};
use sonnen_reader::SonnenReader;

use GIT_REVISION;
use RUST_VERSION;
use VERSION;

lazy_static! {
    static ref BUILD_INFO: GaugeVec = register_gauge_vec!(
        "sonnen_build_info",
        "A metric with a constant '1' value labeled by version, revision and rustversion",
        &["version", "revision", "rustversion"]
    ).unwrap();
    static ref ONLINE: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_online",
        "Metric scraping successful",
        &["host", "sn"]
    ).unwrap();
    static ref CONSUMPTION_WATTS: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_consumption_watts",
        "Number of watts being consumed",
        &["host", "sn"]
    ).unwrap();
    static ref INVERTER_AC_FREQ: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_frequency_inverter",
        "Frequency of AC inverter",
        &["host", "sn"]
    ).unwrap();
    static ref FEED_IN_WATTS: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_grid_feed_in_watts",
        "Number of watts being fed into the grid (negative denotes grid purchase)",
        &["host", "sn"]
    ).unwrap();
    static ref DISCHARGE_WATTS: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_discharge_watts",
        "Number of watts being discharged by the battery (negative means battery charging)",
        &["host", "sn"]
    ).unwrap();
    static ref PRODUCTION_WATTS: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_production_watts",
        "Number of watts being produced by solar",
        &["host", "sn"]
    ).unwrap();
    static ref SOC: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_state_of_charge_percent",
        "Percent charged of the battery",
        &["host", "sn"]
    ).unwrap();
    static ref INVERTER_VOLTS: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_inverter_ac_volts",
        "Number of volts produced by the inverter",
        &["host", "sn"]
    ).unwrap();
    static ref BATTERY_VOLTS: IntGaugeVec = register_int_gauge_vec!(
        "sonnen_battery_volts",
        "Number of volts produced by the battery",
        &["host", "sn"]
    ).unwrap();
    static ref TEMPERATURE: GaugeVec = register_gauge_vec!(
        "sonnen_temperature",
        "Temperature of different probes in Celcius",
        &["host", "sn", "probe"]
    ).unwrap();
    static ref GRID_FREQUENCY: GaugeVec = register_gauge_vec!(
        "sonnen_frequency_grid",
        "Frequency of grid",
        &["host", "sn"]
    ).unwrap();
    static ref GRID_VOLTAGE: GaugeVec = register_gauge_vec!(
        "sonnen_grid_volts",
        "Number of volts of each grid phase",
        &["host", "sn", "phase"]
    ).unwrap();
    static ref METER_CURRENT: GaugeVec = register_gauge_vec!(
        "sonnen_meter_current",
        "Number of amps of each power meter phase",
        &["host", "sn", "direction", "phase"]
    ).unwrap();
    static ref METER_VOLTS: GaugeVec = register_gauge_vec!(
        "sonnen_meter_volts",
        "Number of volts of each power meter phase",
        &["host", "sn", "direction", "phase"]
    ).unwrap();
}

static LANDING_PAGE: &'static str = "<html>
<head><title>Sonnen Exporter</title></head>
<body>
<h1>Sonnen Exporter</h1>
<p><a href=\"/metrics\">Metrics</a></p>
</body>
";

pub fn index(_req: &HttpRequest<Vec<System>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Auto)
        .content_type("text/html")
        .body(LANDING_PAGE)
}

pub fn metrics(req: &HttpRequest<Vec<System>>) -> HttpResponse {
    for sys in req.state().clone() {
        let host = sys.host.unwrap_or(String::from(""));
        let url = sys.url.unwrap_or(String::from(""));
        let sn = sys.sn.unwrap_or(String::from(""));
        let status = match SonnenReader::status(&url) {
            Ok(x) => x,
            Err(x) => {
                ONLINE.with_label_values(&[&host, &sn]).set(0);
                println!("{} failed: {}", host, x);
                continue;
            }
        };
        ONLINE
            .with_label_values(&[&host, &sn])
            .set(if status.online { 1 } else { 0 });
        CONSUMPTION_WATTS
            .with_label_values(&[&host, &sn])
            .set(status.consumption_watts);
        INVERTER_AC_FREQ
            .with_label_values(&[&host, &sn])
            .set(status.inverter_ac_frequency);
        FEED_IN_WATTS
            .with_label_values(&[&host, &sn])
            .set(status.grid_feed_in_watts);
        DISCHARGE_WATTS
            .with_label_values(&[&host, &sn])
            .set(status.discharge_watts);
        PRODUCTION_WATTS
            .with_label_values(&[&host, &sn])
            .set(status.production_watts);
        SOC.with_label_values(&[&host, &sn])
            .set(status.state_of_charge_percent);
        INVERTER_VOLTS
            .with_label_values(&[&host, &sn])
            .set(status.inverter_ac_volts);
        BATTERY_VOLTS
            .with_label_values(&[&host, &sn])
            .set(status.battery_volts);
        for (probe, measurement) in status.temperatures {
            TEMPERATURE
                .with_label_values(&[&host, &sn, &probe])
                .set(measurement);
        }
        GRID_FREQUENCY
            .with_label_values(&[&host, &sn])
            .set(status.grid_frequency);
        for (i, val) in status.grid_voltage.iter().enumerate() {
            let phase = (i + 1).to_string();
            GRID_VOLTAGE
                .with_label_values(&[&host, &sn, &phase])
                .set(*val);
        }
        for (direction, measurements) in status.current {
            for (i, val) in measurements.iter().enumerate() {
                let phase = (i + 1).to_string();
                METER_CURRENT
                    .with_label_values(&[&host, &sn, &direction, &phase])
                    .set(*val);
            }
        }
        for (direction, measurements) in status.volts {
            for (i, val) in measurements.iter().enumerate() {
                let phase = (i + 1).to_string();
                METER_VOLTS
                    .with_label_values(&[&host, &sn, &direction, &phase])
                    .set(*val);
            }
        }
    }

    let git_revision = GIT_REVISION.unwrap_or("");
    let rust_version = RUST_VERSION.unwrap_or("");
    BUILD_INFO
        .with_label_values(&[&VERSION, &git_revision, &rust_version])
        .set(1.0);

    let metric_families = prometheus::gather();
    let encoder = TextEncoder::new();
    let mut buffer: Vec<u8> = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Auto)
        .content_type(encoder.format_type())
        .body(Bytes::from(buffer))
}
