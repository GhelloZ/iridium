use crate::model::{
    Deck,
    DeckId,
    Game,
    GameId,
    GamePlayer,
    Player,
    PlayerDeckStats,
    PlayerId,
};

pub trait Store {
    /// Error type used by this store implementation.
    type Error;

    // Players
    fn get_player(&self, id: PlayerId) -> Result<Option<Player>, Self::Error>;
    fn list_players(&self) -> Result<Vec<Player>, Self::Error>;

    // Decks
    fn get_deck(&self, id: DeckId) -> Result<Option<Deck>, Self::Error>;
    fn list_decks(&self) -> Result<Vec<Deck>, Self::Error>;
    fn list_decks_by_owner(
        &self,
        owner_id: PlayerId,
    ) -> Result<Vec<Deck>, Self::Error>;

    // Games
    fn get_game(&self, id: GameId) -> Result<Option<Game>, Self::Error>;
    fn list_games(&self) -> Result<Vec<Game>, Self::Error>;

    // Join / stats tables
    fn list_game_players_for_game(
        &self,
        game_id: GameId,
    ) -> Result<Vec<GamePlayer>, Self::Error>;

    fn get_player_deck_stats(
        &self,
        player_id: PlayerId,
        deck_id: DeckId,
    ) -> Result<Option<PlayerDeckStats>, Self::Error>;

    fn list_player_deck_stats_for_player(
        &self,
        player_id: PlayerId,
    ) -> Result<Vec<PlayerDeckStats>, Self::Error>;
}
