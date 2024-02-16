#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <string.h>

#define DEVICE_FILE "/dev/dma_mmap_device"  // 장치 파일 경로
#define BUFFER_SIZE 4096                    // 버퍼 크기

int main() {
    int fd;                               // 파일 디스크립터
    char *mapped_mem;                      // 매핑된 메모리 포인터
    char test_data[] = "Hello, Zero Copy!"; // 테스트 데이터

    // 장치 파일을 읽기/쓰기 모드로 열기
    fd = open(DEVICE_FILE, O_RDWR);
    if (fd == -1) {
        perror("Error opening device");   // 오류 메시지 출력
        exit(EXIT_FAILURE);               // 프로그램 종료
    }

    // 메모리 매핑
    mapped_mem = mmap(NULL, BUFFER_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (mapped_mem == MAP_FAILED) {
        perror("Error mapping memory");  // 오류 메시지 출력
        close(fd);                       // 파일 디스크립터 닫기
        exit(EXIT_FAILURE);              // 프로그램 종료
    }
    
    // 매핑된 메모리로부터 데이터 출력
    printf("Read from DMA Buffer: %s\n", mapped_mem);

    printf("Write to DMA Buffer...\n");
    // 매핑된 메모리로 데이터 복사
    memcpy(mapped_mem, test_data, sizeof(test_data));

    // 매핑된 메모리로부터 데이터 출력
    printf("Read from DMA Buffer: %s\n", mapped_mem);

    // 메모리 매핑 해제
    if (munmap(mapped_mem, BUFFER_SIZE) == -1) {
        perror("Error unmapping memory"); // 오류 메시지 출력
    }

    close(fd);                           // 파일 디스크립터 닫기
    return 0;
}