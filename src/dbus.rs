use crate::messages::{Message, Update};
use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::stream;

use futures::StreamExt;
use futures_core::Stream;
use std::future::pending;
use zbus::connection;
use zbus::interface;
use zbus::{Connection, MatchRule, MessageStream};

pub fn subscription() -> Subscription<Message> {
    Subscription::run(dbus_worker)
}

struct DBusListen {
    output: mpsc::Sender<Message>,
}

#[interface(name = "org.cristal.Shell")]
impl DBusListen {
    async fn hey(&mut self) -> String {
        let _ = self.output.send(Message::Test).await;
        String::from("yay")
    }
    async fn toggle_surface(&mut self, surface: &str) -> String {
        let msg = match surface {
            "dashboard" => Message::Update(Update::ToggleDashBoard),
            _ => return String::from("unrecognised surface"),
        };
        let _ = self.output.send(msg).await;
        format!("toggle {} surface", surface)
    }
}

fn dbus_worker() -> impl Stream<Item = Message> {
    stream::channel(0, |mut output: mpsc::Sender<Message>| async move {
        let listener = DBusListen { output };

        let _conn = connection::Builder::session()
            .unwrap()
            .name("org.cristal.Shell")
            .unwrap()
            .serve_at("/org/cristal/Shell", listener)
            .unwrap()
            .build()
            .await
            .unwrap();

        println!("connection created");

        // Do other things or go to wait forever
        pending::<()>().await;
    })
}
