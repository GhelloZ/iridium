pub type GameId = i64;
pub type PlayerId = i64;
pub type DeckId = i64;

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    id: GameId,
    date_played: i64,
    duration_min: Option<i64>,
    comment: Option<String>,
}

impl Game {
    pub fn new(id: GameId, date_played: i64) -> Self {
        Self {
            id,
            date_played,
            duration_min: None,
            comment: None,
        }
    }

    pub fn id(&self) -> GameId {
        self.id
    }

    pub fn date_played(&self) -> i64 {
        self.date_played
    }

    pub fn duration_min(&self) -> Option<i64> {
        self.duration_min
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn set_duration_min(&mut self, minutes: Option<i64>) {
        self.duration_min = minutes;
    }

    pub fn set_comment<S: Into<String>>(&mut self, comment: Option<S>) {
        self.comment = comment.map(Into::into);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    id: PlayerId,
    name: String,
    total_games: i64,
    wins: i64,
    second_places: i64,
    third_places: i64,
    other_finishes: i64,
    decks_owned_cnt: i64,
    decks_used_cnt: i64,
}

impl Player {
    pub fn new<S: Into<String>>(id: PlayerId, name: S) -> Self {
        Self {
            id,
            name: name.into(),
            total_games: 0,
            wins: 0,
            second_places: 0,
            third_places: 0,
            other_finishes: 0,
            decks_owned_cnt: 0,
            decks_used_cnt: 0,
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total_games(&self) -> i64 {
        self.total_games
    }

    pub fn wins(&self) -> i64 {
        self.wins
    }

    pub fn second_places(&self) -> i64 {
        self.second_places
    }

    pub fn third_places(&self) -> i64 {
        self.third_places
    }

    pub fn other_finishes(&self) -> i64 {
        self.other_finishes
    }

    pub fn decks_owned_cnt(&self) -> i64 {
        self.decks_owned_cnt
    }

    pub fn decks_used_cnt(&self) -> i64 {
        self.decks_used_cnt
    }

    pub fn record_summary(&self) -> (i64, i64, i64, i64, i64) {
        (
            self.total_games,
            self.wins,
            self.second_places,
            self.third_places,
            self.other_finishes,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    id: DeckId,
    title: String,
    commander_1: String,
    commander_2: Option<String>,
    companion: Option<String>,
    card_list: Option<String>,
    total_games: i64,
    wins: i64,
    second_places: i64,
    third_places: i64,
    other_finishes: i64,
    owner_id: PlayerId,
}

impl Deck {
    pub fn new<S: Into<String>>(
        id: DeckId,
        title: S,
        commander_1: S,
        owner_id: PlayerId,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            commander_1: commander_1.into(),
            commander_2: None,
            companion: None,
            card_list: None,
            total_games: 0,
            wins: 0,
            second_places: 0,
            third_places: 0,
            other_finishes: 0,
            owner_id,
        }
    }

    pub fn id(&self) -> DeckId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn commander_1(&self) -> &str {
        &self.commander_1
    }

    pub fn commander_2(&self) -> Option<&str> {
        self.commander_2.as_deref()
    }

    pub fn companion(&self) -> Option<&str> {
        self.companion.as_deref()
    }

    pub fn owner_id(&self) -> PlayerId {
        self.owner_id
    }

    pub fn total_games(&self) -> i64 {
        self.total_games
    }

    pub fn wins(&self) -> i64 {
        self.wins
    }

    pub fn second_places(&self) -> i64 {
        self.second_places
    }

    pub fn third_places(&self) -> i64 {
        self.third_places
    }

    pub fn other_finishes(&self) -> i64 {
        self.other_finishes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GamePlayer {
    game_id: GameId,
    player_id: PlayerId,
    deck_id: DeckId,
    rank: Option<i64>,
}

impl GamePlayer {
    pub fn new(game_id: GameId, player_id: PlayerId, deck_id: DeckId) -> Self {
        Self {
            game_id,
            player_id,
            deck_id,
            rank: None,
        }
    }

    pub fn game_id(&self) -> GameId {
        self.game_id
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn deck_id(&self) -> DeckId {
        self.deck_id
    }

    pub fn rank(&self) -> Option<i64> {
        self.rank
    }

    pub fn set_rank(&mut self, rank: Option<i64>) {
        self.rank = rank;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerDeckStats {
    player_id: PlayerId,
    deck_id: DeckId,
    total_games: i64,
    wins: i64,
    second_places: i64,
    third_places: i64,
    other_finishes: i64,
}

impl PlayerDeckStats {
    pub fn new(player_id: PlayerId, deck_id: DeckId) -> Self {
        Self {
            player_id,
            deck_id,
            total_games: 0,
            wins: 0,
            second_places: 0,
            third_places: 0,
            other_finishes: 0,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn deck_id(&self) -> DeckId {
        self.deck_id
    }

    pub fn total_games(&self) -> i64 {
        self.total_games
    }

    pub fn wins(&self) -> i64 {
        self.wins
    }

    pub fn second_places(&self) -> i64 {
        self.second_places
    }

    pub fn third_places(&self) -> i64 {
        self.third_places
    }

    pub fn other_finishes(&self) -> i64 {
        self.other_finishes
    }

    pub fn win_rate(&self) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.wins as f64 / self.total_games as f64
        }
    }
}
