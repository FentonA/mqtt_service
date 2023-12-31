#[cfg(feature ="ch04")]
#[derive(Debug, Clone)]
pub struct MqttClientConfig {
    pub mqtt_service: String, 
    pub mqtt_port: u16,
}

#[cfg(feature = "ch04")]
impl MqttClientConfig {
    pub fn server_name(&self) ->String {
        match sys_info::hostname(){
            Ok(name) =>{
                name
            },
            Err(error) =>{
                "server".to_string()
            }
        }
    }
}

use rumqtt::{Receiver, Notification, Notification::Publish};
// use rumqtt::Notification::{Publish};
use std::{thread, str};
use log::{debug, error};

#[cfg(feature = "ch04")]
pub fn monitor_notifications(notifications: Receriver<Notification>, url: String, f: fn(&str, String, String)) {
    thread::spawn(move || {
        for notification in notifications {
            debug!("Notification {:?}", notification);
            match notification {
                Publish(p) => {
                    let pl = String::from_utf8(p.payload.to_vec());
                    debug!("The Payload :: PKID: {:?}, QOS: {:?}, Duplicate: {:?} ", p.pkid, p.qs, p.dup);
                    match pl {
                        Ok(Payload) => f(url.as_str(), p.topic_name, payload),
                        Err(error) => {
                            error!("Error sending the payload: {:?}", error)
                        },
                    };
                },
                _ => debug!("n/a")
            }
        }
    })
}


