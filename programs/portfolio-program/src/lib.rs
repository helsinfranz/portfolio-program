use anchor_lang::prelude::*;

declare_id!("5WueEVLErzfDRck9tRxBijEfU8Q3XL2bLPXdoGEXLJTj");

#[program]
pub mod portfolio_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.owner = *ctx.accounts.authority.key;
        portfolio.bio = String::default();
        portfolio.links = vec![];
        portfolio.image_url = String::default();
        portfolio.vouches = vec![];
        portfolio.vouch_requests = vec![];
        portfolio.messages = vec![];
        portfolio.tip_amount = 0;
        portfolio.bump = ctx.bumps.portfolio;
        Ok(())
    }

    pub fn create_portfolio(ctx: Context<CreatePortfolio>, bio: String) -> Result<()> {
        if *ctx.accounts.authority.key != ctx.accounts.portfolio.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        ctx.accounts.portfolio.bio = bio;
        Ok(())
    }

    pub fn store_links(ctx: Context<StoreLinks>, links: Vec<String>) -> Result<()> {
        if *ctx.accounts.authority.key != ctx.accounts.portfolio.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        ctx.accounts.portfolio.links.extend(links);
        Ok(())
    }

    pub fn store_image(ctx: Context<StoreImage>, image_url: String) -> Result<()> {
        if *ctx.accounts.authority.key != ctx.accounts.portfolio.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        ctx.accounts.portfolio.image_url = image_url;
        Ok(())
    }

    pub fn request_vouch(ctx: Context<RequestVouch>, vouch: VouchRequest) -> Result<()> {
        ctx.accounts.portfolio.vouch_requests.push(vouch);
        Ok(())
    }

    pub fn approve_vouch(ctx: Context<ApproveVouch>, vouch_user: Pubkey) -> Result<()> {
        let portfolio = &mut ctx.accounts.portfolio;
        if *ctx.accounts.authority.key != portfolio.owner {
            return Err(ErrorCode::Unauthorized.into());
        }

        let index = portfolio
            .vouch_requests
            .iter()
            .position(|v| v.vouched_by == vouch_user);

        if let Some(index) = index {
            let vouch_request = portfolio.vouch_requests.remove(index);
            portfolio.vouches.push(Vouch {
                vouched_by: vouch_request.vouched_by,
                comment: vouch_request.comment,
            });
        }

        Ok(())
    }

    pub fn send_message(ctx: Context<SendMessage>, content: String) -> Result<()> {
        ctx.accounts.portfolio.messages.push(Message {
            sender: *ctx.accounts.authority.key,
            content,
        });
        Ok(())
    }

    pub fn tip(ctx: Context<Tip>, amount: u64) -> Result<()> {
        ctx.accounts.portfolio.tip_amount += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"portfolio", authority.key().as_ref()],
        bump,
        payer = authority,
        space = Portfolio::LEN
    )]
    pub portfolio: Account<'info, Portfolio>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePortfolio<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct StoreLinks<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct StoreImage<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RequestVouch<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
}

#[derive(Accounts)]
pub struct ApproveVouch<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(mut)]
    pub portfolio: Account<'info, Portfolio>,
}

#[account]
pub struct Portfolio {
    pub owner: Pubkey,
    pub bio: String,
    pub links: Vec<String>,
    pub image_url: String,
    pub vouches: Vec<Vouch>,
    pub vouch_requests: Vec<VouchRequest>,
    pub messages: Vec<Message>,
    pub tip_amount: u64,
    pub bump: u8,
}

impl Portfolio {
    pub const LEN: usize = 8 // discriminator
        + 32 // owner
        + 4 + 30 // bio (max 200 chars)
        + 4 + 50 * 2 // links (max 100 links of 50 chars each)
        + 4 + 30 // image_url (max 200 chars)
        + 4 + 3 * (32 + 4 + 20) // vouches (max 100 vouches, each 32-byte Pubkey + 100 chars)
        + 4 + 3 * (32 + 4 + 20) // vouch_requests (same as above)
        + 4 + 2 * (32 + 4 + 10) // messages (max 100 messages)
        + 8 // tip_amount
        + 1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Vouch {
    pub vouched_by: Pubkey,
    pub comment: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VouchRequest {
    pub vouched_by: Pubkey,
    pub comment: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Message {
    pub sender: Pubkey,
    pub content: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access.")]
    Unauthorized,
}
