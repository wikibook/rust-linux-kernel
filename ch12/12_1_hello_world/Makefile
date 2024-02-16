# QEMU 환경 설정이 구성되어 있어야 함
obj-m += hello_world.o

all:
	make -C /root/Rust_Testing/Rust/linux M=$(PWD) modules

clean:
	make -C /root/Rust_Testing/Rust/linux M=$(PWD) clean

obj-m += hello_world.o

###############################################
# 단순 빌드 테스트만 하려면 아래 코드 사용
#all:
#	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules
#
#clean:
#	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
