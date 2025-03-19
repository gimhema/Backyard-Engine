#pragma pack(push, 1)
struct NewPlayer {
    uint32_t pid;
    std::string name;

    NewPlayer(uint32_t pid, std::string name) : pid(pid), name(name) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&pid), reinterpret_cast<const uint8_t*>(&pid + 1));
        uint32_t name_length = name.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&name_length), reinterpret_cast<const uint8_t*>(&name_length + 1));
        buffer.insert(buffer.end(), name.begin(), name.end());
        return buffer;
    }

    static NewPlayer deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t pid = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t name_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string name(buffer.begin() + offset, buffer.begin() + offset + name_length);
        offset += name_length;
        return NewPlayer(pid, name);
    }
};
#pragma pack(pop)
