
struct Proposal {
    uint256 proposalId;
    address proposer;
    string title;
    string description;

    uint256 estimatedCost;
    uint256 submissionTime;
    uint256 votingEndTime;
    enum Status { PendingReview, ActiveVoting, Approved, Rejected, Implemented, Cancelled }
    Status currentStatus;
    mapping(address => bool)

uint256 yesVotes;
    uint256 noVotes;
    uint256 requiredDeposit;
bool depositRefunded;
}

+ MIN_SUBMISSION_INTERVAL < block.timestamp


