pub struct Coin {
    pub id: i32,
    pub numista_id: i32,
    pub name: String,
    pub coin_type: CoinType,
    pub min_year: i32,
    pub max_year: i32,
    pub country: String,
    pub issuer: String,
    pub composition: String,
    pub shape: CoinShape,
    pub diameter: f32,
    pub thickness: f32,
    pub weight: f32,
    pub orientation: CoinOrientation,
    pub denomination: String,
    pub value: f32,
    pub value_numerator: i32,
    pub value_denominator: i32,
    pub currency: String,
    pub grade: i8,
    pub obverse_image: String,
    pub reverse_image: String,
    pub obverse_description: String,
    pub reverse_description: String,
    pub is_demonitized: bool,
    pub comments: String,
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

    pub fn from_sql_row(_row: &sqlite::Row) -> Coin {
        // let c = Coin::new();
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
