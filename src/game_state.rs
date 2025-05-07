use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    #[default]
    RunningSimulation,
    SimulationPausedForAI(u8),
}