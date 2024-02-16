#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>

#define DEVICE_PATH "/dev/blkram"  // 장치 파일 경로를 정의
#define BUFFER_SIZE 128  // 버퍼 크기를 정의

int main() {
    char write_buf[BUFFER_SIZE] = "Hello, RAM Disk!";  // 쓰기를 위한 버퍼 초기화
    char read_buf[BUFFER_SIZE] = {0};  // 읽기를 위한 버퍼 초기화
    int fd;  // 파일 디스크립터

    // 장치 파일을 읽기/쓰기 모드로 열기
    fd = open(DEVICE_PATH, O_RDWR);
    if (fd < 0) {
        perror("Failed to open the device");  // 장치 파일 열기 실패 시 오류 메시지 출력
        return 1;
    }

    // 장치에 쓸 내용 출력
    printf("Write to device: %s\n", write_buf);

    // 장치에 데이터 쓰기
    if (write(fd, write_buf, strlen(write_buf)) < 0) {
        perror("Failed to write to the device");  // 데이터 쓰기 실패 시 오류 메시지 출력
        close(fd);  // 파일 디스크립터 닫기
        return 1;
    }

    lseek(fd, 0, SEEK_SET);  // 파일 포인터를 파일의 시작으로 이동

    // 장치에서 데이터 읽기
    if (read(fd, read_buf, BUFFER_SIZE) < 0) {
        perror("Failed to read from the device");  // 데이터 읽기 실패 시 오류 메시지 출력
        close(fd);  // 파일 디스크립터 닫기
        return 1;
    }

    // 장치에서 읽은 내용 출력
    printf("Read from device: %s\n", read_buf);

    close(fd);  // 파일 디스크립터 닫기
    return 0;
}