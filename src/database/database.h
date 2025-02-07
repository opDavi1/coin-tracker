#ifndef _DATABASE_H
#define _DATABASE_H
#include "coin.h"


/*
 * Initializes the given database.
 *
 * The first argument is the filename as a string.
 *
 * Returns 0 on success and 1 on error.
 */
int db_init(char *filename);


/*
 * Closes the active database. The database needs to be initialized before this
 * function is called.
 * 
 * Returns 0 on success and 1 on error
 */
int db_close();


/* Inserts the given coin into the active database. The database needs to be 
 * initialized before this function is called.
 *
 * The first argument is the Coin to be inserted to the database.
 * The second argument (OUTPUT) is the id of the newly inserted coin. If insertion fails this argument will be set to -1.
 *
 * Returns 0 on success and 1 on error.
 */
int db_insert_coin(Coin *coin, int *id);


/*
 * Retrieves the coin with the given id from the active database. The database needs
 * to be initialized before this function is called.
 *
 * The first argument is the id to be retrieved.
 * The second argument (OUTPUT) is the coin the retrieved data should be written to.
 *
 * Returns 0 on success and 1 on error.
 */
int db_get_coin_by_id(int id, Coin *coin);


/*
 * Retrieves the coin with the given numista_id from the active database. The
 * database needs to be initialized before this function is called.
 *
 * The first argument is the numista_id to be retrieved.
 * The second argument (OUTPUT) is the coin the retrieved data should be written to.
 *
 * Returns 0 on success and 1 on error.
 */
int db_get_coin_by_numista_id(int numista_id, Coin *coin);


/* 
 * Updates the given coin with the contents of the new coin. The database needs to
 * be initialized before this function is called.
 *
 * The first argument is the id of the coin to be updated
 * The second argument is the coin object with the updated data
 *
 * Returns 0 on success and 1 on error.
 */
int db_update_coin(int id, Coin *coin);


/*
 * Removes the coin with the given id from the active database. The database needs
 * to be initialized before this function is called.
 *
 * The first argument is the id to be removed.
 *
 * Returns 0 on success and 1 on error.
 */
int db_remove_coin(int id);
#endif // !_DATABASE_H
