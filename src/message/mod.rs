use std::borrow::Cow;

use serde::Serialize;
use serde_json::Value;

use crate::notification::Notification;

#[cfg(test)]
mod tests;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Normal,
    High,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MessageBody<'a> {
    validate_only: bool,
    message: Message<'a>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Message<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_available: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    delay_while_idle: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Notification<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    registration_ids: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_live: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mutable_content: Option<bool>,
}

/// Represents a FCM message. Construct the FCM message
/// using various utility methods and finally send it.
/// # Examples:
/// ```rust
/// use fcm::FCMRequestBuilder;
///
/// let mut builder = FCMRequestBuilder::new("<FCM API Key>", "<project>", "<registration id>", Some(true));
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct FCMRequest<'a> {
    pub api_key: &'a str,
    pub project: &'a str,
    pub body: MessageBody<'a>,
}

#[derive(Debug)]
pub struct MessageBuilder<'a> {
    collapse_key: Option<&'a str>,
    content_available: Option<bool>,
    data: Option<Value>,
    delay_while_idle: Option<bool>,
    notification: Option<Notification<'a>>,
    priority: Option<Priority>,
    registration_ids: Option<Vec<Cow<'a, str>>>,
    restricted_package_name: Option<&'a str>,
    time_to_live: Option<i32>,
    topic: Option<&'a str>,
    mutable_content: Option<bool>,
}

///
/// A builder to get a `FCMRequest` instance. If the validate_only parameter is not defined, it will default to false (i.e. the message will actually be sent)
///
/// # Examples
///
/// ```rust
/// use fcm::FCMRequestBuilder;
///
/// let mut builder = FCMRequestBuilder::new("<FCM API Key>", "<project>", "<registration id>", None);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct FCMRequestBuilder<'a> {
    api_key: &'a str,
    project: &'a str,
    validate_only: Option<bool>,
    message: MessageBuilder<'a>,
}

impl<'a> FCMRequestBuilder<'a> {
    /// Get a new instance of FCMRequest. You need to supply topic.
    pub fn new(api_key: &'a str, project: &'a str, topic: &'a str, validate_only: Option<bool>) -> Self {
        FCMRequestBuilder {
            api_key,
            project,
            validate_only,
            message: MessageBuilder {
                topic: Some(topic),
                registration_ids: None,
                collapse_key: None,
                priority: None,
                content_available: None,
                delay_while_idle: None,
                time_to_live: None,
                restricted_package_name: None,
                data: None,
                notification: None,
                mutable_content: None,
            },
        }
    }

    /// Get a new instance of FCMRequest. You need to supply registration ids.
    pub fn new_multi<S>(api_key: &'a str, project: &'a str, ids: &'a [S], validate_only: Option<bool>) -> Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = ids.iter().map(|a| a.as_ref().into()).collect();

        FCMRequestBuilder {
            api_key,
            project,
            validate_only,
            message: MessageBuilder {
                topic: None,
                registration_ids: Some(converted),
                collapse_key: None,
                priority: None,
                content_available: None,
                delay_while_idle: None,
                time_to_live: None,
                restricted_package_name: None,
                data: None,
                notification: None,
                mutable_content: None,
            },
        }
    }

    /// String value to replace format specifiers in the body string.
    pub fn registration_ids<S>(&mut self, ids: &'a [S]) -> &mut Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = ids.iter().map(|a| a.as_ref().into()).collect();

        self.message.registration_ids = Some(converted);
        self
    }

    /// Set this parameter to identify groups of messages that can be collapsed.
    pub fn collapse_key(&mut self, collapse_key: &'a str) -> &mut Self {
        self.message.collapse_key = Some(collapse_key);
        self
    }

    /// Set the priority of the message. You can set Normal or High priorities.
    /// # Examples:
    /// ```rust
    /// use fcm::{FCMRequestBuilder, Priority};
    ///
    /// let mut builder = FCMRequestBuilder::new("<FCM API Key>", "<project>", "<registration id>", None);
    /// builder.priority(Priority::High);
    /// let message = builder.finalize();
    /// ```
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.message.priority = Some(priority);
        self
    }

    /// To set the `content-available` field on iOS
    pub fn content_available(&mut self, content_available: bool) -> &mut Self {
        self.message.content_available = Some(content_available);
        self
    }

    /// When set to `true`, sends the message only when the device is active.
    pub fn delay_while_idle(&mut self, delay_while_idle: bool) -> &mut Self {
        self.message.delay_while_idle = Some(delay_while_idle);
        self
    }

    /// How long (in seconds) to keep the message on FCM servers in case the device
    /// is offline. The maximum and default is 4 weeks.
    pub fn time_to_live(&mut self, time_to_live: i32) -> &mut Self {
        self.message.time_to_live = Some(time_to_live);
        self
    }

    /// Package name of the application where the registration tokens must match.
    pub fn restricted_package_name(&mut self, restricted_package_name: &'a str) -> &mut Self {
        self.message.restricted_package_name = Some(restricted_package_name);
        self
    }

    /// Use this to add custom key-value pairs to the message. This data
    /// must be handled appropriately on the client end. The data can be
    /// anything that Serde can serialize to JSON.
    ///
    /// # Examples:
    /// ```rust
    /// use fcm::FCMRequestBuilder;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let mut builder = FCMRequestBuilder::new("<FCM API Key>", "<project>", "<registration id>", None);
    /// builder.data(&map);
    /// let message = builder.finalize();
    /// ```
    pub fn data(&mut self, data: &dyn erased_serde::Serialize) -> Result<&mut Self, serde_json::Error> {
        self.message.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// Use this to set a `Notification` for the message.
    /// # Examples:
    /// ```rust
    /// use fcm::{FCMRequestBuilder, NotificationBuilder};
    ///
    /// let mut builder = NotificationBuilder::new();
    /// builder.title("Hey!");
    /// builder.body("Do you want to catch up later?");
    /// let notification = builder.finalize();
    ///
    /// let mut builder = FCMRequestBuilder::new("<FCM API Key>", "<project>", "<registration id>", None);
    /// builder.notification(notification);
    /// let message = builder.finalize();
    /// ```
    pub fn notification(&mut self, notification: Notification<'a>) -> &mut Self {
        self.message.notification = Some(notification);
        self
    }

    /// To set the `mutable_content` field on iOS
    pub fn mutable_content(&mut self, mutable_content: bool) -> &mut Self {
        self.message.mutable_content = Some(mutable_content);
        self
    }

    /// Complete the build and get a `FCMRequest` instance
    pub fn finalize(self) -> FCMRequest<'a> {
        FCMRequest {
            api_key: self.api_key,
            project: self.project,
            body: MessageBody {
                message: Message {
                    topic: self.message.topic,
                    registration_ids: self.message.registration_ids,
                    collapse_key: self.message.collapse_key,
                    priority: self.message.priority,
                    content_available: self.message.content_available,
                    delay_while_idle: self.message.delay_while_idle,
                    time_to_live: self.message.time_to_live,
                    restricted_package_name: self.message.restricted_package_name,
                    data: self.message.data.clone(),
                    notification: self.message.notification,
                    mutable_content: self.message.mutable_content,
                },
                validate_only: self.validate_only.unwrap_or(false),
            },
        }
    }
}
