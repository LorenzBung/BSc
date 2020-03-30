CC=gcc
CFLAGS=-g
RM=rm -f
TARGET=null malloc intArray1 intArray2 free
.PHONY: all clean
all: $(TARGET)
clean:
	$(RM) null malloc intArray1 intArray2 free

$(TARGET): $(TARGET).c
	$(CC) $< $(CFLAGS) -o $@
