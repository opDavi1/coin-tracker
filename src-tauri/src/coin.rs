// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use rusqlite::{Result, Row};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Coin {
    pub id: i64,
    pub numista_id: i64,
    pub name: String,
    pub coin_type: CoinType,
    pub issuer: String,
    pub country: String,
    pub min_year: i64,
    pub max_year: i64,
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
    pub is_demonetized: bool,
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
            issuer: String::from("Austrian Empire"),
            country: String::from("Austrian Empire"),
            min_year: 1829,
            max_year: 1830,
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
            is_demonetized: true,
            comments: String::from("There are slight differences between the workshops. \
                The image below, for example, highlights the shift in the legends on the obverse left between A and B, \
                whereas the portraits and branches are almost exactly the same. The writing on B is shifted downwards:"),
        }
    }

    pub fn from_sql_row(row: &Row) -> Result<Coin> {
        let mut c = Coin::new();
        c.id = row.get::<usize, i64>(0)?;
        c.numista_id = row.get::<usize, i64>(1)?;
        c.name = row.get::<usize, String>(2)?;
        c.coin_type = row.get::<usize, CoinType>(3)?;
        c.issuer = row.get::<usize, String>(4)?;
        c.country = row.get::<usize, String>(5)?;
        c.min_year = row.get::<usize, i64>(6)?;
        c.max_year = row.get::<usize, i64>(7)?;
        c.composition = row.get::<usize, String>(8)?;
        c.shape = row.get::<usize, CoinShape>(9)?;
        c.diameter = row.get::<usize, f64>(10)?;
        c.thickness = row.get::<usize, f64>(11)?;
        c.weight = row.get::<usize, f64>(12)?;
        c.orientation = row.get::<usize, CoinOrientation>(13)?;
        c.denomination = row.get::<usize, String>(14)?;
        c.value = row.get::<usize, f64>(15)?;
        c.value_numerator = row.get::<usize, i64>(16)?;
        c.value_denominator = row.get::<usize, i64>(17)?;
        c.currency = row.get::<usize, String>(18)?;
        c.grade = row.get::<usize, i8>(19)?;
        c.obverse_image = row.get::<usize, String>(20)?;
        c.reverse_image = row.get::<usize, String>(21)?;
        c.obverse_description = row.get::<usize, String>(22)?;
        c.reverse_description = row.get::<usize, String>(23)?;
        c.is_demonetized = row.get::<usize, bool>(24)?;
        c.comments = row.get::<usize, String>(25)?;
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
            issuer: String::from("Unknown"),
            country: String::from("Unknown"),
            min_year: 0,
            max_year: 0,
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
            is_demonetized: false,
            comments: String::from(""),
        }
    }
}


#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
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


#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
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


#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
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
