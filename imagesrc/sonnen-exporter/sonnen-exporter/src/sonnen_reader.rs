use reqwest;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct SonnenReader<'a> {
    url: &'a str,
    status: SonnenStatus<'a>,
}

impl<'a> SonnenReader<'a> {
    pub fn status(url: &'a str) -> Result<SonnenStatus, Box<Error>> {
        let mut reader = SonnenReader {
            url,
            status: SonnenStatus::new(),
        };
        reader.main_status()?;
        //reader.inverter()?;
        reader.power_meter()?;
        reader.status.online = true;
        Ok(reader.status)
    }

    fn fetch_json(&self, suffix: &str) -> Result<Value, Box<Error>> {
        let url = self.url.to_owned() + suffix;
        let mut resp = reqwest::get(url.as_str())?;
        let text = resp.text()?;
        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    fn main_status(&mut self) -> Result<(), Box<Error>> {
        let json: Value = self.fetch_json("/api/v1/status")?;
        self.status.online = true;
        self.status.consumption_watts = json["Consumption_W"].as_i64().unwrap();
        self.status.inverter_ac_frequency = json["Fac"].as_i64().unwrap();
        self.status.grid_feed_in_watts = json["GridFeedIn_W"].as_i64().unwrap();
        self.status.discharge_watts = json["Pac_total_W"].as_i64().unwrap();
        self.status.production_watts = json["Production_W"].as_i64().unwrap();
        self.status.state_of_charge_percent = json["RSOC"].as_i64().unwrap();
        self.status.inverter_ac_volts = json["Uac"].as_i64().unwrap();
        self.status.battery_volts = json["Ubat"].as_i64().unwrap();
        Ok(())
    }

    fn inverter(&mut self) -> Result<(), Box<Error>> {
        let json: Value = self.fetch_json("/api/inverter")?;
        let status = json["status"].as_object().unwrap();
        self.status
            .temperatures
            .insert("bdc", to_f64(&status["tempbdc"]));
        self.status
            .temperatures
            .insert("hmi", to_f64(&status["temphmi"]));
        self.status
            .temperatures
            .insert("pu", to_f64(&status["temppu"]));
        self.status.grid_frequency = to_f64(&status["fac"]);
        self.status.grid_voltage[0] = to_f64(&status["uac1"]);
        self.status.grid_voltage[1] = to_f64(&status["uac2"]);
        self.status.grid_voltage[2] = to_f64(&status["uac3"]);
        Ok(())
    }

    fn power_meter(&mut self) -> Result<(), Box<Error>> {
        let json: Value = self.fetch_json("/api/powermeter")?;
        let json_prod = json["4_1"].as_object().unwrap();
        let json_cons = json["5_1"].as_object().unwrap();
        let mut current_prod = [0.0; 3];
        let mut current_cons = [0.0; 3];
        let mut volts_prod = [0.0; 3];
        let mut volts_cons = [0.0; 3];
        current_prod[0] = to_f64(&json_prod["a_l1"]);
        current_prod[1] = to_f64(&json_prod["a_l2"]);
        current_prod[2] = to_f64(&json_prod["a_l3"]);
        current_cons[0] = to_f64(&json_cons["a_l1"]);
        current_cons[1] = to_f64(&json_cons["a_l2"]);
        current_cons[2] = to_f64(&json_cons["a_l3"]);
        volts_prod[0] = to_f64(&json_prod["v_l1_n"]);
        volts_prod[1] = to_f64(&json_prod["v_l2_n"]);
        volts_prod[2] = to_f64(&json_prod["v_l3_n"]);
        volts_cons[0] = to_f64(&json_cons["v_l1_n"]);
        volts_cons[1] = to_f64(&json_cons["v_l2_n"]);
        volts_cons[2] = to_f64(&json_cons["v_l3_n"]);
        self.status.current.insert("production", current_prod);
        self.status.current.insert("consumption", current_cons);
        self.status.volts.insert("production", volts_prod);
        self.status.volts.insert("consumption", volts_cons);
        Ok(())
    }
}

fn to_f64(val: &Value) -> f64 {
    let str_val = val.to_string();
    let trimmed = str_val.trim_matches('"');
    return trimmed.parse::<f64>().unwrap();
}

#[derive(Debug)]
pub struct SonnenStatus<'a> {
    pub online: bool,
    pub consumption_watts: i64,
    pub inverter_ac_frequency: i64,
    pub grid_feed_in_watts: i64,
    pub discharge_watts: i64,
    pub production_watts: i64,
    pub state_of_charge_percent: i64,
    pub inverter_ac_volts: i64,
    pub battery_volts: i64,
    pub temperatures: HashMap<&'a str, f64>,
    pub grid_frequency: f64,
    pub grid_voltage: [f64; 3],
    pub current: HashMap<&'a str, [f64; 3]>,
    pub volts: HashMap<&'a str, [f64; 3]>,
}

impl<'a> SonnenStatus<'a> {
    pub fn new() -> SonnenStatus<'a> {
        SonnenStatus {
            online: false,
            consumption_watts: 0,
            inverter_ac_frequency: 0,
            grid_feed_in_watts: 0,
            discharge_watts: 0,
            production_watts: 0,
            state_of_charge_percent: 0,
            inverter_ac_volts: 0,
            battery_volts: 0,
            temperatures: HashMap::new(),
            grid_frequency: 0.0,
            grid_voltage: [0.0; 3],
            current: HashMap::new(),
            volts: HashMap::new(),
        }
    }
}
