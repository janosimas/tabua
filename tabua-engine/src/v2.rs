use async_trait::async_trait;
use color_eyre::eyre::bail;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

/// Players unique identifier
pub type PlayerId = usize;

/// Game context managed by the framework
#[derive(Debug, Clone)]
pub struct GameContext {
    /// All players in the game
    pub players: Vec<PlayerId>,
    /// Players that can currently take actions
    pub current_players: Vec<PlayerId>,
}

pub trait GameState {}

pub trait Action {
    fn name(&self) -> String;
}

pub trait PhaseStructure {
    fn name(&self) -> String;

    fn before_begin(&self);
    fn before_end(&self);
    fn end_if(&self);
}

pub trait TurnStructure {
    fn before_begin(&self);
    fn before_end(&self);
    fn end_if(&self);
}

pub trait StageStructure {
    fn name(&self) -> String;

    fn before_begin(&self);
    fn before_end(&self);
    fn end_if(&self);
}

#[async_trait]
pub trait Game<'a> {
    type GameState: GameState;
    type SetupData: Default;
    type Phase: PhaseStructure;
    type Stage: StageStructure;
    type PublicState: Serialize + Deserialize<'a>;
    type PrivateState: Serialize + Deserialize<'a>;
    type PlayerId: Serialize + Deserialize<'a>;
    type Action: Action + Serialize + Deserialize<'a>;
    type EndGame: Serialize + Deserialize<'a>;

    fn name() -> String;
    fn info() -> String;

    async fn setup(setup_data: &Self::SetupData) -> Result<(GameContext, Self::GameState)>;

    async fn apply_action(
        ctx: &GameContext,
        state: &Self::GameState,
        action: Self::Action,
    ) -> Result<Self::GameState>;

    async fn results(ctx: &GameContext, state: &Self::GameState) -> bool;

    async fn set_phase(
        ctx: &GameContext,
        state: &Self::GameState,
        phase: &Self::Phase,
    ) -> Result<Self::GameState>;
    async fn is_phase_over(ctx: &GameContext, state: &Self::GameState) -> bool;
    async fn phase_next(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
    async fn on_phase_begin(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
    async fn on_phase_end(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;

    async fn set_stage(
        ctx: &GameContext,
        state: &Self::GameState,
        stage: &Self::Stage,
    ) -> Result<Self::GameState>;
    async fn is_stage_over(ctx: &GameContext, state: &Self::GameState) -> bool;
    async fn stage_next(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
    async fn on_stage_begin(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
    async fn on_stage_end(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;

    async fn is_turn_over(ctx: &GameContext, state: &Self::GameState) -> bool;
    async fn turn_next(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
    async fn on_turn_begin(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
    async fn on_turn_end(ctx: &GameContext, state: &Self::GameState) -> Result<Self::GameState>;
}

async fn run<'a, T: Game<'a>>() {
    let (ctx, mut state) = T::setup(&Default::default()).await.unwrap();
}

async fn apply_action<'a, T: Game<'a>>(ctx: GameContext, mut state: T::GameState, action: T::Action) -> Result<T::GameState>{
    state = T::apply_action(&ctx, &state, action).await?;
    if T::is_phase_over(&ctx, &state).await {
        state = T::on_turn_end(&ctx, &state).await?;
        state = T::on_phase_end(&ctx, &state).await?;
    }

    if T::is_turn_over(&ctx, &state).await {
        state = T::on_turn_end(&ctx, &state).await?;
    }

    Ok(state)
}
