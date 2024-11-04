autofill::autofill! {

    #[derive(Default)]
    struct GameState {
        character: Character,
        room: Location,
    }

    #[derive(Default, PartialEq, Clone)]
    enum Location {
        #[default]
        Kitchen,
        Garden,
        Windmill,
        Barn,
    }

    #[derive(Default)]
    struct Character {
        inventory: Vec<Item>,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum Item {
        Cake,
        Eggs,
        Wheat,
        Milk,
    }

    /// Implements text-adventure style game that's about baking a cake by gathering the raw materials.
    /// - Reads from standard in.
    /// - Prompts are in an imaginative and fun style.
    /// - The game is won by sucessfully making a cake.
    pub fn play_game() {
        todo!()
    }
}
