#pragma pack(push, 1)
#include <cstdint>
#include <cstring>
#include <vector>

struct BaseMessage {
    uint32_t id;

    explicit BaseMessage(uint32_t id_ = 0) : id(id_) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer(4);
        std::memcpy(buffer.data(), &id, 4);
        return buffer;
    }

    static BaseMessage deserialize(const std::vector<uint8_t>& buffer) {
        BaseMessage out{ 0 };

        const size_t n = buffer.size();
        if (n < 4) {
            // 데이터가 너무 짧으면 id=0 (상위 switch의 default로 빠짐)
            return out;
        }

        // (A) 길이 프리픽스 패턴: len == n (헤더 포함) 혹은 len + 4 == n (payload 길이)
        if (n >= 8) {
            uint32_t len = 0;
            std::memcpy(&len, buffer.data(), 4);

            if (len == n || len + 4 == n) {
                uint32_t id1 = 0;
                std::memcpy(&id1, buffer.data() + 4, 4); // ★ +4에서 id
                out.id = id1;
                return out;
            }
        }

        // (B) 프리픽스가 없거나 감지 실패 → 0 오프셋에서 id
        uint32_t id0 = 0;
        std::memcpy(&id0, buffer.data(), 4);
        out.id = id0;
        return out;
    }
};
#pragma pack(pop)
