#![feature(generic_associated_types)]

use color_eyre::Result;
use serde::{Deserialize, Serialize};

pub trait Engine {
    type PublicState<'a>: Serialize + Deserialize<'a>
    where
        Self: 'a;
    type PrivateState<'a>: Serialize + Deserialize<'a>
    where
        Self: 'a;
    type PlayerId;
    type Action;
    type EndGame;

    fn public_state(&self) -> &Self::PublicState<'_>;
    fn private_state(&self, user: &Self::PlayerId) -> &'_ [&Self::PublicState<'_>];

    fn is_action_valid(&self, action: &Self::Action) -> Result<()>;
    fn apply_action(&mut self, action: Self::Action) -> Result<&Self::PublicState<'_>>;

    fn current_players(&self) -> Vec<Self::PlayerId>;

    fn is_over(&self) -> bool;
    fn results(&self) -> Result<Self::EndGame>;
}
