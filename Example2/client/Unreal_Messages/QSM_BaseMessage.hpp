#pragma pack(push, 1)
struct BaseMessage {
    uint32_t id;

    BaseMessage(uint32_t id) : id(id) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer(sizeof(BaseMessage));
        std::memcpy(buffer.data(), &id, sizeof(id));
        return buffer;
    }

    static BaseMessage deserialize(const std::vector<uint8_t>& buffer) {
        if (buffer.size() < sizeof(uint32_t)) {
            throw std::runtime_error("Buffer too short");
        }
        uint32_t id;
        std::memcpy(&id, buffer.data(), sizeof(uint32_t));
        return BaseMessage(id);
    }
};
#pragma pack(pop)