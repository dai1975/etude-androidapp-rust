RANLIB = $(NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib

help:
	echo "make <build | run | studio | clean>"

build:
	env RANLIB=$(RANLIB) cargo android build

apk:
	env RANLIB=$(RANLIB) cargo android apk build aarch64 --release

run:
	adb devices
	env RANLIB=$(RANLIB) cargo android run -f Debug

studio:
	/usr/local/android-studio/bin/studio.sh ./gen/android

clean:
	cargo clean
