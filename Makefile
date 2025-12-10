NAME        := computor
CARGO       := cargo
TARGET_DIR  := target
BUILD_TYPE  := release
BUILD_DIR   := $(TARGET_DIR)/$(BUILD_TYPE)
BIN_PATH    := $(BUILD_DIR)/$(NAME)
OUT         := $(NAME)

SRC 		:= 	src/main.rs src/parser.rs src/math/mod.rs src/math/polynomial.rs \
				src/math/term.rs

RM			:=	/bin/rm -f

.PHONY: all
all: $(OUT)

$(OUT): $(BIN_PATH)
	ln -sf $< $@

$(BIN_PATH): $(SRC) Cargo.toml
	$(CARGO) build --release

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: fclean
fclean: clean
	$(RM) $(OUT)
	$(RM) -r $(TARGET_DIR)
	$(RM) Cargo.lock

.PHONY: re
re: fclean all

.PHONY: test
test:
	$(CARGO) $@
