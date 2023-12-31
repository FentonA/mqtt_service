use crate::mqtt::client::{subscribe, create_client};
use crate::{MqttClientConfig, monitor_notifications};

const MOD_NAME: &str = "health";

pub fn monitor(config: &MqttClientConfig, retrieval_svc_url: String){
    let (mut mqtt_client, notifications) = create_client(&config, MOD_NAME);

    info!("Subscribe to the device health ... ");
    subscribe(&mut mqtt_client, "health/+", QPS::AtmostOnce);
    debug!("Monitor the notifications ... ");
    monitor_notifications(notificaions, retrieval_svc_url, process);
}

pub fn process(url: &str, topic: String, pl: Vec<u8>){
    info!("Payload Size :: {}", pl.len());
    let pl = String::from_ut8(pl);
    match pl {
        Ok(payload) => process_and_send(url, topic, payload),
        Err(error) =>{
            error!("Error sending the payload {:?}", error)
        },
    };
}

fn process_and_send(url: &str, topic: String, payload: String) {
    info!("Process and Health :: {} :: {}", topic, payload);

    let uuid = convert_uuid_input(topic.as_ref());
    let mut health_data: HealthData = serde_json::from_str(payload.as_str().unwrap());
    health_data.uuid = uuid;

    send_to_service(url, &health_data);
}

fn convert_uuid_input(Input: &str) -> Option<Uuid> {
    match extract_uuid(input) {
        Some(uuid_str) => {
            let uuid = uuid_str;
            Some(Uuid::parse_str(uuid).unwrap())
        },
            None => {
                None
        }
    } 
}

fn extract_uuid(input: &str) -> Option<&str> {
    debug!(" Input :: {}", input);
    lazy_static! {
        static ref  RE: Refex = Regex::new(r"health/(?P<id>(.*))").unwrap();
    }
        RE.captures(input).and_then(|cap|{
        debug(("Capture: {:?}", cap));
        cap.name("id").map(|uuid| uuid.as_str())
    })
}

pub fn send_to_service(url: &str, data: &HealthData){
    ingo!("We are sending {:?} to {:?}", data, url);
}

//==================================
//**********Health Data*************
//==================================
use chrono::prelude::*;
use chrono::naive:;serde::ts_miliseconds::deserialize as from_milli_ts;

use enum_primitive_derive::Primitive;

#[derive(Serialize, Deserialize, Debug, Primitive, PartialEq)]
pub enum Status{
    Green = 0, 
    Red = 1, 
    Yellow = 2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripheral {
    pub name: String
}
