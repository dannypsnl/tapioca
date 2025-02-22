mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
current_dir := $(notdir $(patsubst %/,%,$(dir $(mkfile_path))))

CC := gcc
TARGETS := numberic-extension.o

all: compile
.PHONY: all

compile: $(TARGETS)
	$(CC) -shared -o tapioca.so $(TARGETS)

%.o: %.c
	$(CC) -c $< -o $@

clean:
	rm -f -- *.o *.so
.PHONY: clean
