use super::Agent;



// It can be a async task
pub trait AgentLoopManager {
    fn Update();
}

// AgentAction is dependent on AgentLoopManager
pub trait AgentAction {
    fn Update();    
}
