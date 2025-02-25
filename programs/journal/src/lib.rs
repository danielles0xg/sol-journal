use anchor_lang::prelude::*;

declare_id!("DjwpwZd5D1eSQynpGWRXYxgmppYHxwr46Faj3qrE7TdH");

#[program]
pub mod journal {
    use super::*;

    pub fn initialize(
        ctx: Context<CreateEntry>, title: String, message:String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.message = message;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(()) 
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, _title:String,message: String) -> Result<()>{
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = message;
        
        Ok(())
    }

    // no logic needed here since action is delete account
    // and all account logic is handle in the account (struct)
    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title:String) -> Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, message:String)]
pub struct CreateEntry<'info>{
    /// anchor accunt macro
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        space = 8 + JournalEntryState::INIT_SPACE,
        payer = owner
    )]
    pub journal_entry : Account<'info,JournalEntryState>,

    #[account(mut)] // state change = rust all vars are inmmutable default
    pub owner : Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, message:String)]
pub struct UpdateEntry<'info>{
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE, // reallocate space to refund rent
        realloc::payer = owner, // who receives the rent
        realloc::zero = true // reset the original calculation back to zero
    )]
    pub journal_entry : Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteEntry<'info>{
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner)]
    pub journal_entry: Account<'info,JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>

}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,

    #[max_len(50)]
    pub title: String,

    #[max_len(1000)]
    pub message: String,
}
