use bevy::prelude::*;



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum LevelState {
    #[default]
    LevelStart,
    PlaceWidget,
    ShootBall,
    BallBouncing,
    EndOfRound,
    Shop,
    GameOver,
}