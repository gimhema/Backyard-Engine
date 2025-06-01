#pragma pack(push, 1)
struct VerifyAccount {
    uint32_t mid;
    std::string userId;
    std::string userName;
    std::string password;
    std::string connect_info;

    VerifyAccount(uint32_t mid, std::string userId, std::string userName, std::string password, std::string connect_info) : mid(mid), userId(userId), userName(userName), password(password), connect_info(connect_info) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&mid), reinterpret_cast<const uint8_t*>(&mid + 1));
        uint32_t userId_length = userId.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&userId_length), reinterpret_cast<const uint8_t*>(&userId_length + 1));
        buffer.insert(buffer.end(), userId.begin(), userId.end());
        uint32_t userName_length = userName.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&userName_length), reinterpret_cast<const uint8_t*>(&userName_length + 1));
        buffer.insert(buffer.end(), userName.begin(), userName.end());
        uint32_t password_length = password.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&password_length), reinterpret_cast<const uint8_t*>(&password_length + 1));
        buffer.insert(buffer.end(), password.begin(), password.end());
        uint32_t connect_info_length = connect_info.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&connect_info_length), reinterpret_cast<const uint8_t*>(&connect_info_length + 1));
        buffer.insert(buffer.end(), connect_info.begin(), connect_info.end());
        return buffer;
    }

    static VerifyAccount deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t mid = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t userId_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string userId(buffer.begin() + offset, buffer.begin() + offset + userId_length);
        offset += userId_length;
        uint32_t userName_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string userName(buffer.begin() + offset, buffer.begin() + offset + userName_length);
        offset += userName_length;
        uint32_t password_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string password(buffer.begin() + offset, buffer.begin() + offset + password_length);
        offset += password_length;
        uint32_t connect_info_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string connect_info(buffer.begin() + offset, buffer.begin() + offset + connect_info_length);
        offset += connect_info_length;
        return VerifyAccount(mid, userId, userName, password, connect_info);
    }
};
#pragma pack(pop)
