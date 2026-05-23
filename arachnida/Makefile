NAME	=spider
CARGO	=cargo
TARGET	=target/release/$(NAME)

all: $(NAME)

$(NAME):
	$(CARGO) build --release --bin $(NAME)
	cp $(TARGET) ./$(NAME)

clean:
	$(CARGO) clean

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re $(NAME)
