use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::channel::Message,
};

#[check]
#[name = "NSFW"]
async fn nsfw_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    match msg.channel_id.to_channel(&ctx).await.unwrap().is_nsfw() {
        true => Ok(()),
        false => Err(Reason::User(
            "This command can only be used in nsfw-enabled channels".to_string(),
        )),
    }
}

#[check]
#[name = "Admin"]
async fn admin_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let member = msg.member(&ctx).await.expect("can't get member");
    let perms = member
        .permissions(&ctx)
        .await
        .expect("can't get permissions");

    match perms.administrator() {
        true => Ok(()),
        false => Err(Reason::User(
            "This command can only be run as an administrator".to_string(),
        )),
    }
}

#[check]
#[name = "Moderator"]
async fn mod_check(
    ctx: &Context,
    msg: &Message,
    args: &mut Args,
    options: &CommandOptions,
) -> Result<(), Reason> {
    let member = msg.member(&ctx).await.expect("can't get member");
    let roles = member.roles(&ctx).await.expect("can't get roles");

    if admin_check(ctx, msg, args, options).await.is_ok() {
        return Ok(());
    }

    // todo: allow admins to set a mod role, store it in a database, and check against that
    match roles
        .iter()
        .any(|role| role.id.to_string() == "134040353517862912")
    {
        true => Ok(()),
        false => Err(Reason::User(
            "This command can only be run as a moderator".to_string(),
        )),
    }
}
