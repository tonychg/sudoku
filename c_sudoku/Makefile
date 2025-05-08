SRC = board.c \
    list.c \
    random.c \
    dancing_links.c \

SRC_MAIN = $(SRC)
SRC_MAIN += main.c
OBJ_MAIN = $(SRC_MAIN:.c=.o)

NAME      = sudoku
OBJ       = $(SRC:.c=.o)
CC        = gcc -g
RM        = rm -f
PROFILER  = gprof

define run_test
	$(eval $@_TEST = $(1)_test)
	$(CC) -Wall -o ${$@_TEST} $(OBJ) ${$@_TEST}.c
	printf "## Testing %s\n" ${$@_TEST}
	./${$@_TEST}
	$(RM) ./${$@_TEST}
endef

.PHONY:all
all:    $(NAME)

$(NAME):    $(OBJ_MAIN)
	$(CC) -pg -Wall -o $(NAME) $(OBJ_MAIN)

.PHONY:clean
clean:
	$(RM) $(OBJ_MAIN)

test: $(OBJ)
	@$(call run_test, "list")
	@$(call run_test, "board")
	@$(call run_test, "dancing_links")

profile: all
	./$(NAME)
	$(PROFILER) $(NAME)

re: clean test all
