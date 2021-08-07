use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "쿠쿠루삥뽕").await?;

    Ok(())
}

#[command]
#[owners_only]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = args
        .rest()
        .split("\n")
        .filter(|x| match x {
            &"```" | &"```rs" => false,
            _ => true,
        })
        .map(|x| x.to_string() + "\n")
        .collect::<String>();
    msg.reply(
        ctx,
        format!(
            "```rs\n{}\n```",
            result = match super::eval_lib::eval(r.as_str(), true) {
                Ok(e) => e,
                Err(e) => format!("Error: {}", e),
            }
            .as_str()
        ),
    )
    .await?;

    Ok(())
}