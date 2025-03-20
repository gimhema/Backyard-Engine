#include <memory>

enum class QFunctionType : int {
    DEFAULT = 0,
    CHAT = 1,
    NEW_PLAYER = 2,
    PLAYER_MOVEMENT = 3
};

namespace std {
    template <>
    struct hash<FunctionType> {
        size_t operator()(const FunctionType& type) const noexcept {
            return static_cast<size_t>(type);
        }
    };
}

constexpr int toInt(FunctionType type) {
    return static_cast<int>(type);
}
