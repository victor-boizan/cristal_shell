use crate::messages::{Message, Update};
use iced::Subscription;

use futures::StreamExt;
use futures_core::Stream;
use zbus::{AsyncDrop, Connection, MatchRule, MessageStream, fdo::NameOwnerChanged};

pub fn subscription() -> Subscription<Message> {
    Subscription::run(stream).map(|_| {
        println!("message received");
        Message::Update(Update::ToggleDashBoard)
    })
}

fn stream() -> impl Stream<Item = std::result::Result<zbus::Message, zbus::Error>> {
    futures::stream::once(async {
        let conn = Connection::session().await.unwrap();
        let rule = MatchRule::builder()
            .msg_type(zbus::message::Type::Signal)
            //.sender("org.cristal.Shell")
            //.unwrap()
            .interface("org.cristal.Shell")
            .unwrap()
            .member("ToggleDashBoard")
            .unwrap()
            .build();
        return MessageStream::for_match_rule(
            rule,
            &conn,
            // For such a specific match rule, we don't need a big queue.
            Some(1),
        )
        .await
        .unwrap();
    })
    .flatten()
}
