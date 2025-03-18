// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use std::convert::From;
use std::fmt::Debug;

use rusqlite::{Result, Row};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};

#[allow(dead_code)]
#[derive(Debug)]
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

    pub fn from_sql_row(row: &Row) -> Result<Coin> {
        let mut c = Coin::new();
        c.id = row.get::<usize, i64>(1)?;
        c.numista_id = row.get::<usize, i64>(2)?;
        c.name = row.get::<usize, String>(3)?;
        c.coin_type = row.get::<usize, CoinType>(4)?;
        c.min_year = row.get::<usize, i64>(5)?;
        c.max_year = row.get::<usize, i64>(6)?;
        c.country = row.get::<usize, String>(7)?;
        c.issuer = row.get::<usize, String>(8)?;
        c.composition = row.get::<usize, String>(9)?;
        c.shape = row.get::<usize, CoinShape>(10)?;
        c.diameter = row.get::<usize, f64>(11)?;
        c.thickness = row.get::<usize, f64>(12)?;
        c.weight = row.get::<usize, f64>(13)?;
        c.orientation = row.get::<usize, CoinOrientation>(14)?;
        c.denomination = row.get::<usize, String>(15)?;
        c.value = row.get::<usize, f64>(16)?;
        c.value_numerator = row.get::<usize, i64>(17)?;
        c.value_denominator = row.get::<usize, i64>(18)?;
        c.currency = row.get::<usize, String>(19)?;
        c.grade = row.get::<usize, i8>(20)?;
        c.obverse_image = row.get::<usize, String>(21)?;
        c.reverse_image = row.get::<usize, String>(22)?;
        c.obverse_description = row.get::<usize, String>(23)?;
        c.reverse_description = row.get::<usize, String>(24)?;
        c.is_demonitized = row.get::<usize, bool>(25)?;
        c.comments = row.get::<usize, String>(26)?;
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


#[derive(Copy, Clone, Debug, Default)]
pub enum CoinOrientation {
    Medal,
    #[default]
    Coin,
    Other,
}


impl From<i64> for CoinOrientation {
    fn from(value: i64) -> Self {
        match value {
            0 => CoinOrientation::Medal,
            1 => CoinOrientation::Coin,
            _ => CoinOrientation::Other,
        }
    }
}


impl FromSql for CoinOrientation {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => Ok(CoinOrientation::from(i)),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}


#[derive(Copy, Clone, Debug, Default)]
pub enum CoinShape {
    #[default]
    Round,
    Square,
    Polygonal,
    Scalloped,
    Triangular,
    Other,
}


impl From<i64> for CoinShape {
    fn from(value: i64) -> Self {
        match value {
            0 => CoinShape::Round,
            1 => CoinShape::Square,
            2 => CoinShape::Polygonal,
            3 => CoinShape::Scalloped,
            4 => CoinShape::Triangular,
            _ => CoinShape::Other,
        }
    }
}


impl FromSql for CoinShape {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => Ok(CoinShape::from(i)),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}


#[derive(Copy, Clone, Debug, Default)]
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


impl From<i64> for CoinType {
    fn from(value: i64) -> Self {
        match value {
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


impl FromSql for CoinType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => Ok(CoinType::from(i)),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
