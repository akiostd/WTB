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

//trd_sc.
public fun create_aset(account: &signer) {
        let addr = signer::address_of(account);
        assert!(!exists<AsetResource>(addr), EALREADY_HAS_ASSET);

        move_to(account, AsetResource {
            balance: 0,
        });
    }
