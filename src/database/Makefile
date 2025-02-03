SOURCE_FILES = database.c include/sqlite3.c
OBJECT_FILES = database.o sqlite3.o


bin: $(SOURCE_FILES)
	@mkdir -p bin
	gcc -c -Wall -Werror -fpic $(SOURCE_FILES)
	gcc -shared -o bin/database.so $(OBJECT_FILES)


test: $(OBJECT_FILES) test.o
	@mkdir -p bin
	gcc -o bin/test -Wall -Werror $(OBJECT_FILES) test.o


database.o: database.c
	gcc -c -Wall -Werror database.c


sqlite3.o: include/sqlite3.c
	gcc -c include/sqlite3.c


test.o: test.c
	gcc -c -Wall -Werror test.c


clean:
	rm -rf bin/ $(OBJECT_FILES) test.o

