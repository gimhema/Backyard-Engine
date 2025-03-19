#pragma pack(push, 1)
struct ChatMessage {
    uint32_t id;
    std::string content;

    ChatMessage(uint32_t id, std::string content) : id(id), content(content) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&id), reinterpret_cast<const uint8_t*>(&id + 1));
        uint32_t content_length = content.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&content_length), reinterpret_cast<const uint8_t*>(&content_length + 1));
        buffer.insert(buffer.end(), content.begin(), content.end());
        return buffer;
    }

    static ChatMessage deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t id = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t content_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string content(buffer.begin() + offset, buffer.begin() + offset + content_length);
        offset += content_length;
        return ChatMessage(id, content);
    }
};
#pragma pack(pop)
