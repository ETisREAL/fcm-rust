use argparse::{ArgumentParser, Store};
use fcm_http1::{Client, FCMRequestBuilder};
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
    let mut project = String::new();
    let mut validate_only = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A simple FCM notification sender");
        ap.refer(&mut device_token)
            .add_option(&["-t", "--device_token"], Store, "Device token");
        ap.refer(&mut api_key)
            .add_option(&["-k", "--api_key"], Store, "API key");
        ap.refer(&mut project)
            .add_option(&["-p", "--project"], Store, "Project");
        ap.refer(&mut validate_only)
        .add_option(&["-d", "--validate-only"], Store, "Validate only (dry run)");
        ap.parse_args_or_exit();
    }

    let client = Client::new();
    let data = CustomData { message: "howdy" };
    let validate_only_bool = match validate_only.parse().unwrap_or(String::from("false")).as_str() {
        "true" => true,
        "false" => false,
        _ => false,
    };

    let mut builder = FCMRequestBuilder::new(&api_key, &project, &device_token, Some(validate_only_bool));
    builder.data(&data)?;

    let response = client.send(builder.finalize()).await?;
    println!("Sent: {:?}", response);

    Ok(())
}
