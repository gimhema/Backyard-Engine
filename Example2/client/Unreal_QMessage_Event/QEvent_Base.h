#include <iostream>
#include <unordered_map>
#include <memory>
#include <functional>
#include <vector>

// 함수 객체 인터페이스
class IFunction {
public:
    std::vector<uint8_t> m_buffer;
    virtual ~IFunction() = default;
    virtual void operator()() const = 0;
    virtual void operator()(const std::vector<uint8_t>& data) const = 0;
};

