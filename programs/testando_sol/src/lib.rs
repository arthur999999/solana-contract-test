use anchor_lang::prelude::*;

declare_id!("9AtepmRcorvUw7sHrHTcfABofswrkFKXzmmK2kJiMW8b");

#[program]
pub mod testando_sol {
    use super::*;



    pub fn send_tweet(send_tweet_ctx: Context<SendTAweet>, topic:String, content: String) -> Result<( )>{

        if topic.chars().count() > 50 {

        }

        if content.chars().count() > 280 {

        }

        let my_tweet = &mut send_tweet_ctx.accounts.my_tweet;
        let sender_of_tweet = &send_tweet_ctx.accounts.sender_of_tweet;
        let clock = Clock::get().unwrap();

        my_tweet.author = *sender_of_tweet.key;
        my_tweet.timestamp = clock.unix_timestamp;
        my_tweet.topic = topic;
        my_tweet.content = content;
        Ok(())
    }
}

#[error_code]
pub enum tweetsErros {
    #[msg("This Topic is so much long")]
    TopicTooLong,
    ContentTooLong
}


#[derive(Accounts)]
pub struct SendTAweet<'info> {
    #[account(init, payer=sender_of_tweet, space=tweetOnSolana::LEN )]
    pub my_tweet: Account<'info, tweetOnSolana>,

    #[account(mut)]
    pub sender_of_tweet: Signer<'info>,
    pub system_program: Program<'info, System>

}


#[account]
pub struct tweetOnSolana{
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String
}

const DISCRIMINATION_LENGTH:usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;

impl tweetOnSolana {
    const LEN: usize = DISCRIMINATION_LENGTH + PUBLIC_KEY_LENGTH 
    + TIMESTAMP_LENGTH + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH + MAX_TOPIC_LENGTH;
}

