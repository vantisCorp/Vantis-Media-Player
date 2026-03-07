//! DAO (Decentralized Autonomous Organization) Module
//!
//! Community governance for Vantis Media Player development.
//! Token holders can vote on features, funding, and direction.

use std::collections::HashMap;

/// DAO Proposal
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub quorum: u64,
    pub deadline: u64, // Unix timestamp
    pub executed: bool,
}

#[derive(Debug, Clone)]
pub enum ProposalType {
    /// Feature request
    FeatureRequest { feature: String },
    /// Funding allocation
    Funding { amount: u64, recipient: String },
    /// Parameter change
    ParameterChange { parameter: String, new_value: String },
    /// Treasury operation
    TreasuryOperation { operation: String },
    /// General
    General,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

/// Voting power
#[derive(Debug, Clone)]
pub struct VotingPower {
    pub address: String,
    pub tokens: u64,
    pub delegated_to: Option<String>,
}

/// DAO Governance
pub struct DAO {
    proposals: HashMap<String, Proposal>,
    voting_powers: HashMap<String, VotingPower>,
    total_supply: u64,
    quorum_threshold: f32, // Percentage
    voting_period: u64,    // Seconds
}

impl DAO {
    pub fn new(total_supply: u64) -> Self {
        Self {
            proposals: HashMap::new(),
            voting_powers: HashMap::new(),
            total_supply,
            quorum_threshold: 0.1, // 10%
            voting_period: 7 * 24 * 60 * 60, // 7 days
        }
    }

    /// Create a new proposal
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        proposer: String,
        proposal_type: ProposalType,
    ) -> String {
        let id = format!("prop-{}", self.proposals.len() + 1);
        let deadline = chrono::Utc::now().timestamp() as u64 + self.voting_period;

        let proposal = Proposal {
            id: id.clone(),
            title,
            description,
            proposer,
            proposal_type,
            status: ProposalStatus::Active,
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            quorum: (self.total_supply as f32 * self.quorum_threshold) as u64,
            deadline,
            executed: false,
        };

        self.proposals.insert(id.clone(), proposal);
        id
    }

    /// Vote on a proposal
    pub fn vote(
        &mut self,
        proposal_id: &str,
        voter: &str,
        vote: Vote,
    ) -> Result<(), String> {
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Active {
            return Err("Proposal is not active".to_string());
        }

        let voting_power = self.voting_powers.get(voter)
            .ok_or("Voter has no voting power")?;

        let weight = voting_power.tokens;

        match vote {
            Vote::For => proposal.votes_for += weight,
            Vote::Against => proposal.votes_against += weight,
            Vote::Abstain => proposal.votes_abstain += weight,
        }

        Ok(())
    }

    /// Finalize a proposal
    pub fn finalize(&mut self, proposal_id: &str) -> Result<ProposalStatus, String> {
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        let total_votes = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
        
        if total_votes < proposal.quorum {
            proposal.status = ProposalStatus::Rejected;
            return Ok(ProposalStatus::Rejected);
        }

        if proposal.votes_for > proposal.votes_against {
            proposal.status = ProposalStatus::Passed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }

        Ok(proposal.status.clone())
    }

    /// Execute a passed proposal
    pub fn execute(&mut self, proposal_id: &str) -> Result<(), String> {
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Passed {
            return Err("Proposal must be passed to execute".to_string());
        }

        proposal.executed = true;
        proposal.status = ProposalStatus::Executed;

        Ok(())
    }

    /// Get proposal details
    pub fn get_proposal(&self, proposal_id: &str) -> Option<&Proposal> {
        self.proposals.get(proposal_id)
    }

    /// List all proposals
    pub fn list_proposals(&self) -> Vec<&Proposal> {
        self.proposals.values().collect()
    }

    /// Delegate voting power
    pub fn delegate(&mut self, from: &str, to: &str) -> Result<(), String> {
        let power = self.voting_powers.get_mut(from)
            .ok_or("No voting power to delegate")?;

        power.delegated_to = Some(to.to_string());
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Vote {
    For,
    Against,
    Abstain,
}

/// Treasury management
pub struct Treasury {
    balance: u64,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
    pub tx_type: TransactionType,
}

#[derive(Debug, Clone)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Grant,
    Bounty,
}

impl Treasury {
    pub fn new() -> Self {
        Self {
            balance: 0,
            transactions: Vec::new(),
        }
    }

    /// Deposit funds
    pub fn deposit(&mut self, from: &str, amount: u64) {
        self.balance += amount;
        self.transactions.push(Transaction {
            id: format!("tx-{}", self.transactions.len()),
            from: from.to_string(),
            to: "treasury".to_string(),
            amount,
            timestamp: chrono::Utc::now().timestamp() as u64,
            tx_type: TransactionType::Deposit,
        });
    }

    /// Get balance
    pub fn balance(&self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_proposal() {
        let mut dao = DAO::new(1000000);
        let id = dao.create_proposal(
            "Add AI Features".to_string(),
            "Implement AI-powered recommendations".to_string(),
            "alice".to_string(),
            ProposalType::FeatureRequest { feature: "AI".to_string() },
        );
        
        assert!(dao.get_proposal(&id).is_some());
    }

    #[test]
    fn test_voting() {
        let mut dao = DAO::new(1000000);
        
        // Setup voters
        dao.voting_powers.insert("alice".to_string(), VotingPower {
            address: "alice".to_string(),
            tokens: 100000,
            delegated_to: None,
        });
        dao.voting_powers.insert("bob".to_string(), VotingPower {
            address: "bob".to_string(),
            tokens: 50000,
            delegated_to: None,
        });

        let id = dao.create_proposal(
            "Test".to_string(),
            "Test proposal".to_string(),
            "alice".to_string(),
            ProposalType::General,
        );

        dao.vote(&id, "alice", Vote::For).unwrap();
        dao.vote(&id, "bob", Vote::Against).unwrap();

        let proposal = dao.get_proposal(&id).unwrap();
        assert_eq!(proposal.votes_for, 100000);
        assert_eq!(proposal.votes_against, 50000);
    }

    #[test]
    fn test_treasury() {
        let mut treasury = Treasury::new();
        treasury.deposit("sponsor", 1000);
        assert_eq!(treasury.balance(), 1000);
    }
}