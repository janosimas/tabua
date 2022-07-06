
use async_trait::async_trait;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Engine<'a> {
    type PublicState: Serialize + Deserialize<'a>;
    type PrivateState: Serialize + Deserialize<'a>;
    type PlayerId: Serialize + Deserialize<'a>;
    type Action: Serialize + Deserialize<'a>;
    type EndGame: Serialize + Deserialize<'a>;

    async fn public_state(&self) -> Result<Self::PublicState>;
    async fn private_state(&self, user: &Self::PlayerId) -> Result<Vec<Self::PrivateState>>;

    async fn validate_action(&self, action: &Self::Action) -> Result<()>;
    async fn apply_action(&mut self, action: Self::Action) -> Result<()>;

    async fn current_players(&self) -> Result<Vec<Self::PlayerId>>;

    async fn results(&self) -> Result<Self::EndGame>;
}
