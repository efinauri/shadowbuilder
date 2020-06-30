//! Global variables common to most modules.
/// `2,048`
pub const POPULATION_SIZE: usize = 2_048;
/// `8`
pub const PP_CURVE_SIZE: usize = 8;
/// `40`
pub const DECK_SIZE: usize = 40;
/// `3`
pub const MAX_CARD_COPIES: usize = 3;
/// The json files in `src/assets/` are loaded here at compile time to make the executable
/// fully portable, without a significant impact on file size.
///
/// Given the format used in [game_mode](../context/struct.Context.html#structfield.game_mode),
/// the json for game mode `g` and class `c` is provided at `ASSETS[9*(g%3) + c]`.
/// The classes are in this order:
///
/// `Neutral, Forestcraft, Swordcraft, Runecraft, Dragoncraft, Shadowcraft, Bloodcraft, Havencraft, Portalcraft`
///
/// `Neutral` is set as the first element so that the actual crafts are indexed to their exact
/// [shadowverse-portal](https://shadowverse-portal.com) number.

pub const ASSETS: [&str; 18] = [
    include_str!("assets/Neutral_Rotation.json"),
    include_str!("assets/Forestcraft_Rotation.json"),
    include_str!("assets/Swordcraft_Rotation.json"),
    include_str!("assets/Runecraft_Rotation.json"),
    include_str!("assets/Dragoncraft_Rotation.json"),
    include_str!("assets/Shadowcraft_Rotation.json"),
    include_str!("assets/Bloodcraft_Rotation.json"),
    include_str!("assets/Havencraft_Rotation.json"),
    include_str!("assets/Portalcraft_Rotation.json"),
    include_str!("assets/Neutral_Unlimited.json"),
    include_str!("assets/Forestcraft_Unlimited.json"),
    include_str!("assets/Swordcraft_Unlimited.json"),
    include_str!("assets/Runecraft_Unlimited.json"),
    include_str!("assets/Dragoncraft_Unlimited.json"),
    include_str!("assets/Shadowcraft_Unlimited.json"),
    include_str!("assets/Bloodcraft_Unlimited.json"),
    include_str!("assets/Havencraft_Unlimited.json"),
    include_str!("assets/Portalcraft_Unlimited.json"),
];
