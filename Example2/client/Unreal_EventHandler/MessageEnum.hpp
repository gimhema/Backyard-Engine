#include <memory>

enum class QFunctionType : int {
    DEFAULT = 0,
    SERVER_TEST = 1,
    SERVER_TEST_2 = 2,
    SERVER_TEST_3 = 3,
    CHAT_MESSAGE = 4,
    PLAYER_MOVEMENT_UPDATE = 5,
    NEW_PLAYER = 6,
    MAKE_ACCOUNT = 7,
    VERIFY_ACCOUNT = 8,
    ENTER_NEW_PAYER = 9,
    DELETE_PLAYER = 10,
    ALLOW_CONNECT_GAME = 11,
    SERVER_RESPONSE = 12,
    ENTER_PLAYER_TO_GAME = 13,
    END = 14
};

namespace std {
    template <>
    struct hash<QFunctionType> {
        size_t operator()(const QFunctionType& type) const noexcept {
            return static_cast<size_t>(type);
        }
    };
}

constexpr int toInt(QFunctionType type) {
    return static_cast<int>(type);
}
