use std::?::?;
use uuid::Uuid;
//000001
struct Owner {
    id: Uuid,
    name: String,
}
//wast
struct Asset {
    id: Uuid,
    description: String,
    owner: Owner,
    history: Vec<HistoryEntry>,
}
// 

// 
