# coin.py is part of coin_tracker by opDavi1; see LICENSE for details

from enum import Enum


class CoinType(Enum):
    STANDARD_CIRCULATION_COINS = 0
    CIRCULATING_COMMEMORATIVE_COINS = 1
    NON_CIRCULATING_COINS = 2
    COLLECTOR_COINS = 3
    SIEGE_COINS = 4
    OFFICIAL_NECESSITY_COINS = 5
    MERCHANT_TOKENS = 6
    LOCAL_COINS = 7
    PATTERNS = 8
    CONTEMPORARY_COUNTERFEITS = 9
    PROTO_COINS = 9
    OTHER = 10


def coin_type_from_str(str: str):
    str = str.upper()
    if str == "STANDARD CIRCULATION COINS":
        return CoinType.STANDARD_CIRCULATION_COINS
    elif str == "CIRCULATING COMMEMORATIVE COINS":
        return CoinType.CIRCULATING_COMMEMORATIVE_COINS
    elif str == "NON-CIRCULATING COINS":
        return CoinType.NON_CIRCULATING_COINS
    elif str == "COLLECTOR COINS":
        return CoinType.COLLECTOR_COINS
    elif str == "SEIGE COINS":
        return CoinType.SIEGE_COINS
    elif str == "OFFICIEL NECESSITY COINS":
        return CoinType.OFFICIAL_NECESSITY_COINS
    elif str == "MERCHANT TOKENS":
        return CoinType.MERCHANT_TOKENS
    elif str == "LOCAL COINS":
        return CoinType.LOCAL_COINS
    elif str == "PATTERNS":
        return CoinType.PATTERNS
    elif str == "CONTEMPORARY COUNTERFEITS":
        return CoinType.CONTEMPORARY_COUNTERFEITS
    elif str == "PROTO-COINS":
        return CoinType.PROTO_COINS
    else:
        return CoinType.OTHER


class CoinShape(Enum):
    ROUND = 0
    SQUARE = 1
    POLYGONAL = 2
    SCALLOPED = 3
    TRIANGULAR = 4
    OTHER = 5


class CoinOrientation(Enum):
    MEDAL = 0
    COIN = 1
    OTHER = 2


class Coin:
    def __init__(self):
        self.id = 0
        self.numista_id = 0
        self.name = ""
        self.coin_type = CoinType.STANDARD_CIRCULATION_COINS
        self.min_year = 0
        self.max_year = 0
        self.country = ""
        self.issuer = ""
        self.composition = ""
        self.shape = CoinShape.ROUND
        self.diameter = 0  # millimeters
        self.thickness = 0  # millimeters
        self.weight = 0  # grams
        self.orientation = CoinOrientation.MEDAL
        self.denomination = ""
        self.value = 0
        self.value_numerator = 0
        self.value_denominator = 0
        self.currency = ""
        self.grade = 0
        self.obverse_image = ""  # file path or url for images
        self.reverse_image = ""
        self.obverse_description = ""
        self.reverse_description = ""
        self.is_demonitized = False
        self.comments = ""

    def default(self):
        self = Coin()
        self.id = 0
        self.numista_id = 22995
        self.name = "20 Kreuzers - Francis I"
        self.coin_type = CoinType.STANDARD_CIRCULATION_COINS
        self.min_year = 1829
        self.max_year = 1830
        self.country = "Austrian Empire"
        self.issuer = "Austrian Empire"
        self.composition = "Silver (.583)"
        self.shape = CoinShape.ROUND
        self.diameter = 27.6
        self.thickness = 1.16
        self.weight = 6.68
        self.orientation = CoinOrientation.MEDAL
        self.denomination = "Kreuzer"
        self.value = 20
        self.currency = "Gulden"
        self.value_numerator = 1
        self.value_denominator = 3
        self.grade = 70
        self.obverse_image = ""
        self.reverse_image = ""
        self.obverse_description = "Bust of Franz I flanked by boughs"
        self.reverse_description = "Double-headed eagle"
        self.is_demonitized = True
        self.comments = "There are slight differences between the workshops. The image below, for example, highlights the shift in the legends on the obverse left between A and B, whereas the portraits and branches are almost exactly the same. The writing on B is shifted downwards:"
        return self
