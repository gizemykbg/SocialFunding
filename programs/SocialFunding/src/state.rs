use anchor_lang::prelude::*;

#[account]
pub struct Management {
    pub admin: Pubkey,
    pub pause: bool,
    pub project_stage: i64,
    pub voting_stage: i64,
    pub funding_stage: i64,
}
#[derive(Accounts)]
pub struct Stage<'info> {
    #[account(init,payer=admin,space=32+1+8+8+8)]
    pub management: Account<'info, Management>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//-----------------------
//COMMUNİTY
//-----------------------

#[account]
pub struct Community {
    pub members: Vec<Pubkey>,
    pub members_pool: Vec<Pubkey>,
    pub name: String,
    pub description: String,
    pub timestamp: i64,
    pub permission: bool,
}

#[derive(Accounts)]
pub struct CreateCommunity<'info> {
    #[account(init,payer=user,space=8+32+8)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct JoinCommunity<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    pub user: Signer<'info>,
}

//-----------------------
//PROJECT
//-----------------------

#[account]
pub struct Project {
    pub creator: Pubkey,
    pub community: Pubkey,
    pub subject: String,
    pub description: String,
}
#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(init,payer=creator,space=8+32+32+8+4*50+4*200)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub management: Account<'info, Management>,
    pub community: Account<'info, Community>,
    #[account(init,seeds=[b"counter",project.key().as_ref()],bump,payer=creator,space=8+8+8)]
    pub counter: Account<'info, VoteCounter>,
    pub system_program: Program<'info, System>,
}

//-----------------------
//VOTİNG PROJECT
//-----------------------
#[account]
pub struct Voting {
    pub user: Pubkey,
    pub project: Pubkey,
    pub timestamp: i64,
    pub result: VotingResult,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(project:Pubkey)]
pub struct Vote<'info> {
    #[account(init,payer=user,space=8+32+32+8+2+8,seeds=[b"voting",user.key().as_ref(),project.key().as_ref()],bump)]
    pub voting: Account<'info, Voting>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub management: Account<'info, Management>,
    pub project: Account<'info, Project>,
    #[account(
        mut,
        seeds = [b"counter", project.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, VoteCounter>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VotingResult {
    Yes,
    No,
}
#[account]
pub struct VoteCounter {
    pub yes_count: i64,
    pub no_count: i64,
}
