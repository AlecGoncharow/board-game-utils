use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BoardGames {
    #[serde(default)]
    pub name: String,
    pub entries: Vec<BoardGame>,
}

/// Noodling: I think it is worth capturing so attributes that may be worth using to pick a game
/// when ambivalence strikes. For example you could pick a random game on your shelf of shame
/// (never played before but owned games), or you could pick games that you haven't played in the
/// past month, vice versa or some combination of factors.
/// Ultimate noodle is in theory we could pull down viable player counts from BGG and use avaliable
/// players to help filter the choice even further, but thats a proper weekend project
#[derive(Debug, Deserialize)]
pub struct BoardGame {
    pub name: String,
    /// Easilly found in URL, kind of sucks to manually input but name collision and that, This may
    /// well just accept a URL or do a search to resolve and take most popular in case of
    /// collision, but likely we would want to cache the id on the loaded file to avoid hammering their API
    #[serde(default)]
    pub bgg_id: usize,
    /// Follows BGG's rating scheme
    /// See: https://boardgamegeek.com/wiki/page/ratings
    #[serde(default = "default_rating")]
    pub rating: u8,
    /// To me this means it is owned as part of a collection but has not been played ever before,
    /// granted this could also just mean the collection has a new copy that hasn't been played and
    /// the group has played it before, eye of the beholder.
    #[serde(default)]
    #[serde(alias = "unplayed")]
    #[serde(alias = "brand_new")]
    pub shelf_of_shame: bool,

    /// Probably not strictly useful but book keeping and that
    #[serde(default = "default_time")]
    pub date_acquired: NaiveDate,
    #[serde(default = "default_time")]
    pub last_played: NaiveDate,
    #[serde(default)]
    pub times_played: usize,
    // @TODO is a field for no longer in collection useful? obviously it would default to assuming
    // every entry is avaliable, but if you wanted to keep scores or sync with BGG API it could be
    // nice to not use unavaliable titles when rolling a random game
}

/// this is just a marker, to avoid needing an `Option` over the dates
fn default_time() -> NaiveDate {
    chrono::naive::MIN_DATE
}

/// this is just a marker, to avoid needing an `Option` over rating
fn default_rating() -> u8 {
    0
}
