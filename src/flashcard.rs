pub mod properties {
    use std::cmp::Ordering;
    use bevy::prelude::*;
        
    #[derive(Debug, PartialEq, Clone)]
    pub enum Side {
        Front,
        Back
    }

    impl Side {
        pub fn flip(&mut self) {
            *self = match self {
                Side::Front => Side::Back,
                Side::Back => Side::Front
            }
        }
    }

    #[derive(Clone)]
    pub struct Flashcard {
        pub front_text: String,
        pub back_text: String,
        pub visible_side: Side
    }

    impl Flashcard {
        pub fn new(front_text: String, back_text: String) -> Flashcard {
            Flashcard {
                front_text,
                back_text,
                visible_side: Side::Front
            }
        }

        pub fn flip(&mut self) {
            self.visible_side.flip();
        }
    }


    #[derive(Resource, Clone)]
    pub struct Deck(pub Vec<Flashcard>);

    impl Deck {
        pub fn new(data: Vec<Flashcard>) -> Deck {
            Deck(data)
        }
    }


    #[derive(Component)]
    pub struct Holder {
        pub text: String,
        index: usize,
        deck: Vec<Flashcard>
    }

    impl Holder {
        pub fn new(deck: Vec<Flashcard>) -> Holder {
            let initial_text = deck[0].front_text.clone();
            Holder {
                text: initial_text,
                index: 0,
                deck
            }
        }

        pub fn next(&mut self) {
            self.index = match self.index.cmp(&(self.deck.len() - 1)) {
                Ordering::Less => self.index + 1,
                _ => 0,
            };
            self.text = self.deck[self.index].front_text.clone();
        }

        pub fn prev(&mut self) {
            self.index = match self.index.cmp(&0) {
                Ordering::Greater => self.index - 1,
                _ => self.deck.len() - 1,
            };
            self.text = self.deck[self.index].front_text.clone();
        }

        pub fn flip(&mut self) {
            self.text = match self.deck[self.index].visible_side {
                Side::Front => self.deck[self.index].front_text.clone(),
                Side::Back => self.deck[self.index].back_text.clone(),
            };
            self.deck[self.index].flip();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::properties::*;

    #[test]
    fn test_side_flip() {
        let mut side = Side::Front;
        side.flip();
        assert_eq!(side, Side::Back);
    }

    #[test]
    fn test_flashcard_flip() {
        let mut flashcard = Flashcard::new(
            "Front".to_string(),
            "Back".to_string()
        );
        flashcard.flip();
        assert_eq!(flashcard.visible_side, Side::Back);        
    }

    #[test]
    fn test_holder() {
        let deck = vec![
            Flashcard::new(
                "Front 1".to_string(),
                "Back 1".to_string()
            ),
            Flashcard::new(
                "Front 2".to_string(),
                "Back 2".to_string()
            ),
        ];
        // let deck = Deck::new(deck);
        let holder = Holder::new(deck);
        assert_eq!(holder.text, "Front 1");
    }
    
    #[test]
    fn test_holder_next() {
        let deck = vec![
            Flashcard::new(
                "Front 1".to_string(),
                "Back 1".to_string()
            ),
            Flashcard::new(
                "Front 2".to_string(),
                "Back 2".to_string()
            ),
        ];
        // let deck = Deck::new(deck);
        let mut holder = Holder::new(deck);
        holder.next();
        assert_eq!(holder.text, "Front 2");
    }

    #[test]
    fn test_holder_next_wrap() {
        let deck = vec![
            Flashcard::new(
                "Front 1".to_string(),
                "Back 1".to_string()
            ),
            Flashcard::new(
                "Front 2".to_string(),
                "Back 2".to_string()
            ),
        ];
        // let deck = Deck::new(deck);
        let mut holder = Holder::new(deck);
        holder.next();
        holder.next();
        assert_eq!(holder.text, "Front 1");
    }
    
    #[test]
    fn test_holder_prev() {
        let deck = vec![
            Flashcard::new(
                "Front 1".to_string(),
                "Back 1".to_string()
            ),
            Flashcard::new(
                "Front 2".to_string(),
                "Back 2".to_string()
            ),
        ];
        // let deck = Deck::new(deck);
        let mut holder = Holder::new(deck);
        holder.next();
        holder.prev();
        assert_eq!(holder.text, "Front 1");
    }
    
    #[test]
    fn test_holder_prev_wrap() {
        let deck = vec![
            Flashcard::new(
                "Front 1".to_string(),
                "Back 1".to_string()
            ),
            Flashcard::new(
                "Front 2".to_string(),
                "Back 2".to_string()
            ),
        ];
        // let deck = Deck::new(deck);
        let mut holder = Holder::new(deck);
        holder.prev();
        assert_eq!(holder.text, "Front 2");
    }
}
