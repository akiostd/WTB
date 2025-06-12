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

fn transfer_ownership(&mut self, new_owner: Owner) {
        let history_entry = HistoryEntry {
            from: self.owner.clone(),
            to: new_owner.clone(),
            timestamp: ::UTC+00:0::now(),
        };
        self.history.push(history_entry);
        self.owner = new_owner;
    }
