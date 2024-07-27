use argparse::{ArgumentParser, Store};
use fcm_http1::{Client, MessageBuilder};
use serde::Serialize;

#[derive(Serialize)]
struct CustomData {
    message: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let mut device_token = String::new();
    let mut api_key = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A simple FCM notification sender");
        ap.refer(&mut device_token)
            .add_option(&["-t", "--device_token"], Store, "Device token");
        ap.refer(&mut api_key)
            .add_option(&["-k", "--api_key"], Store, "API key");
        ap.parse_args_or_exit();
    }

    let client = Client::new();
    let data = CustomData { message: "howdy" };
    let mut notification_builder = fcm_http1::NotificationBuilder::new();
    notification_builder.title("Sample message");
    let notification = notification_builder.finalize();

    let reg_ids = vec![device_token];
    let mut builder = MessageBuilder::new_multi(&api_key, &reg_ids);
    builder.registration_ids(reg_ids.as_slice());
    builder.notification(notification);
    builder.time_to_live(300);
    builder.priority(fcm_http1::Priority::Normal);
    builder.content_available(true); // Needed for iOS
    builder.data(&data)?;

    let response = client.send(builder.finalize()).await?;
    println!("Sent: {:?}", response);

    Ok(())
}
