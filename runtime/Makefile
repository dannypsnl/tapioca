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
