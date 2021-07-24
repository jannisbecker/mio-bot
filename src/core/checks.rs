use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::channel::Message,
};

#[check]
#[name = "is_nsfw"]
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
#[name = "is_admin"]
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
#[name = "is_mod"]
async fn mod_check(
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

    match perms.manage_roles() {
        true => Ok(()),
        false => Err(Reason::User(
            "This command can only be run as a moderator".to_string(),
        )),
    }
}
