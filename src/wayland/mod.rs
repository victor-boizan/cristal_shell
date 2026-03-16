pub mod outputs;
pub mod workspaces;

use crate::messages::{Message,Update};
use iced::Subscription;
use iced::stream;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::Stream;
use outputs::Output;
use std::collections::HashMap;
use std::sync::Arc;
use wayland_client::{
    protocol::{wl_output, wl_registry},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::ext::workspace::v1::client::ext_workspace_group_handle_v1;
use wayland_protocols::ext::workspace::v1::client::ext_workspace_handle_v1;
use wayland_protocols::ext::workspace::v1::client::ext_workspace_manager_v1;
use workspaces::{Workspace, WorkspaceGroup};
use std::sync::OnceLock;

static CONNECTION: OnceLock<Arc<Connection>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct WaylandState {
    pub outputs: Vec<Output>,
    workspace_groups:
        HashMap<ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1, WorkspaceGroup>,
    pub workspaces: HashMap<ext_workspace_handle_v1::ExtWorkspaceHandleV1, Workspace>,
}

impl WaylandState {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
            workspace_groups: HashMap::new(),
            workspaces: HashMap::new(),
        }
    }
    pub fn subscription(conn: Arc<Connection>) -> Subscription<Message> {
	CONNECTION.set(conn).ok();
	Subscription::run(Self::stream)
    }
    fn stream() -> impl Stream<Item = Message> {
	let conn = CONNECTION.get().unwrap().clone();
	stream::channel(0, |mut output: mpsc::Sender<Message>| async move {
	    let (_sender, mut _receiver) = mpsc::channel::<Message>(0);
            let display = conn.display();
	    let mut event_queue = conn.new_event_queue();
	    let qh = event_queue.handle();
            let _registry = display.get_registry(&qh, ());
	    let mut state = Self::new();
	    let _ = event_queue.roundtrip(&mut state);
            let _ = event_queue.roundtrip(&mut state);
	    let _ = output.send(Message::Update(Update::WaylandInit(state.clone()))).await;
	    loop {
		println!("event waiting");
		event_queue.blocking_dispatch(&mut state).unwrap();
		println!("event recieved");
		let _ = output.send(Message::Update(Update::WaylandUpdate(state.clone()))).await;
	    }
	})
    }
    pub fn workspaces_for_output(&self, output: Output) -> Vec<Workspace> {
        let wl_output = output.output;

        let mut workspaces = Vec::new();

        for (_group, data) in self.workspace_groups.clone() {
            if data.outputs.contains(&wl_output) {
                for handle in data.workspaces {
                    if let Some(ws) = self.workspaces.get(&handle) {
                        workspaces.push(ws.clone());
                    }
                }
            }
        }
        return workspaces;
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for WaylandState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == "wl_output" {
                let output = registry.bind::<wl_output::WlOutput, _, _>(name, version, qh, ());
                state.outputs.push(Output {
                    output,
                    name: format!("output-{}", name), // Temporary name, will be updated
                });
            } else if interface == "ext_workspace_manager_v1" {
                if let workspace = registry
                    .bind::<ext_workspace_manager_v1::ExtWorkspaceManagerV1, _, _>(
                        name,
                        version,
                        qh,
                        (),
                    )
                {
                } else {
                    eprintln!("error binding");
                }
            }
        }
    }
}
