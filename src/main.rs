extern crate discord;

use discord::model::Event;
use discord::Discord;
use std::env;

fn main() {
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("Expected token"))
        .expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!(
                    "{} says: {} in {}",
                    message.author.name, message.content, message.channel_id
                );

                // Stop the bot from replying to itself
                if message.author.name == "Rodney" {
                    continue;
                }

                if message.content == "!test" {
                    let _ = discord.send_message(
                        message.channel_id,
                        "This is a reply to the test.",
                        "",
                        false,
                    );
                } else if message.content.contains("horse") {
                    let _ = discord.send_message(message.channel_id, ":horse:", "", false);
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
