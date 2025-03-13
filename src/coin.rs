pub struct Coin {
    id: i32,
    numista_id: i32,
    name: String,
    coin_type: CoinType,
    min_year: i32,
    max_year: i32,
    country: String,
    issuer: String,
    composition: String,
    shape: CoinShape,
    diameter: f32,
    thickness: f32,
    weight: f32,
    orientation: CoinOrientation,
    denomination: String,
    value: f32,
    value_numerator: i32,
    value_denominator: i32,
    currency: String,
    grade: i8,
    obverse_image: String,
    reverse_image: String,
    obverse_description: String,
    reverse_description: String,
    is_demonitized: bool,
    comments: String,
}

impl Coin {
    pub fn new() -> Coin {
        Coin::default()
    }

    pub fn example() -> Coin {
        Coin {
            id: 0,
            numista_id: 22995,
            name: String::from("20 Kreuzers - Francis I"),
            coin_type: CoinType::StandardCirculationCoins,
            min_year: 1829,
            max_year: 1830,
            country: String::from("Austrian Empire"),
            issuer: String::from("Austrian Empire"),
            composition: String::from("Silver (.583)"),
            shape: CoinShape::Round,
            diameter: 27.6,
            thickness: 1.16,
            weight: 6.68,
            orientation: CoinOrientation::Medal,
            denomination: String::from("Kreuzer"),
            value: 20,
            value_numerator: 1,
            value_denominator: 3,
            currency: String::from("Gulden"),
            grade: 70,
            obverse_image: String::from(""),
            reverse_image: String::from(""),
            obverse_description: String::from("Bust of Franz I flanked by boughs"),
            reverse_description: String::from("Double-headed eagle"),
            is_demonitized: true,
            comments: String::from("There are slight differences between the workshops. \
                The image below, for example, highlights the shift in the legends on the obverse left between A and B, \
                whereas the portraits and branches are almost exactly the same. The writing on B is shifted downwards:"),
        }
    }

    pub fn from_sql_row(row: &sqlite::Row) -> Coin {
        todo!()
    }
}

impl Default for Coin {
    fn default() -> Coin {
        Coin {
            id: 0,
            numista_id: 0,
            name: String::from("Default Coin"),
            coin_type: CoinType::StandardCirculationCoins,
            min_year: 0,
            max_year: 0,
            country: String::from("Unknown"),
            issuer: String::from("Unknown"),
            composition: String::from("Unknown"),
            shape: CoinShape::Round,
            diameter: 0.0,
            thickness: 0.0,
            weight: 0.0,
            orientation: CoinOrientation::Coin,
            denomination: String::from("Unknown"),
            value: 0.0,
            value_numerator: 0,
            value_denominator: 0,
            currency: String::from("Unknown"),
            grade: 70,
            obverse_image: String::from(""),
            reverse_image: String::from(""),
            obverse_description: String::from(""),
            reverse_description: String::from(""),
            is_demonitized: false,
            comments: String::from(""),
        }
    }
}

pub enum CoinOrientation {
    Medal,
    Coin,
    Other,
}

pub enum CoinShape {
    Round,
    Square,
    Polygonal,
    Scalloped,
    Triangular,
    Other,
}

pub enum CoinType {
    StandardCirculationCoins,
    CirculatingCommemorativeCoins,
    NonCirculatingCoins,
    CollectorCoins,
    SiegeCoins,
    OfficialNecessityCoins,
    MerchantTokens,
    LocalCoins,
    Patterns,
    ContemporaryCounterfeits,
    ProtoCoins,
    Other,
}
