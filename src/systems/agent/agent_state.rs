#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AgentState{
    Running,
    WaitForAction(u64)
}