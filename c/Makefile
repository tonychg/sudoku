SRC = board.c \
    list.c \
    random.c \
    links.c \
    sudoku.c \
    hashtable.c \
    cli.c \

SRC_MAIN = $(SRC)
SRC_MAIN += main.c
OBJ_MAIN = $(SRC_MAIN:.c=.o)

NAME      = sudoku
DEBUG	  = debug
BIN       = bin
OBJ       = $(SRC:.c=.o)
CC        = gcc -g
RM        = rm -f
PROFILER  = gprof
TESTDIR   = .tests

define run_test
	$(eval $@_TEST = $(1)_test)
	$(CC) -Wall -o $(TESTDIR)/${$@_TEST} $(OBJ) ${$@_TEST}.c
	printf "## Testing %s\n" ${$@_TEST}
	$(TESTDIR)/./${$@_TEST}
endef

.PHONY:all
all:    $(NAME)

$(NAME):    $(OBJ_MAIN)
	$(CC) -Wall -o $(BIN)/$(NAME) $(OBJ_MAIN)

debug:      $(OBJ_MAIN)
	$(CC) -Wall -o $(BIN)/$(DEBUG) $(OBJ_MAIN)

.PHONY:clean
clean:
	$(RM) $(OBJ_MAIN)
	$(RM) $(TESTDIR)/*

test: $(OBJ)
	@$(call run_test, "list")
	@$(call run_test, "board")
	@$(call run_test, "links")
	@$(call run_test, "sudoku")
	@$(call run_test, "hashtable")

profile: all
	./$(NAME)
	$(PROFILER) $(NAME)

re: clean test all
