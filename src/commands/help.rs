use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::Context,
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument.\n"]
#[command_not_found_text = "Could not find: `{}`."]
#[strikethrough_commands_tip_in_guild = ""]
#[max_levenshtein_distance(3)]
#[lacking_conditions = "Hide"]
pub async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;

    Ok(())
}
