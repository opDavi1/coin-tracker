#ifndef _COIN_H
#define _COIN_H

typedef enum {
  COIN_ORIENTATION_COIN = 0,
  COIN_ORIENTATION_MEDAL,
  COIN_ORIENTATION_OTHER,
} CoinOrientation;

typedef enum {
    COIN_TYPE_STANDARD_CIRCULATION_COINS = 0,
    COIN_TYPE_CIRCULATING_COMMEMORATIVE_COINS,
    COIN_TYPE_NON_CIRCULATING_COINS,
    COIN_TYPE_COLLECTOR_COINS,
    COIN_TYPE_SIEGE_COINS,
    COIN_TYPE_OFFICIAL_NECESSITY_COINS,
    COIN_TYPE_MERCHANT_TOKENS,
    COIN_TYPE_LOCAL_COINS,
    COIN_TYPE_PATTERNS,
    COIN_TYPE_CONTEMPORARY_COUNTERFEITS,
    COIN_TYPE_PROTO_COINS,
    COIN_TYPE_OTHER,
} CoinType;

typedef enum {
    COIN_SHAPE_ROUND = 0,
    COIN_SHAPE_SQUARE,
    COIN_SHAPE_POLYGONAL,
    COIN_SHAPE_SCALLOPED,
    COIN_SHAPE_TRIANGULAR,
    COIN_SHAPE_OTHER,
} CoinShape;


typedef struct {
        int id;
        int numista_id ;
        char *name; 
        CoinType coin_type; 
        char *issuer;
        char *country;
        int min_year;
        int max_year;
        char *composition;
        CoinShape shape;
        float diameter;
        float thickness;
        float weight;
        CoinOrientation orientation;
        char *denomination;
        float value;
        int value_numerator;
        int value_denominator;
        char *currency;
        char grade; // sheldon scale 0-70
        char *obverse_image;
        char *reverse_image;
        char *obverse_description;
        char *reverse_description;
        int is_demonitized;
        char *comments;

} Coin;


#endif // !_COIN_H
