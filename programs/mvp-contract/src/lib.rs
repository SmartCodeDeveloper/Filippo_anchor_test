use anchor_lang::prelude::*;

declare_id!("YMC3YPso17ZRQNm5dAZw8vjWAdjexWoyKZs6E3pyA4B");

#[program]
pub mod mvp_contract {
    use super::*;
    use anchor_lang::solana_program::program::invoke;
    use anchor_lang::solana_program::system_instruction::transfer;

    pub fn initialize(
        ctx: Context<Initialize>,
        _price: u64,
        _slots: u64
    ) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.filled = 0;
        base_account.price = _price;
        base_account.slots = _slots;
        base_account.treasury = "66hNdzKJU2XWdDVZUkqfYtdnNBV3gjS8vC4QH5QDv9gE"
        .parse()
        .unwrap();
        Ok(())
    }

    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {

        let base_account = &mut ctx.accounts.base_account;  
        let timestamp = Clock::get().unwrap().unix_timestamp;
        let user = &ctx.accounts.user;
        let ukey = *ctx.accounts.user.to_account_info().key;

        invoke(
            &transfer(&user.key(), &base_account.key(), base_account.price),
            &[user.to_account_info(), base_account.to_account_info()],
        )
        .unwrap();

        let member = Member {ukey, timestamp };

        base_account.members.push(member);
        base_account.filled += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 4096)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BaseAccount {
    pub slots: u64,
    pub filled: u64,
    pub members: Vec<Member>,
    pub price: u64,
    pub treasury: Pubkey
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Member {
    pub ukey: Pubkey,
    pub timestamp: i64,
}