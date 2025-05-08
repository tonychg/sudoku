SRC = grid.c \
    random.c \
    heuristic.c \
    backtracking.c \
    dfs.c \
    list.c \

NAME      = sudoku
OBJ       = $(SRC:.c=.o)
CC        = gcc -g
RM        = rm -f
TEST_ARGS = --test

all:    $(NAME)

$(NAME):    $(OBJ)
	$(CC) -Wall -o $(NAME) $(OBJ) main.c

.PHONY:clean
clean:
	$(RM) $(OBJ)

test: test-list

test-list:	$(OBJ)
	$(CC) -Wall -o list_test $(OBJ) list_test.c
	@./list_test
	@rm -f ./list_test

re: clean all
