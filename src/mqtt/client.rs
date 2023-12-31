use mqtt::{MqttClient, MqttOptions, QoS, ReconnectionOptions, Notification, Reciever};
use super::MqttClientConfig;
const CLIENT_NAME: &str = "mqtt";

#[cfg(feauture = "ch04")]
fn create_client(config: MqttClientConfig, name: &str) ->  (MqttClient, Receiver<Notification>) {
    let client_id = format!("{}-{}-{}", CLIENT_NAME, name, config.server_name());

    debug!("Create the connection ... ");

    let reconnection_otptions = ReconnectOptions::Always(10);

    let mqtt_options = MqttOptions::new(client_id, config.mqtt_server.as_str(), config.mqtt_port)
                                        .set_keep_alive(10)
                                        .set_reconnect_opts(reconection_options)
                                        .set_clean_session(false);

    MqttClient::start(mqtt_options).expect()
}

pub fn subscribe(client: &mut MqttClient, topic: &'static str, qos: QOS){
    info!("Subscribe and prcoess: :  {:? }", topic);
    client.subscribe(topic, qos).unwrap();
}

pub fn publish(client: &mut MqttClient, topic: &str, payload: String, qos: QoS) {
    info!("Publish to the topic: : {:?}", topic);
    client.publish(topic, qos, false, payload).unwrap();
}
