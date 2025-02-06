#include <stdio.h>
#include "coin.h"
#include "database.h"


Coin test_coin = {
    0,
    0,
    "Test Coin",
    COIN_TYPE_NON_CIRCULATING_COINS,
    "opDavi1",
    "United States of America",
    2025,
    2025,
    "100% Digital",
    COIN_SHAPE_ROUND,
    1,
    0.2,
    1,
    COIN_ORIENTATION_COIN,
    "op",
    1,
    1,
    1,
    "Davi",
    70,
    "",
    "",
    "opdavi1's face",
    "a crudly drawn picture of a dollar sign",
    1,
    "TEST COMMENT"
};


int main(int argc, char *argv[]) {
    int id;
    int rc;
    rc = db_init("test_database.db");
    if (rc == 1) {
        return 1;
    }

    rc = db_insert_coin(&test_coin, &id);
    if (rc == 1) {
        return 1;
    }
    printf("Inserted coin with id: %i\n", id);

    Coin coin;
    rc = db_get_coin_by_id(1, &coin);
    if (rc == 1) {
        return 1;
    }
    db_close();

    return 0;
}
