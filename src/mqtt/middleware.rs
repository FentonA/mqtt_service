use super::MqttClientConfig;
use super::client::create_client;

pub struct MqttClientMiddleware{
    config: MqttClientConfig,
    client: Option<MqttClient>
}

impl MqttClientMiddleware{
    pub fn new(config: MqttClientConfig) -> MqttClientMiddleware {
        MqttClientMiddleware {
            conig: config,
            client: None
        }
    }
}

pub struct Value(MqttClientConfig, Option<MqttClient>);

impl typemap::key for MqttClientMiddleware{type Value = Value;}

impl BeforeMiddleware for MqttClientMiddleware {
    fn before(&self, req:&mut Request) -> IronResult<()> {
        req.extensions.insert::<MqttClientMiddleware>(Value(self.config.clone(), None));
        Ok(())
    }
}


//TODO: Implement AfterMiddleware 
//page 168

pub trait MqttRequestExt {
    fn mqtt_client(&mut self) -> (MqttClient, Receiver<Notification>);
}

impl<'a, 'b> MqttRequestExt for Request<'a, 'b> {
    fn mqtt_client(&mut self) -> (MqttClient, Receiver<Notification>) {
        debug!("Get Client Request");
        let config_val = self.extensions.get::<MqttClientMiddleware>().
        expect("Mqtt Client");
        let Value(ref config, _) = config_val;

        let (client, note) = create_client(&config, random().as_str());
        self.extensions.insert::<MqttClientMiddleware>(Value(config.to_owned(), Some(client.clone())));
        return (client, note)
    }
}


fn random() -> String{

    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    thread_rang()
        .sample_iter(&Alphanumeric)
        .take(5)
        .collect()
}
