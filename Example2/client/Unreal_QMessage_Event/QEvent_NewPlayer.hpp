#include "QEvent_Base.h"

// 특정 기능을 수행하는 함수 클래스
class QEvent_NewPlayer : public IFunction {
    public:
        void operator()() const override {
            std::cout << "QEvent_NewPlayer executed!" << std::endl;
        }

        void operator()(const std::vector<uint8_t>& data) const override {
            std::cout << "QEvent_NewPlayer received data: ";
            for (uint8_t byte : data) {
                // print . . .
            }
            std::cout << std::endl;
        }
    };