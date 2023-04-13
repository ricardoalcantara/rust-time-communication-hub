use super::dto::message::NofityMessage;
use crate::{
    domain::dto::error::{HttpError, HttpResult},
    hub::hub_connection::HubConnection,
};

pub struct MessageService {
    hub_connection: HubConnection,
}

impl MessageService {
    pub fn new(hub_connection: HubConnection) -> MessageService {
        MessageService { hub_connection }
    }

    pub async fn notify(self, notify_message: NofityMessage) -> HttpResult {
        let payload = notify_message.payload;
        if notify_message.users.is_none() && notify_message.groups.is_none() {
            return Err(HttpError::bad_request("Destiny is required"));
        }
        if let Some(users) = notify_message.users {
            for user in users {
                self.hub_connection.notify_user(user, payload.clone()).await;
            }
        }

        if let Some(groups) = notify_message.groups {
            for group in groups {
                self.hub_connection
                    .notify_group(group, payload.clone())
                    .await;
            }
        }

        Ok(())
    }
}
