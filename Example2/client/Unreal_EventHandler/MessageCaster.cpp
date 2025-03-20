#include "MessageCaster.h"
#include "../Messages/ExampleMessage.hpp"


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