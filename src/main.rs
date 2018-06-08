extern crate discord;

use discord::model::Event;
use discord::model::ReactionEmoji;
use discord::Discord;
use std::env;
use std::process::Command;

fn main() {
    // Log in to Discord using a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN env variable");
    let discord = Discord::from_bot_token(&token).expect("Discord login failed");

    // Establish and use a websocket connection
    let (mut connection, event) = discord.connect().expect("connect failed");
    let bot = event.user;
    println!("{} is ready to go.", bot.username);

    // Main event loop -- continuously listen for messages
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                // Stop the bot from interacting with its own messages
                if message.author.id == bot.id {
                    continue;
                }

                if message.content == "!test" {
                    let _ = discord.send_message(
                        message.channel_id,
                        "This is a reply to the test.",
                        "",
                        false,
                    );
                } else if message.content.contains("🐴") {
                    let _ = discord.add_reaction(
                        message.channel_id,
                        message.id,
                        ReactionEmoji::Unicode("💎".to_string()),
                    );
                } else if &message.content[..7] == "!toilet" {
                    let output = Command::new("toilet")
                        .arg("--irc")
                        .arg(&message.content[7..])
                        .output()
                        .expect("Error in toileting text");
                    let response = String::from_utf8(output.stdout).unwrap();

                    println!("{}", response);
                    let _ = discord.send_message(message.channel_id, &format!("```{}```",&response), "", false);
                } else if message.content == "!quit" {
                    if message.author.name == "rayhem" {
                        let _ = discord.send_message(message.channel_id, "Sayonara.", "", false);
                        println!("Quitting.");
                        break;
                    } else {
                        let _ = discord.send_message(
                            message.channel_id,
                            "Only root can do that.",
                            "",
                            false,
                        );
                    }
                } else if message.content == "sudo !quit" {
                    let _ = discord.send_message(message.channel_id, "Nice try", "", false);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}
