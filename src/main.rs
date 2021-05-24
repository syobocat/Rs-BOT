use std::{collections::HashSet, env};

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler, Mentionable};

// Handler構造体。取得したいイベントを実装する
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Botが起動したときに走る処理
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(say, repeat)]
struct General;

#[command]
#[description = "猫のように鳴く"]
async fn say(ctx: &Context, msg: &Message, text: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, format!("{}", text.rest()))
        .await?;
    msg.delete(&ctx.http).await?;
    Ok(())
}

#[command]
#[description = "任意のテキストを送信します。"]
async fn repeat(ctx: &Context, msg: &Message, text: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, format!("{}", text.rest()))
        .await?;
    Ok(())
}

#[help] // Helpコマンド
#[individual_command_tip = "ヘルプ"] // Helpコマンドの説明
#[strikethrough_commands_tip_in_guild = ""] // 使用できないコマンドについての説明を削除
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    // _ は使用しない返り値を捨てることを明示している
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    // 空のタプルをreturn（仕様）
    // Rustでは`;`なしの行は値としてreturnすることを表す
    // return Ok(()); と同義
    Ok(())
}

#[tokio::main]
async fn main() {
    // TOKEN
    let token: &str = &env::var("token").unwrap();
    // コマンド設定
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!!")) // コマンドプレフィックス
        .help(&MY_HELP) // ヘルプコマンドを追加
        .group(&GENERAL_GROUP); // general を追加するには,GENERAL_GROUP とグループ名をすべて大文字にする

    // Botのクライアントを作成
    let mut client = Client::builder(&token)
        .event_handler(Handler) // 取得するイベント
        .framework(framework) // コマンドを登録
        .await
        .expect("Err creating client"); // エラーハンドリング

    // メインループ。Botを起動
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
