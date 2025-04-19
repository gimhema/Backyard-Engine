#pragma pack(push, 1)
struct AllowConnectGame {
    uint32_t sessionId;
    uint32_t pid;
    std::string accountId;
    std::string name;
    std::string connect_info;

    AllowConnectGame(uint32_t sessionId, uint32_t pid, std::string accountId, std::string name, std::string connect_info) : sessionId(sessionId), pid(pid), accountId(accountId), name(name), connect_info(connect_info) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&sessionId), reinterpret_cast<const uint8_t*>(&sessionId + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&pid), reinterpret_cast<const uint8_t*>(&pid + 1));
        uint32_t accountId_length = accountId.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&accountId_length), reinterpret_cast<const uint8_t*>(&accountId_length + 1));
        buffer.insert(buffer.end(), accountId.begin(), accountId.end());
        uint32_t name_length = name.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&name_length), reinterpret_cast<const uint8_t*>(&name_length + 1));
        buffer.insert(buffer.end(), name.begin(), name.end());
        uint32_t connect_info_length = connect_info.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&connect_info_length), reinterpret_cast<const uint8_t*>(&connect_info_length + 1));
        buffer.insert(buffer.end(), connect_info.begin(), connect_info.end());
        return buffer;
    }

    static AllowConnectGame deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t sessionId = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t pid = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t accountId_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string accountId(buffer.begin() + offset, buffer.begin() + offset + accountId_length);
        offset += accountId_length;
        uint32_t name_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string name(buffer.begin() + offset, buffer.begin() + offset + name_length);
        offset += name_length;
        uint32_t connect_info_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string connect_info(buffer.begin() + offset, buffer.begin() + offset + connect_info_length);
        offset += connect_info_length;
        return AllowConnectGame(sessionId, pid, accountId, name, connect_info);
    }
};
#pragma pack(pop)
