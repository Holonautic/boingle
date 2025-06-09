use bevy::prelude::*;



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Startup,
    Menu,
    InGame,
    Experiments,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum LevelState {
    #[default]
    LevelStart,
    WidgetSelection,
    PlaceWidget,
    ShootBall,
    BallBouncing,
    EndOfRound,
    Shop,
    GameOver,
}