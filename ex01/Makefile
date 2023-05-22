GREEN = "\033[1;32m"
RED = "\033[1;31m"
YELLOW = "\033[1;33m"
BLUE = "\033[1;34m"
WHITE = "\033[1;37m"
RESET = "\033[0m"

NAME = multiplier

PROJECT = $(shell echo $(NAME) | tr '[:lower:]' '[:upper:]')

SRCS = multiplier.rs \

RUST_FLAGS = -W unused -W nonstandard_style -D warnings

all: $(NAME)

$(NAME): $(SRCS)
	@rustc $(RUST_FLAGS) $(SRCS) -o $(NAME)
	@printf $(GREEN)"\r\033[K✅ SUCCESS: "$(WHITE)$(NAME)$(GREEN)" has been created\n"$(RESET)

fclean:
	@rm -rf $(NAME)
	@printf $(RED)"\r\033[K➜ ["$(PROJECT)"] "$(WHITE)"fclean"$(RED)" has been done\n"$(RESET)

re: fclean all

.PHONY: all fclean re
