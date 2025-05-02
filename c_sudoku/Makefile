SRC = main.c \
    board.c \

NAME      = sudoku
OBJ       = $(SRC:.c=.o)
CC        = gcc -g
RM        = rm -f
TEST_ARGS = --test

all:    $(NAME)

$(NAME):    $(OBJ)
	$(CC) -o $(NAME) $(OBJ)

.PHONY:clean
clean:
	$(RM) $(OBJ)

test: all
	./$(NAME) --test

re: clean all
