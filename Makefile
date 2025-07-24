# Cargo commands
CARGO := cargo

# Logger
define log_info
	echo -e "[\033[0;33m*\033[0m] $(1)"
endef

define log_success
	echo -e "[\033[0;32m+\033[0m] Done"
endef

# Targets for different architectures
all: x64 x86

# x64 builds
x64:
	@ $(call log_info,[x64] Compiling...)
	@ $(CARGO) build --release --target x86_64-pc-windows-gnu
	@ $(call log_success)

# x86 builds
x86:
	@ $(call log_info,[x86] Compiling...)
	@ $(CARGO) build --release --target i686-pc-windows-gnu
	@ $(call log_success)

clean:
	@ $(call log_info,Cleaning compile artifacts)
	@ rm -rf target
	@ $(call log_success)

.PHONY: all debug x64 x86 x64-debug x86-debug clean
