//! Native app checkpoint. The simulation is tested; UI integration is unfinished.
//! PNG sprite/terrain assets rendered by Bevy are the agreed graphics direction.
//! `pixels` is an experimental art reference, not the final rendering architecture.

use bevy::prelude::*;
use retrosim_sim::{Action, Order, State};

pub mod pixels;
pub mod ui;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Step,
    Realtime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Strike,
    Bolt,
    Guard,
    Heal,
    Interact,
    Wait,
}

#[derive(Resource)]
pub struct Lab {
    pub state: State,
    pub mode: Mode,
    pub paused: bool,
    pub selected: usize,
    pub tool: Tool,
    pub pending: Option<Action>,
    pub queued: Option<Action>,
    pub grid: bool,
    pub hover: Option<(i32, i32)>,
    pub notice: String,
    pub notice_until: f32,
    pub since_beat: f32,
    pub clock: f32,
    pub help: bool,
}

#[derive(Resource)]
pub struct Visuals {
    pub scene: Handle<Image>,
    pub portraits: Vec<Handle<Image>>,
    pub icons: Vec<Handle<Image>>,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum UiAction {
    Mode(Mode),
    SelectTool(Tool),
    Confirm,
    Cancel,
    Pause,
    Reset,
    Variation,
    Grid,
    Order(Order),
    SelectActor(usize),
    Help,
    CloseHelp,
}
