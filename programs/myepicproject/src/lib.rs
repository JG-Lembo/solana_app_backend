use anchor_lang::prelude::*;

declare_id!("2asCsbKNet9BuQpVZCjya3Nfk6N4J6yGWXmZojcDwoTJ");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  // A função agora aceita um parâmetro gif_link do usuário. Também referenciamos o usuário do Contexto
  pub fn add_gif(ctx: Context<HandleGif>, gif_link: String, name: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Constroi o struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      name: name.to_string(),
      users_liked: Vec::new(),
      num_likes: 0,
    };

	// Adiciona ele ao array gif_list.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn like_gif(ctx: Context<HandleGif>, name: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let user_address = *user.to_account_info().key;

    for item in &mut base_account.gif_list {
      if item.name == name {

        for user_entry in &item.users_liked {
          if *user_entry == user_address {
            return err!(AlreadyLikedError::AlreadyLiked);  
          }
        }

        item.users_liked.push(user_address);
        item.num_likes += 1;
        return Ok(())
      }
    }

    Ok(())
  }

  pub fn remove_like_gif(ctx: Context<HandleGif>, name: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let user_address = *user.to_account_info().key;

    for item in &mut base_account.gif_list {
      if item.name == name {

        let pre_length = item.users_liked.len();
        item.users_liked.retain(|&user_entry| user_entry != user_address);
        let post_length = item.users_liked.len();

        if pre_length != post_length {
          item.num_likes -= 1;
        }
        return Ok(())
      }
    }

    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Adicione o signatário que chama o método AddGif ao struct para que possamos salvá-lo
#[derive(Accounts)]
pub struct HandleGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Crie uma estrutura personalizada para trabalharmos.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub name: String,
    pub users_liked: Vec<Pubkey>,
    pub num_likes: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Anexe um vetor do tipo ItemStruct à conta.
    pub gif_list: Vec<ItemStruct>,
}

#[error_code]
pub enum AlreadyLikedError {
    #[msg("Esse usuário já curtiu esse GIF.")]
    AlreadyLiked
}