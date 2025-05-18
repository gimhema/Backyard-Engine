#pragma pack(push, 1)
struct DeletePlayer {
    uint32_t id;

    DeletePlayer(uint32_t id) : id(id) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&id), reinterpret_cast<const uint8_t*>(&id + 1));
        return buffer;
    }

    static DeletePlayer deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t id = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        return DeletePlayer(id);
    }
};
#pragma pack(pop)
