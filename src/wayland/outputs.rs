use super::WaylandState;
use wayland_client::{Connection, Dispatch, QueueHandle, protocol::wl_output};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Output {
    pub output: wl_output::WlOutput,
    pub name: String,
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
            _ => {
                println!("output event: {:?}", event);
            }
        }
    }
}
