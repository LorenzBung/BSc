
BINARIES = main-two-cvs-while main-two-cvs-if main-one-cv-while main-two-cvs-while-extra-unlock
HEADERS = mythreads.h main-header.h main-common.c pc-header.h

CC=gcc -Werror

all: $(BINARIES)

clean:
	rm -f $(BINARIES)

main-one-cv-while: main-one-cv-while.c $(HEADERS)
	$(CC) -o main-one-cv-while main-one-cv-while.c -Wall -pthread

main-two-cvs-if: main-two-cvs-if.c $(HEADERS)
	$(CC) -o main-two-cvs-if main-two-cvs-if.c -Wall -pthread

main-two-cvs-while: main-two-cvs-while.c $(HEADERS)
	$(CC) -o main-two-cvs-while main-two-cvs-while.c -Wall -pthread

main-two-cvs-while-extra-unlock: main-two-cvs-while-extra-unlock.c $(HEADERS)
	$(CC) -o main-two-cvs-while-extra-unlock main-two-cvs-while-extra-unlock.c -Wall -pthread


