use std::sync::Arc;
use wayland_client::{
    protocol::{wl_output, wl_registry},
    Connection, Dispatch, QueueHandle,
};

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub struct Output {
   pub output: wl_output::WlOutput,
   pub name: String,
} 
struct WaylandState {
    outputs: Vec<Output>,
}

impl Output {
    pub fn get_all(conn: Arc<Connection>) -> Vec<Self>{
        let display = conn.display();

        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, ());

        let mut state = WaylandState {
            outputs: Vec::new(),
        };

        let _ = event_queue.roundtrip(&mut state);

        // Do another roundtrip to get output names
        let _ = event_queue.roundtrip(&mut state);
        state.outputs
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
        if let wl_registry::Event::Global { name, interface, version } = event {
            if interface == "wl_output" {
                let output = registry.bind::<wl_output::WlOutput, _, _>(name, version, qh, ());
                state.outputs.push(Output {
                    output,
                    name: format!("output-{}", name), // Temporary name, will be updated
                });
            }
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for WaylandState {
    fn event(
        state: &mut Self,
        output: &wl_output::WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        // Handle output events (geometry, mode, etc.)
        match event {
            wl_output::Event::Name { name } => {
                // Update the output name
                if let Some(out) = state.outputs.iter_mut().find(|o| &o.output == output) {
                    out.name = name;
                }
            }
            _ => {}
        }
    }
}
