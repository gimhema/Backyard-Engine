#pragma once

#include <iostream>
#include <vector>
#include <cstring>  // memcpy
#include <cstdint>  // uint32_t, uint64_t
#include <stdexcept> // std::runtime_error
#include <memory>
#include <unordered_map>

#include "MessageEnum.hpp"
#include "../Unreal_QMessage_Event/QEvent_Base.h"


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


class FunctionDispatcher {
    public:
        using FunctionPtr = std::unique_ptr<IFunction>;
    
        void registerFunction(FunctionType type, FunctionPtr func) {
            functions_.emplace(type, std::move(func));
        }
    
        void execute(FunctionType type) const {
            auto it = functions_.find(type);
            if (it != functions_.end()) {
                (*it->second)(); // 기본 실행
            } else {
                std::cerr << "No function registered for type: " << toInt(type) << std::endl;
            }
        }
    
        void execute(FunctionType type, const std::vector<uint8_t>& data) const {
            auto it = functions_.find(type);
            if (it != functions_.end()) {
                (*it->second)(data); // 데이터 전달 실행
            } else {
                std::cerr << "No function registered for type: " << toInt(type) << std::endl;
            }
        }
    
    private:
        std::unordered_map<FunctionType, FunctionPtr> functions_;
    };

class MessageCaster
{
public:
    MessageCaster();
    ~MessageCaster();

private:
    int MAX_BUUFER_SIZE = 2048;
    FunctionDispatcher f_Dispatcher;

public:
    void RecvPostProcess();
    void SendPreProcess();
    void HandleMessage(const std::vector<uint8_t>& buffer);
    
    // Custom
public:
    void InitEventMap();
    void ExecFunc(int fid, const std::vector<uint8_t>& buffer);

};

