# numista.py is part of coin_tracker by opDavi1; see LICENSE for details

import requests
import coin
from settings import settings

API_KEY = settings["api_key"]


def get_coin_by_numista_id(numista_id: int):
    response = requests.get(
        "https://api.numista.com/v3/types/" + str(numista_id),
        headers={"Numista-API-Key": API_KEY},
    )

    if response.status_code == 401:
        raise Exception("The API key is missing or incorrect")

    data = response.json()
    print("===== Coin Found: =====")
    print(data)
    print("=======================")
    numista_id = data.get("id")
    name = data.get("title")
    min_year = data.get("min_year")
    max_year = data.get("max_year")
    issuer = data.get("issuer")["name"]
    composition = data.get("composition")["text"]
    diameter = data.get("size")
    thickness = data.get("thickness")
    weight = data.get("weight")
    value = data.get("value").get("numeric_value")
    value_numerator = data.get("value").get("numerator")
    value_denominator = data.get("value").get("denominator")
    currency = data.get("value").get("currency").get("name")
    obverse_description = data.get("obverse").get("description")
    reverse_destription = data.get("reverse").get("description")
    is_demonitized = data.get("demonetization").get("is_demonitized")
    comments = data.get("comments")
    shape = data.get("shape")
    orientation = data.get("orientation")
    coin_type = data.get("type")
    # c.denomination = data["value"][""]

    c = coin.Coin()
    c.numista_id = numista_id
    c.name = name
    c.min_year = min_year if min_year else 0
    c.max_year = max_year if max_year else 0
    c.issuer = issuer if issuer else ""
    c.composition = composition if composition else ""
    c.diameter = diameter if diameter else 0
    c.thickness = thickness if thickness else 0
    c.weight = weight if weight else 0
    c.value = value if value else 0
    c.value_numerator = value_numerator if value_numerator else 0
    c.value_denominator = value_denominator if value_denominator else 0
    c.currency = currency if currency else ""
    c.grade = 70
    c.obverse_description = obverse_description if obverse_description else ""
    c.reverse_description = reverse_destription if reverse_destription else ""
    c.is_demonitized = is_demonitized if is_demonitized else False
    c.comments = comments if comments else ""
    c.shape = coin.CoinShape[shape.upper()] if shape else coin.CoinShape.ROUND
    c.orientation = coin.CoinOrientation[orientation.upper()] if orientation else coin.CoinOrientation.COIN
    c.coin_type = coin.coin_type_from_str(coin_type) if coin_type else coin.CoinType.STANDARD_CIRCULATION_COINS

    # TODO download image from link provided in response and put the path in
    # these vars:

    # c.obverse_image =
    # c.reverse_image =

    return c
