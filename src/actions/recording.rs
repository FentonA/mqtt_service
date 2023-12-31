use crate::mqtt::middleware::MqttRequestExt;
use crate::mqtt::client::publish;
use rumqtt::QoS;
use log::{info, warn};

#[derive(Serialize, Deserialize, Debug)]
enum RecordingType{
    Start,
    Stop
}

#[derive(Serialize, Deserialize, Debug)]
struct Recording{
    uuid: Uuid,
    rtype: RecordingType
}


pub fn run (req: &mut Request) -> IronResult<Response> {
    info!("Recording Start/Stop");

    let recording = Recording {
        uuid: super::get_uuid_value(req, "id"),
        rtype: get_recording_type(req)
    };

    info!("Set Recording type to : {:?}",recording);

    //Sends data over
    let (mut client, _) = req.mqtt_client();
    let topic = format!("recording/{}", recording.uuid);
    let json = serde_json::to_string(&recording).unwrap();

    publish(&mut client, topic.as_str(), json, QoS::AtLeastOnce);

    Ok(Response::with((status::Ok, "Ok")))
}

fn get_recording_type(req: &Request) -> RecordingType{
    match super::get_value(req, "type").as_ref(){
        "start" => RecordingType::Start,
        "stop" => RecodringType::Stop,
        _ => panic!("You have bad code")
    }
}
