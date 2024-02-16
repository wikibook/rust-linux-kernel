#pragma once

#include <cstdint>
#include <sstream>
#include <stdint.h>
#include <string>

// 테스트 클래스
class Test {
public:
    Test() : cnt(0) {}
    void inc(); // cnt를 1 증가합니다.
    std::string to_string() const; // 현재 상태를 문자열로 출력합니다.
private:
    uint32_t cnt;
};

// 정수하나를 입력받아 x3합니다.
inline uint32_t x3(uint32_t a) {
    return a * 3;
}

inline void Test::inc() { cnt++; }
inline std::string Test::to_string() const {
    std::ostringstream oss;
    oss << "Test: Called " << cnt << " times.";
    return oss.str();
}