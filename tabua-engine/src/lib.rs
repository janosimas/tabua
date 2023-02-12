#![feature(associated_type_defaults)]

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Engine<'a> {
    type PublicState: Serialize + Deserialize<'a>;
    type PrivateState: Serialize + Deserialize<'a>;
    type PlayerId: Serialize + Deserialize<'a>;
    type Action: Serialize + Deserialize<'a>;
    type EndGame: Serialize + Deserialize<'a>;
    type CurrentPlayers = Vec<Self::PlayerId>;
    type Error;

    async fn public_state(&self) -> Result<&Self::PublicState, Self::Error>;
    async fn private_state(
        &self,
        user: &Self::PlayerId,
    ) -> Result<Vec<Self::PrivateState>, Self::Error>;

    async fn validate_action(&self, action: &Self::Action) -> Result<(), Self::Error>;
    async fn apply_action(&mut self, action: Self::Action) -> Result<(), Self::Error>;

    async fn current_players(&self) -> Result<Self::CurrentPlayers, Self::Error>;

    async fn results(&self) -> Result<Self::EndGame, Self::Error>;
}
