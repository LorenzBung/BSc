# Makefile
# Commands
CC = pdflatex -shell-escape -interaction=nonstopmode -halt-on-error
RM = rm -f

# File Variables
SOURCE = HAMA.tex
TARGET = $(SOURCE:.tex=.pdf)
HELPFILES = $(SOURCE:.tex=.aux) $(SOURCE:.tex=.log) $(SOURCE)~

# PHONY Rules
.PHONY: all clean

all: $(TARGET)

clean:
	$(RM) $(HELPFILES) $(TARGET)

# Default rule
$(TARGET): $(SOURCE)
	$(CC) $<
