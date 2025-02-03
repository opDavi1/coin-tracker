OBJECT_FILES = database.o sqlite3.o

bin: $(OBJECT_FILES)
	@mkdir -p bin
	gcc -o bin/database $(OBJECT_FILES)

test: $(OBJECT_FILES) test.o
	@mkdir -p bin
	gcc -o bin/test $(OBJECT_FILES) test.o

database.o: database.c
	gcc -c database.c

sqlite3.o: include/sqlite3.c
	gcc -c include/sqlite3.c

test.o: test.c
	gcc -c test.c

clean:
	rm -rf bin/ $(OBJECT_FILES) test.o

