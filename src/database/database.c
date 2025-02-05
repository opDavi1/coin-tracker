#include <stdio.h>
#include <string.h>
#include "include/sqlite3.h"
#include "coin.h"


const char *SQL_CREATE_COIN_TABLE = "CREATE TABLE IF NOT EXISTS coins("
    "id INTEGER PRIMARY KEY AUTOINCREMENT,"
    "numista_id INTEGER,"
    "name TEXT NOT NULL,"
    "coin_type INT,"
    "issuer TEXT,"
    "country TEXT,"
    "min_year INT,"
    "max_year INT,"
    "composition TEXT,"
    "shape INT,"
    "diameter REAL,"
    "thickness REAL,"
    "weight REAL,"
    "orientation INT,"
    "denomination TEXT,"
    "value REAL,"
    "value_numerator INT,"
    "value_denominator INT,"
    "currency TEXT,"
    "grade INT,"
    "obverse_image TEXT,"
    "reverse_image TEXT,"
    "obverse_description TEXT,"
    "reverse_description TEXT,"
    "is_demonitized INT,"
    "comments TEXT);";


const char *SQL_INSERT_COIN = "INSERT INTO coins ("
    "id,"
    "numista_id,"
    "name,"
    "coin_type,"
    "issuer,"
    "country,"
    "min_year,"
    "max_year,"
    "composition,"
    "shape,"
    "diameter,"
    "thickness,"
    "weight,"
    "orientation,"
    "denomination,"
    "value,"
    "value_numerator,"
    "value_denominator,"
    "currency,"
    "grade,"
    "obverse_image,"
    "reverse_image,"
    "obverse_description,"
    "reverse_description,"
    "is_demonitized,"
    "comments)"
    "VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";


const char *SQL_UPDATE_COIN = "UPDATE coins SET "
    "numista_id = ?,"
    "name = ?,"
    "coin_type.value = ?,"
    "issuer = ?,"
    "country = ?,"
    "min_year = ?,"
    "max_year = ?,"
    "composition = ?,"
    "shape.value = ?,"
    "diameter = ?,"
    "thickness = ?,"
    "weight = ?,"
    "orientation.value = ?,"
    "denomination = ?,"
    "value = ?,"
    "value_numerator = ?,"
    "value_denominator = ?,"
    "currency = ?,"
    "grade = ?,"
    "obverse_image = ?,"
    "reverse_image = ?,"
    "obverse_description = ?,"
    "reverse_description = ?,"
    "is_demonitized = ?,"
    "comments = ?"
    "WHERE id = ?;";


int db_initialized = 0;
sqlite3 *db;


int _bind_coin_to_stmt(Coin *coin, sqlite3_stmt *stmt) {
    int rc = 0;

    /*
    * OR all of the rc's together so that if one binding fails we can return 
    * SQLITE_ERROR without having to write an if statement for every single
    * binding
    */
    rc |= sqlite3_bind_int(stmt, 1, coin->numista_id);
    rc |= sqlite3_bind_text(stmt, 2, coin->name, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_int(stmt, 3, coin->coin_type);
    rc |= sqlite3_bind_text(stmt, 4, coin->issuer, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_text(stmt, 5, coin->country, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_int(stmt, 6, coin->min_year);
    rc |= sqlite3_bind_int(stmt, 7, coin->max_year);
    rc |= sqlite3_bind_text(stmt, 8, coin->composition, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_int(stmt, 9, coin->shape);
    rc |= sqlite3_bind_double(stmt, 10, coin->diameter);
    rc |= sqlite3_bind_double(stmt, 11, coin->thickness);
    rc |= sqlite3_bind_double(stmt, 12, coin->weight);
    rc |= sqlite3_bind_int(stmt, 13, coin->orientation);
    rc |= sqlite3_bind_text(stmt, 14, coin->denomination, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_double(stmt, 15, coin->value);
    rc |= sqlite3_bind_int(stmt, 16, coin->value_numerator);
    rc |= sqlite3_bind_int(stmt, 17, coin->value_denominator);
    rc |= sqlite3_bind_text(stmt, 18, coin->currency, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_int(stmt, 19, coin->grade);
    rc |= sqlite3_bind_text(stmt, 20, coin->obverse_image, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_text(stmt, 21, coin->reverse_image, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_text(stmt, 22, coin->obverse_description, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_text(stmt, 23, coin->reverse_description, -1, SQLITE_STATIC);
    rc |= sqlite3_bind_int(stmt, 24, coin->is_demonitized);
    rc |= sqlite3_bind_text(stmt, 25, coin->comments, -1, SQLITE_STATIC);
    if (rc != SQLITE_OK) {
        return SQLITE_ERROR;
    }
    return SQLITE_OK;
}


void _sql_res_to_coin(sqlite3_stmt *stmt, Coin *coin) {
    printf("Converting from sql to coin object...\n");

    coin->id = sqlite3_column_int(stmt, 0);
    printf("Coin id: %i\n", coin->id);

    coin->numista_id = sqlite3_column_int(stmt, 1);
    printf("coin numista id: %i\n", coin->numista_id);

    strcpy(coin->name, (char*)(sqlite3_column_text(stmt, 2)));
    printf("coin name: %s\n", coin->name);


    /*
    coin->coin_type = sqlite3_column_int(stmt, 3);
    strcpy(coin->issuer, (char*)sqlite3_column_text(stmt, 4));
    strcpy(coin->country, (char*)sqlite3_column_text(stmt, 5));
    coin->min_year = sqlite3_column_int(stmt, 6);
    coin->max_year = sqlite3_column_int(stmt, 7);
    strcpy(coin->composition, (char*)sqlite3_column_text(stmt, 8));
    coin->shape = sqlite3_column_int(stmt, 9);
    coin->diameter = sqlite3_column_double(stmt, 10);
    coin->thickness = sqlite3_column_double(stmt, 11);
    coin->weight = sqlite3_column_double(stmt, 12);
    coin->orientation = sqlite3_column_int(stmt, 13);
    strcpy(coin->denomination, (char*)sqlite3_column_text(stmt, 14));
    coin->value = sqlite3_column_double(stmt, 15);
    coin->value_numerator = sqlite3_column_int(stmt, 16);
    coin->value_denominator = sqlite3_column_int(stmt, 17);
    strcpy(coin->currency, (char*)sqlite3_column_text(stmt, 18));
    coin->grade = sqlite3_column_int(stmt, 19);
    strcpy(coin->obverse_image, (char*)sqlite3_column_text(stmt, 20));
    strcpy(coin->reverse_image, (char*)sqlite3_column_text(stmt, 21));
    strcpy(coin->obverse_description, (char*)sqlite3_column_text(stmt, 22));
    strcpy(coin->reverse_description, (char*)sqlite3_column_text(stmt, 23));
    coin->is_demonitized = sqlite3_column_int(stmt, 24);
    strcpy(coin->comments, (char*)sqlite3_column_text(stmt, 25));
    */

    printf("Done!\n");
}


int db_init(char *filename) {
    printf("Initializing database...\n");
    if(db_initialized) {
        printf("Database already initialized\n");
        return 0;
    }

    int rc;
    sqlite3_stmt *stmt;


    rc = sqlite3_open(filename, &db);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Error: Failed to open database \"%s\": %s\n", filename, sqlite3_errmsg(db));
        return 1;
    }

    rc = sqlite3_prepare_v2(db, SQL_CREATE_COIN_TABLE, -1, &stmt, 0);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Error: Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
        return 1;
    }

    rc = sqlite3_step(stmt);
    if(rc != SQLITE_DONE) {
        fprintf(stderr, "Error: Failed to execute statement: %s\n", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return 1;
    }

    sqlite3_finalize(stmt);
    db_initialized = 1;
    printf("Done!\n");
    return 0;
}


int db_close() {
    printf("Closing databse...\n");
    if (db_initialized == 0) {
        fprintf(stderr, "Error: Database not initialized\n");
        return 1;
    }

    sqlite3_close(db);
    db_initialized = 0;
    printf("Closed.\n");
    return 0;
}


int db_insert_coin(Coin *coin, int *id) {
    if (!db_initialized) {
        *id = -1;
        fprintf(stderr, "Error: Database not initialized\n");
        return 1;
    }

    int rc;
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(db, SQL_INSERT_COIN, -1, &stmt, 0);
    if (rc != SQLITE_OK) {
        *id = -1;
        fprintf(stderr, "Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    rc = _bind_coin_to_stmt(coin, stmt);
    if (rc != SQLITE_OK) {
        sqlite3_finalize(stmt);
        fprintf(stderr, "Failed to bind coin to stmt: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        *id = -1;
        sqlite3_finalize(stmt);
        fprintf(stderr, "Failed to insert coin into database: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    *id = sqlite3_last_insert_rowid(db);
    sqlite3_finalize(stmt);

    return 0;
}


int db_get_coin_by_id(int id, Coin *coin) {
    printf("Getting coin with id: %i...\n", id);
    if (!db_initialized) {
        fprintf(stderr, "Error: Database not initialized\n");
        return 1;
    }

    int rc;
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare(db, "SELECT * FROM coins WHERE (id = ?);", -1, &stmt, 0);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        return 1;
    }
    rc = sqlite3_bind_int(stmt, 1, id);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to bind numista id to statement: %s\n", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return 1;
    }
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_ROW) {
        fprintf(stderr, "Failed to retrieve coin from database: %s\n", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return 1;
    }
    
    _sql_res_to_coin(stmt, coin);
    printf("Done!\n");
    return 0;
}


int db_get_coin_by_numista_id(int numista_id, Coin *coin) {
    if (!db_initialized) {
        fprintf(stderr, "Error: Database not initialized\n");
        return 1;
    }

    int rc;
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare(db, "SELECT * FROM coins WHERE (numista_id = ?);", -1, &stmt, 0);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        return 1;
    }
    rc = sqlite3_bind_int(stmt, 1, numista_id);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to bind numista id to statement: %s\n", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return 1;
    }
    rc = sqlite3_step(stmt);
    if (rc != SQLITE_ROW) {
        fprintf(stderr, "Failed to retrieve coin from database: %s\n", sqlite3_errmsg(db));
        sqlite3_finalize(stmt);
        return 1;
    }

    return 0;
}


int db_update_coin(int id, Coin *coin) {
    if (!db_initialized) {
        fprintf(stderr, "Error: Database not initialized\n");
        return 1;
    }

    int rc;
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(db, SQL_UPDATE_COIN, -1, &stmt, 0);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    rc = _bind_coin_to_stmt(coin, stmt);
    if (rc != SQLITE_OK) {
        sqlite3_finalize(stmt);
        fprintf(stderr, "Failed to bind coin to stmt: %s\n", sqlite3_errmsg(db));
        return 1;
    }
    sqlite3_bind_int(stmt, 26, coin->id);

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        sqlite3_finalize(stmt);
        fprintf(stderr, "Failed to update coin: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    sqlite3_finalize(stmt);
    printf("Successfully updated coin id: %i\n", id);
    return 0;
}


int db_remove_coin(int id, Coin *coin) {
    if (!db_initialized) {
        fprintf(stderr, "Error: Database not initialized\n");
        return 1;
    }
    
    int rc;
    sqlite3_stmt *stmt;
    rc = sqlite3_prepare_v2(db, "DELETE FROM coins WHERE id = ?", -1, &stmt, 0);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    rc = sqlite3_bind_int(stmt, 1, id);
    if (rc != SQLITE_OK) {
        sqlite3_finalize(stmt);
        fprintf(stderr, "Failed to bind int to statement: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    rc = sqlite3_step(stmt);
    if (rc != SQLITE_DONE) {
        sqlite3_finalize(stmt);
        fprintf(stderr, "Failed to remove coin from database: %s\n", sqlite3_errmsg(db));
        return 1;
    }

    sqlite3_finalize(stmt);
    printf("Successfully removed coin id: %i\n", id);
    return 0;
}
