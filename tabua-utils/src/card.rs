#[derive(Debug, PartialEq, Eq)]
pub struct Card<FrontType, BackType> {
    front: FrontType,
    back: BackType,
}

impl<FrontType, BackType> Card<FrontType, BackType> {
    fn new(front: FrontType, back: BackType) -> Self {
        Self { front, back }
    }

    pub fn front(&self) -> &FrontType {
        &self.front
    }

    pub fn front_mut(&mut self) -> &mut FrontType {
        &mut self.front
    }

    pub fn back(&self) -> &BackType {
        &self.back
    }

    pub fn back_mut(&mut self) -> &mut BackType {
        &mut self.back
    }
}

pub struct CardSetBuilder<FrontType, BackType>
where
    BackType: Clone,
    FrontType: Clone,
{
    back: BackType,
    front: Vec<(FrontType, usize)>,
}

impl<FrontType, BackType> CardSetBuilder<FrontType, BackType>
where
    BackType: Clone,
    FrontType: Clone,
{
    pub fn new(back: BackType) -> Self {
        Self {
            back,
            front: vec![],
        }
    }

    pub fn with_cards(mut self, front: FrontType, copies: usize) -> Self {
        self.front.push((front, copies));
        self
    }

    pub fn generate_deck(&self) -> Vec<Card<FrontType, BackType>> {
        self.front.iter().fold(vec![], |mut deck, (front, copies)| {
            for _ in 0..*copies {
                deck.push(Card::new(front.clone(), self.back.clone()))
            }
            deck
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_card_set() {
        let deck = CardSetBuilder::new("back")
            .with_cards("front_1", 1)
            .with_cards("front_2", 2)
            .generate_deck();

        assert_eq!(
            deck,
            vec![
                Card::new("front_1", "back"),
                Card::new("front_2", "back"),
                Card::new("front_2", "back")
            ]
        );
    }
}
