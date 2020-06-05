//! Global variables common to most modules.
/// ```["Neutral", "Forestcraft", "Swordcraft", "Runecraft", "Dragoncraft", "Shadowcraft", "Bloodcraft", "Havencraft", "Portalcraft"]```
///
/// `Neutral` is set as the first element so that the actual crafts are indexed to their exact shadowverse-portal number.
pub const CRAFTS: [&str; 9] = ["Neutral", "Forestcraft", "Swordcraft", "Runecraft", "Dragoncraft",
        "Shadowcraft", "Bloodcraft", "Havencraft", "Portalcraft"];
/// `2,048`
pub const POPULATION_SIZE: usize = 2_048;
/// `8`
pub const PP_CURVE_SIZE: usize = 8;
/// `40`
pub const DECK_SIZE: usize = 40;
/// `3`
pub const MAX_CARD_COPIES: usize = 3;