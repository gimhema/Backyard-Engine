#include "MessageCaster.h"
#include "../Messages/ExampleMessage.hpp"
#include "../Unreal_QMessage_Event/QEvent_Chat.hpp"
#include "../Unreal_QMessage_Event/QEvent_NewPlayer.hpp"
#include "../Unreal_QMessage_Event/QEvent_PlayerMovement.hpp"

MessageCaster::MessageCaster()
{

}


MessageCaster::~MessageCaster()
{

}


void MessageCaster::RecvPostProcess()
{

}

void MessageCaster::SendPreProcess()
{

}

void MessageCaster::HandleMessage(const std::vector<uint8_t>& buffer)
{
    BaseMessage base_message = BaseMessage::deserialize(buffer);

    
}

void MessageCaster::InitEventMap()
{
    dispatcher.registerFunction(QFunctionType::CHAT, std::make_unique<QEvent_Chat>());
    dispatcher.registerFunction(QFunctionType::NEW_PLAYER, std::make_unique<QEvent_NewPlayer>());
    dispatcher.registerFunction(QFunctionType::PLAYER_MOVEMENT, std::make_unique<QEvent_PlayerMovement>());

}

void MessageCaster::ExecFunc(int fid, const std::vector<uint8_t>& buffer)
{
    dispatcher.execute(static_cast<QFunctionType>(fid), buffer);
}
