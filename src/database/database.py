# database.py is part of coin_tracker by opDavi1; see LICENSE for details

import sqlite3
from settings import settings
from coin import Coin

DATABASE_NAME = settings["database_name"]


def init():
    global sqlite_connection
    sqlite_connection = sqlite3.connect(DATABASE_NAME + ".db")
    global cursor
    cursor = sqlite_connection.cursor()
    cursor.execute(
        "CREATE TABLE IF NOT EXISTS coins ("
        + "id INTEGER PRIMARY KEY AUTOINCREMENT,"
        + "numista_id INTEGER,"
        + "name TEXT NOT NULL,"
        + "coin_type INT,"  # CoinType enum as int value
        + "issuer TEXT,"
        + "country TEXT,"
        + "min_year INT,"
        + "max_year INT,"
        + "composition TEXT,"
        + "shape INT,"  # CoinShape enum as int value
        + "diameter REAL,"
        + "thickness REAL,"
        + "weight REAL,"
        + "orientation INT,"  # CoinOrientation enum as int value
        + "denomination TEXT,"
        + "value REAL,"
        + "value_numerator INT,"
        + "value_denominator INT,"
        + "currency TEXT,"
        + "grade INT,"  # 0 - 70 sheldon scale
        + "obverse_image TEXT,"
        + "reverse_image TEXT,"
        + "obverse_description TEXT,"
        + "reverse_destription TEXT,"
        + "is_demonitized INT,"
        + "comments TEXT)"
    )
    print("init db")
    return cursor


def insert_coin(coin: Coin):
    cursor.execute(
        "INSERT INTO coins ("
        + "id,"
        + "numista_id,"
        + "name,"
        + "coin_type,"
        + "issuer,"
        + "country,"
        + "min_year,"
        + "max_year,"
        + "composition,"
        + "shape,"
        + "diameter,"
        + "thickness,"
        + "weight,"
        + "orientation,"
        + "denomination,"
        + "value,"
        + "value_numerator,"
        + "value_denominator,"
        + "currency,"
        + "grade,"
        + "obverse_image,"
        + "reverse_image,"
        + "obverse_description,"
        + "reverse_destription,"
        + "is_demonitized,"
        + "comments)"
        + "VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        (
            coin.numista_id,
            coin.name,
            coin.coin_type.value,
            coin.issuer,
            coin.country,
            coin.min_year,
            coin.max_year,
            coin.composition,
            coin.shape.value,
            coin.diameter,
            coin.thickness,
            coin.weight,
            coin.orientation.value,
            coin.denomination,
            coin.value,
            coin.value_numerator,
            coin.value_denominator,
            coin.currency,
            coin.grade,
            coin.obverse_image,
            coin.reverse_image,
            coin.obverse_description,
            coin.reverse_destription,
            coin.is_demonitized,
            coin.comments,
        ),
    )
    sqlite_connection.commit()


def get_coin_by_numista_id(numista_id: int):
    res = cursor.execute("SELECT * FROM coins WHERE numista_id = ?", numista_id)
    return res.fetchone()


def update_coin(old_coin_id: int, new_coin: Coin):
    cursor.execute(
        "UPDATE coins SET "
        + "numista_id = ?,"
        + "name = ?,"
        + "coin_type.value = ?,"
        + "issuer = ?,"
        + "country = ?,"
        + "min_year = ?,"
        + "max_year = ?,"
        + "composition = ?,"
        + "shape.value = ?,"
        + "diameter = ?,"
        + "thickness = ?,"
        + "weight = ?,"
        + "orientation.value = ?,"
        + "denomination = ?,"
        + "value = ?,"
        + "value_numerator = ?,"
        + "value_denominator = ?,"
        + "currency = ?,"
        + "grade = ?,"
        + "obverse_image = ?,"
        + "reverse_image = ?,"
        + "obverse_description = ?,"
        + "reverse_destription = ?,"
        + "is_demonitized = ?,"
        + "comments = ?"
        + "WHERE id = ?",
        (
            new_coin.numista_id,
            new_coin.name,
            new_coin.coin_type.value,
            new_coin.issuer,
            new_coin.country,
            new_coin.min_year,
            new_coin.max_year,
            new_coin.composition,
            new_coin.shape.value,
            new_coin.diameter,
            new_coin.thickness,
            new_coin.weight,
            new_coin.orientation.value,
            new_coin.denomination,
            new_coin.value,
            new_coin.value_numerator,
            new_coin.value_denominator,
            new_coin.currency,
            new_coin.grade,
            new_coin.obverse_image,
            new_coin.reverse_image,
            new_coin.obverse_description,
            new_coin.reverse_destription,
            new_coin.is_demonitized,
            new_coin.comments,
            new_coin.id,
        ),
    )
    sqlite_connection.commit()


def delete_coin(coin: Coin):
    cursor.execute("DELETE FROM coins WHERE id = ?", coin.id)
