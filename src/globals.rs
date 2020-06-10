//! Global variables common to most modules.
/// ```["Neutral", "Forestcraft", "Swordcraft", "Runecraft", "Dragoncraft", "Shadowcraft", "Bloodcraft", "Havencraft", "Portalcraft"]```
///
/// `Neutral` is set as the first element so that the actual crafts are indexed to their exact
/// [shadowverse-portal](https://shadowverse-portal.com) number.
pub const CRAFTS: [&str; 9] = [
    "Neutral",
    "Forestcraft",
    "Swordcraft",
    "Runecraft",
    "Dragoncraft",
    "Shadowcraft",
    "Bloodcraft",
    "Havencraft",
    "Portalcraft",
];
/// `2,048`
pub const POPULATION_SIZE: usize = 2_048;
/// `8`
pub const PP_CURVE_SIZE: usize = 8;
/// `40`
pub const DECK_SIZE: usize = 40;
/// `3`
pub const MAX_CARD_COPIES: usize = 3;
/// The json files in `src/assets/` are loaded here at compile time to make the executable
/// fully portable, without a significant impact on the executable's size.
///
/// Given the format used in [game_mode](../context/struct.Context.html#structfield.game_mode),
/// the json for game mode `g` and class `c` is provided at `ASSETS[g%3 + c]`.
pub const ASSETS: [&str; 18] = [
    include_str!("assets/3_Neutral.json"),
    include_str!("assets/3_Forestcraft.json"),
    include_str!("assets/3_Swordcraft.json"),
    include_str!("assets/3_Runecraft.json"),
    include_str!("assets/3_Dragoncraft.json"),
    include_str!("assets/3_Shadowcraft.json"),
    include_str!("assets/3_Bloodcraft.json"),
    include_str!("assets/3_Havencraft.json"),
    include_str!("assets/3_Portalcraft.json"),
    include_str!("assets/1_Neutral.json"),
    include_str!("assets/1_Forestcraft.json"),
    include_str!("assets/1_Swordcraft.json"),
    include_str!("assets/1_Runecraft.json"),
    include_str!("assets/1_Dragoncraft.json"),
    include_str!("assets/1_Shadowcraft.json"),
    include_str!("assets/1_Bloodcraft.json"),
    include_str!("assets/1_Havencraft.json"),
    include_str!("assets/1_Portalcraft.json"),
];
