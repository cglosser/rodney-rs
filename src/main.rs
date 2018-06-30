extern crate discord;
extern crate rusqlite;

use discord::model::ReactionEmoji;
use discord::model::{ChannelId, Event};
use discord::Discord;

use rusqlite::Connection;

use std::env;

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

fn initialize_database(fname: &str) -> Connection {
    let connection = Connection::open(fname).expect("Could not connect to database");
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS facts ( 
        id INTEGER PRIMARY KEY, 
        fact TEXT NOT NULL,
        tidbit TEXT NOT NULL,
        verb TEXT NOT NULL default 'is',
        created_by TEXT NOT NULL,
        created_on TEXT NOT NULL default CURRENT_TIMESTAMP)",
            &[],
        )
        .expect("Could not create table");
    connection
}

fn main() {
    let database = initialize_database("facts.sqlite");

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

                let _mentioned = message.mentions.iter().any(|x| x.id == bot.id);
                let (command, _) = split_command(&message.content);

                match command {
                    Some("!test") => {
                        discord.echo(&message.channel_id, "This is a reply to the test.")
                    }
                    Some("!quit") => {
                        if message.author.name == "rayhem" {
                            discord.echo(&message.channel_id, "Sayonara.");
                            break;
                        } else {
                            discord.echo(&message.channel_id, "Only root can do that.");
                        }
                    }
                    _ => {
                        // Detect a horse emoji and respond with a gem
                        if message.content.contains("🐴") {
                            let _ = discord.add_reaction(
                                message.channel_id,
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
