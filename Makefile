TARGET ?= aarch64-none-elf

CARGO ?= RUST_TARGET_PATH="$(shell pwd)" cargo
CARGO_BUILD ?= $(CARGO) xbuild

OBJCOPY = cargo objcopy --
OBJCOPY_PARAMS = --strip-all -O binary

RUST_BINARY := $(shell cat Cargo.toml | grep name | cut -d\" -f 2 | tr - _)
RUST_BUILD_DIR := target/$(TARGET)
RUST_DEBUG_LIB := $(RUST_BUILD_DIR)/debug/$(RUST_BINARY)
RUST_RELEASE_LIB := $(RUST_BUILD_DIR)/release/$(RUST_BINARY)

RUST_DEPS = Cargo.toml src/*

BUILD_DIR := build
RUST_LIB := $(BUILD_DIR)/$(RUST_BINARY)
KERNEL = $(BUILD_DIR)/kernel8.img

TEST_KERNEL_SRC = $(basename $(wildcard src/bin/*.rs))
TEST_KERNEL = $(TEST_KERNEL_SRC:src/bin/%=%)

.PHONY: all clean test

all: $(KERNEL)

$(RUST_DEBUG_LIB): $(RUST_DEPS)
	@echo "+ Building $@"
	$(CARGO_BUILD) --target=$(TARGET)

$(RUST_RELEASE_LIB): $(RUST_DEPS)
	@echo "+ Building $@"
	$(CARGO_BUILD) --release --target=$(TARGET) 

ifeq ($(DEBUG),1)
$(RUST_LIB): $(RUST_DEBUG_LIB) | $(BUILD_DIR)
	@cp $< $@
else
$(RUST_LIB): $(RUST_RELEASE_LIB) | $(BUILD_DIR)
	@cp $< $@
endif

$(BUILD_DIR):
	@mkdir -p $@


$(KERNEL): $(RUST_LIB) | $(BUILD_DIR)
	@echo "+ Building $@"
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

run: $(KERNEL)
	qemu-system-aarch64 -kernel $(KERNEL) -M raspi3 -serial null -serial mon:stdio

test: $(RUST_DEBUG_LIB) | $(BUILD_DIR) 
	for test in $(TEST_KERNEL) ; do \
		cp $(RUST_BUILD_DIR)/debug/$$test $(BUILD_DIR)/$$test ; \
		$(OBJCOPY) $(OBJCOPY_PARAMS) $(BUILD_DIR)/$$test $(BUILD_DIR)/$$test.img ; \
	done


clean:
	$(CARGO) clean
	rm -rf $(BUILD_DIR)
