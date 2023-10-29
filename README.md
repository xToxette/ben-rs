# ben-rs
Discord bot for the w0rre discord server.

Uses [poise](https://github.com/serenity-rs/poise) for handling user request and [songbird](https://github.com/serenity-rs/songbird) for music functionalities.

# contributing
To contribute to developing the bot, you'll need to install the rust programming language.

 - [rust lang](https://www.rust-lang.org/tools/install)

## Getting a discord bot
To run the discord bot, you'll need to register a discord bot with rights to create application commands. To create a discord bot, go to the [Discord Developer Portal](https://discord.com/developers/applications), and create a new application. 

When you've created the bot, go to the 'Bot' tab and toggle all Privileged Gateway Intents to on. Now go to the OAuth2 tab and under 'Default Authorization Link' select 'In-app Authorization', with the scopes 'bot' and 'applications.commands', and with 'Administrator' bot permissions.

Now invite the bot to test discord server by going to the 'OAuth2' tab, URL Generator, selecting the scopes 'bot' and 'applications.commands', and selecting the 'Administrator' bot permissions. At the bottom you'll find a link that you can use to invite the bot to your server.

In the .env.example file, fill in the `DISCORD_TOKEN` environment variable using your discord bot's token and rename it to .env
