// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>
//
pub struct Coin {
    pub id: i64,
    pub numista_id: i64,
    pub name: String,
    pub coin_type: CoinType,
    pub min_year: i64,
    pub max_year: i64,
    pub country: String,
    pub issuer: String,
    pub composition: String,
    pub shape: CoinShape,
    pub diameter: f64,
    pub thickness: f64,
    pub weight: f64,
    pub orientation: CoinOrientation,
    pub denomination: String,
    pub value: f64,
    pub value_numerator: i64,
    pub value_denominator: i64,
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
            value: 20.0,
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

    pub fn from_sql_row(stmt: &sqlite::Statement) -> Result<Coin, sqlite::Error> {
        let mut c = Coin::new();
        c.id = stmt.read::<i64, _>("id")?;
        c.numista_id = stmt.read::<i64, _>("numista_id")?;
        c.name = stmt.read::<String, _>("name")?;
        c.coin_type = match stmt.read::<i64, _>("coin_type") {
            Ok(i) => CoinType::from_int(i),
            Err(_) => return Err(sqlite::Error{code: None, message: Some("Could not read sql".to_string())}),
        };
        c.min_year = stmt.read::<i64, _>("min_year")?;
        c.max_year = stmt.read::<i64, _>("max_year")?;
        c.country = stmt.read::<String, _>("country")?;
        c.issuer = stmt.read::<String, _>("issuer")?;
        c.composition = stmt.read::<String, _>("composition")?;
        c.shape = match stmt.read::<i64, _>("shape") {
            Ok(i) => CoinShape::from_int(i),
            Err(_) => return Err(sqlite::Error{code: None, message: Some("Could not read sql".to_string())}),
        };
        c.diameter = stmt.read::<f64, _>("diameter")?;
        c.thickness = stmt.read::<f64, _>("thickness")?;
        c.weight = stmt.read::<f64, _>("weight")?;
        c.orientation = match stmt.read::<i64, _>("orientation") {
            Ok(i) => CoinOrientation::from_int(i),
            Err(_) => return Err(sqlite::Error{code: None, message: Some("Could not read sql".to_string())}),
        };
        c.denomination = stmt.read::<String, _>("denomination")?;
        c.value = stmt.read::<f64, _>("value")?;
        c.value_numerator = stmt.read::<i64, _>("value_numerator")?;
        c.value_denominator = stmt.read::<i64, _>("value_denominator")?;
        c.currency = stmt.read::<String, _>("currency")?;
        c.grade = stmt.read::<i64, _>("grade")? as i8;
        c.obverse_image = stmt.read::<String, _>("obverse_image")?;
        c.reverse_image = stmt.read::<String, _>("reverse_image")?;
        c.obverse_description = stmt.read::<String, _>("obverse_description")?;
        c.reverse_description = stmt.read::<String, _>("reverse_description")?;
        c.is_demonitized = match stmt.read::<i64, _>("is_demonitized") {
            Ok(i) => {
                if i == 0 {false}
                else {true}
            },
            Err(_) => return Err(sqlite::Error{code: None, message: Some("Could not read sql".to_string())}),

        };
        c.comments = stmt.read::<String, _>("comments")?;
        Ok(c)
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

#[derive(Copy, Clone, Default)]
pub enum CoinOrientation {
    #[default]
    Coin,
    Medal,
    Other,
}

impl CoinOrientation {
    pub fn from_int(int: i64) -> CoinOrientation {
        match int {
            0 => CoinOrientation::Medal,
            1 => CoinOrientation::Coin,
            _ => CoinOrientation::Other,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub enum CoinShape {
    #[default]
    Round,
    Square,
    Polygonal,
    Scalloped,
    Triangular,
    Other,
}

impl CoinShape {
    pub fn from_int(int: i64) -> CoinShape {
        match int {
            0 => CoinShape::Round,
            1 => CoinShape::Square,
            2 => CoinShape::Polygonal,
            3 => CoinShape::Scalloped,
            4 => CoinShape::Triangular,
            _ => CoinShape::Other,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub enum CoinType {
    #[default]
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

impl CoinType {
    pub fn from_int(int: i64) -> CoinType {
        match int {
            0 => CoinType::StandardCirculationCoins,
            1 => CoinType::CirculatingCommemorativeCoins,
            2 => CoinType::NonCirculatingCoins,
            3 => CoinType::CollectorCoins,
            4 => CoinType::SiegeCoins,
            5 => CoinType::OfficialNecessityCoins,
            6 => CoinType::MerchantTokens,
            7 => CoinType::LocalCoins,
            8 => CoinType::Patterns,
            9 => CoinType::ContemporaryCounterfeits,
            10 => CoinType::ProtoCoins,
            _ => CoinType::Other,
        }
    }
}
