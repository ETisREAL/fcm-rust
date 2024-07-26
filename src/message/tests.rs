use crate::notification::NotificationBuilder;
use crate::{FCMRequestBuilder, Priority};
use serde::Serialize;
use serde_json::json;
use std::borrow::Cow;

#[derive(Serialize)]
struct CustomData {
    foo: &'static str,
    bar: bool,
}

#[test]
fn should_create_new_message() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.topic, Some("token"));
}

#[test]
fn should_leave_nones_out_of_the_json() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();
    let payload = serde_json::to_string(&msg.body).unwrap();

    let expected_payload = json!({
        "validate_only": false,
        "message": {
            "topic": "token"
        }
    });

    let expected_value: serde_json::Value = serde_json::from_str(&expected_payload.to_string()).unwrap();
    let actual_value: serde_json::Value = serde_json::from_str(&payload).unwrap();

    assert_eq!(expected_value, actual_value);
}

#[test]
fn should_add_custom_data_to_the_payload() {
    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);

    let data = CustomData { foo: "bar", bar: false };

    builder.data(&data).unwrap();

    let msg = builder.finalize();
    let payload = serde_json::to_string(&msg.body).unwrap();

    let expected_payload = json!({
        "message": {
            "data": {
                "foo": "bar",
                "bar": false,
            },
            "topic": "token"
        },
        "validate_only": false
    });

    let expected_value: serde_json::Value = serde_json::from_str(&expected_payload.to_string()).unwrap();
    let actual_value: serde_json::Value = serde_json::from_str(&payload).unwrap();

    assert_eq!(expected_value, actual_value);
}

#[test]
fn should_be_able_to_render_a_full_message_to_json() {
    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);

    builder
        .registration_ids(&["one", "two"])
        .collapse_key("foo")
        .priority(Priority::High)
        .content_available(false)
        .delay_while_idle(true)
        .time_to_live(420)
        .restricted_package_name("pkg")
        .notification(NotificationBuilder::new().finalize());

    let payload = serde_json::to_string(&builder.finalize().body).unwrap();

    let expected_payload = json!({
        "message": {
            "topic": "token",
            "registration_ids": ["one", "two"],
            "collapse_key": "foo",
            "priority": "high",
            "content_available": false,
            "delay_while_idle": true,
            "time_to_live": 420,
            "restricted_package_name": "pkg",
            "notification": {},
        },
        "validate_only": false
    });

    let expected_value: serde_json::Value = serde_json::from_str(&expected_payload.to_string()).unwrap();
    let actual_value: serde_json::Value = serde_json::from_str(&payload).unwrap();

    assert_eq!(expected_value, actual_value);
}

#[test]
fn should_set_registration_ids() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.registration_ids, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.registration_ids(&["id1"]);
    let msg = builder.finalize();

    assert_eq!(msg.body.message.registration_ids, Some(vec![Cow::from("id1")]));
}

#[test]
fn should_set_collapse_key() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.collapse_key, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.collapse_key("key");
    let msg = builder.finalize();

    assert_eq!(msg.body.message.collapse_key, Some("key"));
}

#[test]
fn should_set_priority() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.priority, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.priority(Priority::Normal);
    let msg = builder.finalize();

    assert_eq!(msg.body.message.priority, Some(Priority::Normal));
}

#[test]
fn should_set_content_available() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.content_available, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.content_available(true);
    let msg = builder.finalize();

    assert_eq!(msg.body.message.content_available, Some(true));
}

#[test]
fn should_set_mutable_content() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.mutable_content, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.mutable_content(true);
    let msg = builder.finalize();

    assert_eq!(msg.body.message.mutable_content, Some(true));
}

#[test]
fn should_set_delay_while_idle() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.delay_while_idle, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.delay_while_idle(true);
    let msg = builder.finalize();

    assert_eq!(msg.body.message.delay_while_idle, Some(true));
}

#[test]
fn should_set_time_to_live() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.time_to_live, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.time_to_live(10);
    let msg = builder.finalize();

    assert_eq!(msg.body.message.time_to_live, Some(10));
}

#[test]
fn should_set_restricted_package_name() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.restricted_package_name, None);

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.restricted_package_name("name");
    let msg = builder.finalize();

    assert_eq!(msg.body.message.restricted_package_name, Some("name"));
}

#[test]
fn should_set_dry_run() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", Some(true)).finalize();

    assert_eq!(msg.body.validate_only, true);
}

#[test]
fn should_set_notifications() {
    let msg = FCMRequestBuilder::new("api_key", "project", "token", None).finalize();

    assert_eq!(msg.body.message.notification, None);

    let nm = NotificationBuilder::new().finalize();

    let mut builder = FCMRequestBuilder::new("api_key", "project", "token", None);
    builder.notification(nm);
    let msg = builder.finalize();

    assert!(msg.body.message.notification != None);
}
