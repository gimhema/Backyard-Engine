#pragma pack(push, 1)
struct ServerResponse {
    uint32_t pId;
    uint32_t responseId;
    std::string responseMessage;

    ServerResponse(uint32_t pId, uint32_t responseId, std::string responseMessage) : pId(pId), responseId(responseId), responseMessage(responseMessage) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&pId), reinterpret_cast<const uint8_t*>(&pId + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&responseId), reinterpret_cast<const uint8_t*>(&responseId + 1));
        uint32_t responseMessage_length = responseMessage.size();
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&responseMessage_length), reinterpret_cast<const uint8_t*>(&responseMessage_length + 1));
        buffer.insert(buffer.end(), responseMessage.begin(), responseMessage.end());
        return buffer;
    }

    static ServerResponse deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t pId = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t responseId = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        uint32_t responseMessage_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        std::string responseMessage(buffer.begin() + offset, buffer.begin() + offset + responseMessage_length);
        offset += responseMessage_length;
        return ServerResponse(pId, responseId, responseMessage);
    }
};
#pragma pack(pop)
