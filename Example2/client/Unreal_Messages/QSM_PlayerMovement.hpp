#pragma pack(push, 1)
struct PlayerMovement  {
    uint32_t id;
    float x;
    float y;
    float z;
    float roll;
    float pitch;
    float yaw;

    PlayerMovement (uint32_t id, float x, float y, float z, float roll, float pitch, float yaw) : id(id), x(x), y(y), z(z), roll(roll), pitch(pitch), yaw(yaw) {}

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&id), reinterpret_cast<const uint8_t*>(&id + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&x), reinterpret_cast<const uint8_t*>(&x + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&y), reinterpret_cast<const uint8_t*>(&y + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&z), reinterpret_cast<const uint8_t*>(&z + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&roll), reinterpret_cast<const uint8_t*>(&roll + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&pitch), reinterpret_cast<const uint8_t*>(&pitch + 1));
        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&yaw), reinterpret_cast<const uint8_t*>(&yaw + 1));
        return buffer;
    }

    static PlayerMovement  deserialize(const std::vector<uint8_t>& buffer) {
        size_t offset = 0;
        uint32_t id = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);
        offset += sizeof(uint32_t);
        float x = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        float y = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        float z = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        float roll = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        float pitch = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        float yaw = *reinterpret_cast<const float*>(buffer.data() + offset);
        offset += sizeof(float);
        return PlayerMovement (id, x, y, z, roll, pitch, yaw);
    }
};
#pragma pack(pop)
