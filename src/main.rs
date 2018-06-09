extern crate discord;

use discord::model::ReactionEmoji;
use discord::model::{ChannelId, Event};
use discord::Discord;
use std::env;
use std::process::Command;

fn split_command(in_string: &str) -> (Option<&str>, Option<&str>) {
    let mut splitter = in_string.splitn(2, ' ');
    let first = splitter.next();
    let second = splitter.next();
    (first, second)
}

pub trait Echo {
    // Simple Trait to remove some boilerplate for sending text
    fn echo(&self, &ChannelId, &str) -> ();
}

impl Echo for Discord {
    fn echo(&self, id: &ChannelId, msg: &str) -> () {
        let _ = self.send_message(*id, msg, "", false);
    }
}

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
                if message.author.id == bot.id {
                    // Stop the bot from interacting with its own messages
                    continue;
                }

                let _mentioned = message.mentions.iter().any(|x| x.id == bot.id);
                let chan_id = &message.channel_id;
                let (command, args) = split_command(&message.content);

                match command {
                    Some("!test") => discord.echo(chan_id, "This is a reply to the test."),
                    Some("!toilet") => {
                        let output = Command::new("toilet")
                            .arg(&args.unwrap())
                            .output()
                            .expect("Error in toileting text");
                        let response = String::from_utf8(output.stdout).unwrap();
                        discord.echo(&chan_id, &format!("```{}```", &response));
                    }
                    Some("!quit") => {
                        if message.author.name == "rayhem" {
                            discord.echo(&chan_id, "Sayonara.");
                            break;
                        } else {
                            discord.echo(&chan_id, "Only root can do that.");
                        }
                    }
                    Some("!sudo") => discord.echo(&chan_id, "Nice try."),
                    _ => {
                        // Detect a horse emoji and respond with a gem
                        if message.content.contains("🐴") {
                            let _ = discord.add_reaction(
                                *chan_id,
                                message.id,
                                ReactionEmoji::Unicode("💎".to_string()),
                            );
                        }
                    }
                };
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
