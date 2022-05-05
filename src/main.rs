use {
    rand::{thread_rng, Rng},
    serde::{Deserialize, Serialize},
    serde_json::{from_str, to_string, to_value, Value},
    serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    },
    std::{
        array, env,
        fs::{read_to_string, File, OpenOptions},
        io::{Read, Write},
        path::Path,
    },
};
// holds link details for !link
#[derive(Deserialize, Serialize, Debug)]
struct PissLink {
    link: String,
    name: String,
    author: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct PissLinkParent {
    links: Vec<PissLink>,
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        let text: &str = &msg.content.clone();
        let mut spaces: Vec<usize> = Vec::new();

        if text[0..=0].contains("!") {
            if text.contains(" ") {
                for (index, letter) in text.chars().enumerate() {
                    if letter == ' ' {
                        spaces.push(index);
                    }
                }
                // commands with params
                let first_space: usize = spaces[0];
                let command: &str = &text[1..first_space];

                match command {
                    "link" => {
                        let mut current_link = String::from("");

                        match &text[first_space + 1..spaces[1]] {
                            "copy" => {
                                //
                                // used to set the current "scope" for file writes
                                let mut current_db = String::from("");
                                //
                                // do all the checkign :sadge:
                                if msg.guild_id != None {
                                    match !Path::new(&format!("g{:?}.json", &msg.guild_id)).exists()
                                    {
                                        true => {
                                            // create the guild's db if it dosen't exist
                                            // after, set this db as the db scope
                                            let mut guild_db =
                                                File::create(format!("g{:?}.json", &msg.guild_id))
                                                    .unwrap();
                                            guild_db
                                                .write_all("{\"links\":[]}".as_bytes())
                                                .unwrap();
                                            current_db = format!("g{:?}.json", &msg.guild_id);
                                        }
                                        false => {
                                            // don't recreate the file if it exists
                                            // set the db scope for this message
                                            current_db = format!("g{:?}.json", &msg.guild_id);
                                        }
                                        _ => {
                                            println!("guild db: error checking path???")
                                        }
                                    }
                                } else {
                                    /* prefix with a u and use userid instead of guild id */
                                    match !Path::new(&format!("u{:?}.json", &msg.author.id))
                                        .exists()
                                    {
                                        true => {
                                            let mut user_db =
                                                File::create(format!("u{:?}.json", &msg.author.id))
                                                    .unwrap();
                                            user_db.write_all("{\"links\":[]}".as_bytes()).unwrap();
                                            current_db = format!("u{:?}.json", &msg.author.id);
                                        }
                                        false => {
                                            current_db = format!("u{:?}.json", &msg.author.id);
                                        }
                                        _ => {
                                            println!("user db: error checking path???")
                                        }
                                    }
                                }
                                /*
                                register a link
                                */
                                //
                                // based on the above checks, set the
                                // destination file path to the db scope
                                let mut db_scope_dest = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .open(&current_db)
                                    .unwrap();
                                println!("{:?}", &db_scope_dest);
                                //
                                // get the !link params and turn them into json
                                let mut db_data_rs: PissLinkParent =
                                    from_str(&read_to_string(&current_db).unwrap().clone())
                                        .unwrap();
                                println!("{:#?}", &db_data_rs);

                                // push the new piss link to the list
                                db_data_rs.links.push(PissLink {
                                    author: format!("{:?}", &msg.author.id),
                                    name: String::from(&text[spaces[2] + 1..]).to_owned(),
                                    link: String::from(&text[spaces[1] + 1..spaces[2]]).to_owned(),
                                });

                                /*
                                save to fiel
                                */
                                current_link = to_string(&db_data_rs).unwrap();
                                println!(
                                    "{:?}\n{} copied a link:\n{}",
                                    &msg.guild_id, &msg.author.name, &current_link
                                );
                                //
                                // write the !link json to it's properly scoped file
                                db_scope_dest.write_all(current_link.as_bytes()).unwrap();
                            }
                            "paste" => { /* grab a link from the guild db and post in chat */ }
                            "delete" => { /* remove a registered link name */ }
                            "list" => { /* show all links for guild id */ }
                            _ => { /* else, not a valid operation */ }
                        }
                    }
                    "pp" => {
                        match &text[first_space + 1..spaces[1]] {
                            "chance" => { /* operate on the tracked pp data */ },
                            "track" => {  },
                            _ => {}
                        }

                    },
                    "echo" => {
                        msg.channel_id
                            .say(&ctx.http, format!("{}", String::from(&text[6..])))
                            .await
                            .unwrap();
                    }
                    "roll" => {
                        //
                        // ROLL A SINGLE DIE
                        let num: u64 =
                            thread_rng().gen_range(1..=(text[6..].parse::<u64>()).unwrap().clone());
                        if let Err(why) = msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!(
                                    "{:?}\n{} rolled a d{}: {}",
                                    &msg.guild_id,
                                    &msg.author.name,
                                    &text[6..],
                                    num
                                ),
                            )
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }

                        println!(
                            "{:?}\n{}: ran \"roll {}: {}\"\n",
                            &msg.guild_id,
                            &msg.author.name,
                            &text[6..],
                            &num
                        );
                    }
                    "stats" => {}
                    // handle "not found" errors
                    _ => {
                        println!(
                            "{:?}\n{} error: not a command: {}",
                            &msg.guild_id, &msg.author.name, &command
                        )
                    }
                }
            } else {
                // non-space commands / commands w  no args go here
                let command: &str = &text[1..];
                match command {
                    "ping" => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    "stats" => {
                        // Roll a random set of 5e ability scores
                        // rolls (4d6dl * 7)dl
                        let mut postmsg: String = String::new();
                        let mut keep_sets: Vec<Vec<u8>> = Vec::new();
                        let mut keep_scores: Vec<u8> = Vec::new();
                        // for all 7 sets of 4

                        for _ in 0..7 {
                            let mut temp_set: Vec<u8> = Vec::new();
                            // roll the set of 4
                            for _ in 0..4 {
                                let d6_roll: u8 = thread_rng().gen_range(1..=6).clone();
                                print!("roll d6: {:?}", &d6_roll);
                                temp_set.push(d6_roll.clone());
                            }

                            println!("{:?}", &temp_set);
                            temp_set.sort();
                            let mut temp_set: Vec<u8> =
                                temp_set.clone().into_iter().rev().collect();
                            // drop the lowest roll forthis set of 4d6
                            temp_set.pop();
                            // then, push remaining 3 to kept sets
                            keep_sets.push(temp_set.clone());
                        }
                        println!("{:?}", &keep_sets);
                        // add up the scores fo each kept set, push to kept scores
                        for set in keep_sets {
                            let mut score: u8 = 0;
                            for roll in set {
                                score = score.clone() + roll;
                            }
                            keep_scores.push(score);
                        }

                        // killthe lowest score
                        keep_scores.sort();
                        let mut keep_scores: Vec<u8> =
                            keep_scores.clone().into_iter().rev().collect();
                        // drop the lowest roll forthis set of 7 scores
                        keep_scores.pop();
                        println!("{:?}", &keep_scores);

                        for score in keep_scores.clone() {
                            postmsg.push_str(&format!("{} ", score.to_string().clone()).clone());
                        }
                        // post the gotdam thing
                        println!("\n{} !stats:{}\n", &msg.author.name, &postmsg);
                        if let Err(why) =
                            msg.channel_id.say(&ctx.http, format!("{}", &postmsg)).await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    "flip" => {
                        let num: u8 = thread_rng().gen_range(0..=1);
                        let flip_result = match num {
                            0 => "Heads",
                            1 => "Tails",
                            _ => "ERROR. THIS SHOULD NOT HAVE HAPPENED WHAT THE FUCK",
                        };
                        if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, format!("{}", &flip_result))
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                        println!(
                            "{:?}\n{} flipped a coin: {}",
                            &msg.guild_id, &msg.author.name, &flip_result
                        );
                    }
                    // handle "not found" errors
                    _ => {
                        println!("error: not a command: {}", &command)
                    }
                }
            }
        }

        /*if msg.content[0..6].contains("!echo ") {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.

        }

        if msg.content[0..6].contains("!roll ")   {

            let num: u64 = thread_rng().gen_range(1..=(msg.content[6..].parse::<u64>()).unwrap().clone());
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("rolled a d{}: {}",&msg.content[6..],num)).await {
                println!("Error sending message: {:?}", why);
            }
        }

        else {}

        /*if msg.content[0..5].contains("!bonk") {

            //let num: u8 = thread_rng().gen_range(0..=1);

            if let Err(why) = msg.channel_id.say(&ctx.http, format!(
                "{}",match num {
                    0 => "Heads",
                    1 => "Tails",
                    _ => "ERROR. THIS SHOULD NOT HAVE HAPPENED WHAT THE FUCK"
                })
            ).await {
                println!("Error sending message: {:?}", why);
            }
        }*/*/
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
