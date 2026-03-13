use super::WaylandState;
use std::collections::HashMap;
use std::sync::Arc;
use wayland_client::{backend::ObjectData, protocol::wl_output, Connection, Dispatch, QueueHandle};
use wayland_protocols::ext::workspace::v1::client::{
    ext_workspace_group_handle_v1, ext_workspace_handle_v1, ext_workspace_manager_v1,
};

#[derive(Clone, Debug)]
pub struct Workspace {
    id: String,
    name: String,
}
#[derive(Debug, Clone)]
pub struct WorkspaceGroup {
    pub outputs: Vec<wl_output::WlOutput>,
    pub workspaces: Vec<ext_workspace_handle_v1::ExtWorkspaceHandleV1>,
}
impl WorkspaceGroup {
    fn new() -> Self {
        Self {
            outputs: Vec::new(),
            workspaces: Vec::new(),
        }
    }
}
impl Workspace {
    fn new() -> Self {
        Self {
            id: String::from(""),
            name: String::from(""),
        }
    }
    pub fn output_workspaces(conn: Arc<Connection>, output: wl_output::WlOutput) {
        let display = conn.display();

        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, ());

        let mut state = WaylandState {
            outputs: Vec::new(),
            workspaces: HashMap::new(),
            workspace_groups: HashMap::new(),
        };

        let _ = event_queue.roundtrip(&mut state);

        // Do another roundtrip to get output names
        let _ = event_queue.roundtrip(&mut state);

        for (_key, data) in state.workspace_groups {
            if data.outputs.contains(&output) {}
        }
    }
}

impl Dispatch<ext_workspace_manager_v1::ExtWorkspaceManagerV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        workspaces: &ext_workspace_manager_v1::ExtWorkspaceManagerV1,
        event: ext_workspace_manager_v1::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            ext_workspace_manager_v1::Event::WorkspaceGroup { workspace_group } => {}
            ext_workspace_manager_v1::Event::Workspace { workspace } => {}
            ext_workspace_manager_v1::Event::Done => {}
            ext_workspace_manager_v1::Event::Finished => {}
            _ => unreachable!(), //non-exaustive enum
        }
    }

    // tells wayland-client how to create the child object produced.
    fn event_created_child(opcode: u16, qh: &QueueHandle<Self>) -> Arc<dyn ObjectData> {
        match opcode {
            ext_workspace_manager_v1::EVT_WORKSPACE_GROUP_OPCODE => {
                qh.make_data::<ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1, ()>(())
            }
            ext_workspace_manager_v1::EVT_WORKSPACE_OPCODE => {
                qh.make_data::<ext_workspace_handle_v1::ExtWorkspaceHandleV1, ()>(())
            }
            ext_workspace_manager_v1::EVT_DONE_OPCODE => {
                panic!("IDK what to do with done");
            }
            ext_workspace_manager_v1::EVT_FINISHED_OPCODE => {
                panic!("IDK what to do with finished");
            }
            _ => panic!("unknown opcode: {opcode}"),
        }
    }
}

impl Dispatch<ext_workspace_handle_v1::ExtWorkspaceHandleV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        workspace_handle: &ext_workspace_handle_v1::ExtWorkspaceHandleV1,
        event: ext_workspace_handle_v1::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if !state.workspaces.contains_key(&workspace_handle.clone()) {
            state
                .workspaces
                .insert(workspace_handle.clone(), Workspace::new());
        }
        //at this point we know that the hashmap contains the handle as a key.
        let mut workspace = state.workspaces.get(workspace_handle).unwrap().clone();
        match event {
            ext_workspace_handle_v1::Event::Id { id } => {
                workspace.id = id;
            }
            ext_workspace_handle_v1::Event::Name { name } => {
                workspace.name = name;
            }
            ext_workspace_handle_v1::Event::Coordinates { coordinates } => {}
            ext_workspace_handle_v1::Event::State { state } => {}
            ext_workspace_handle_v1::Event::Capabilities { capabilities } => {}
            ext_workspace_handle_v1::Event::Removed => {
                state.workspaces.remove(workspace_handle);
                return;
            }
            _ => unreachable!(), //non-exaustive enum
        }
        state
            .workspaces
            .insert(workspace_handle.clone(), workspace.clone());
    }
}

impl Dispatch<ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        workspace_group: &ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1,
        event: ext_workspace_group_handle_v1::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if !state
            .workspace_groups
            .contains_key(&workspace_group.clone())
        {
            state
                .workspace_groups
                .insert(workspace_group.clone(), WorkspaceGroup::new());
        }
        match event {
            ext_workspace_group_handle_v1::Event::Capabilities { capabilities } => {}
            ext_workspace_group_handle_v1::Event::OutputEnter { output } => {
                if let Some(group) = state.workspace_groups.get_mut(workspace_group) {
                    group.outputs.push(output);
                }
            }
            ext_workspace_group_handle_v1::Event::OutputLeave { output } => {
                if let Some(group) = state.workspace_groups.get_mut(workspace_group) {
                    group.outputs.retain(|val| *val != output);
                }
            }
            ext_workspace_group_handle_v1::Event::WorkspaceEnter { workspace } => {
                if let Some(group) = state.workspace_groups.get_mut(workspace_group) {
                    group.workspaces.push(workspace);
                }
            }
            ext_workspace_group_handle_v1::Event::WorkspaceLeave { workspace } => {
                if let Some(group) = state.workspace_groups.get_mut(workspace_group) {
                    group.workspaces.retain(|val| *val != workspace);
                }
            }
            ext_workspace_group_handle_v1::Event::Removed => {}
            _ => unreachable!(), //non-exaustive enum
        }
    }
}
