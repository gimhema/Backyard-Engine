#pragma pack(push, 1)
struct NewPlayer {
    uint32_t id;
    std::string name;
    std::string connect_info;

    NewPlayer(uint32_t id, std::string name, std::string connect_info) : id(id), name(name), connect_info(connect_info) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&id), reinterpret_cast<const uint8_t*>(&id + 1));
        uint32_t name_length = name.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&name_length), reinterpret_cast<const uint8_t*>(&name_length + 1));
        buffer.insert(buffer.end(), name.begin(), name.end());
        uint32_t connect_info_length = connect_info.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&connect_info_length), reinterpret_cast<const uint8_t*>(&connect_info_length + 1));
        buffer.insert(buffer.end(), connect_info.begin(), connect_info.end());
        return buffer;
    }

    static NewPlayer deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t id = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t name_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string name(buffer.begin() + offset, buffer.begin() + offset + name_length);
        offset += name_length;
        uint32_t connect_info_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string connect_info(buffer.begin() + offset, buffer.begin() + offset + connect_info_length);
        offset += connect_info_length;
        return NewPlayer(id, name, connect_info);
    }
};
#pragma pack(pop)
