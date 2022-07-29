//! All functions to actually send a reply

use crate::serenity_prelude as serenity;

/// Send a message in the given context: normal message if prefix command, interaction response
/// if application command.
///
/// If you just want to send a string, use [`say_reply`].
///
/// Note: panics when called in an autocomplete context!
///
/// ```rust,no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let ctx: poise::Context<'_, (), ()> = todo!();
/// ctx.send(|f| f
///     .content("Works for slash and prefix commands")
///     .embed(|f| f
///         .title("Much versatile, very wow")
///         .description("I need more documentation ok?")
///     )
///     .ephemeral(true) // this one only applies in application commands though
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn send_reply<'ctx, 'att, U, E>(
    ctx: crate::Context<'ctx, U, E>,
    builder: crate::CreateReply<'att>,
) -> Result<crate::ReplyHandle<'ctx>, serenity::Error> {
    Ok(match ctx {
        crate::Context::Prefix(ctx) => {
            crate::ReplyHandle::Known(crate::send_prefix_reply(ctx, builder).await?)
        }
        crate::Context::Application(ctx) => crate::send_application_reply(ctx, builder).await?,
    })
}

/// Shorthand of [`send_reply`] for text-only messages
///
/// Note: panics when called in an autocomplete context!
pub async fn say_reply<U, E>(
    ctx: crate::Context<'_, U, E>,
    text: impl Into<String>,
) -> Result<crate::ReplyHandle<'_>, serenity::Error> {
    send_reply(ctx, crate::CreateReply::default().content(text.into())).await
}

/// Send a response to an interaction (slash command or context menu command invocation).
///
/// If a response to this interaction has already been sent, a
/// [followup](serenity::ApplicationCommandInteraction::create_followup_message) is sent.
///
/// No-op if autocomplete context
pub async fn send_application_reply<'ctx, 'att, U, E>(
    ctx: crate::ApplicationContext<'ctx, U, E>,
    mut reply: crate::CreateReply<'att>,
) -> Result<crate::ReplyHandle<'ctx>, serenity::Error> {
    if reply.ephemeral.is_none() {
        reply.ephemeral = Some(ctx.command.ephemeral);
    }

    if reply.allowed_mentions.is_none() {
        reply.allowed_mentions = ctx.framework.options.allowed_mentions.clone()
    }

    let interaction = match ctx.interaction {
        crate::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(x) => x,
        crate::ApplicationCommandOrAutocompleteInteraction::Autocomplete(_) => {
            return Ok(crate::ReplyHandle::Autocomplete)
        }
    };

    if let Some(callback) = ctx.framework.options().reply_callback {
        callback(ctx.into(), &mut reply);
    }

    let has_sent_initial_response = ctx
        .has_sent_initial_response
        .load(std::sync::atomic::Ordering::SeqCst);

    Ok(if has_sent_initial_response {
        crate::ReplyHandle::Known(Box::new(
            interaction
                .create_followup_message(ctx.discord, reply.to_slash_followup_response())
                .await?,
        ))
    } else {
        interaction
            .create_interaction_response(
                ctx.discord,
                serenity::CreateInteractionResponse::default()
                    .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(reply.to_slash_initial_response()),
            )
            .await?;
        ctx.has_sent_initial_response
            .store(true, std::sync::atomic::Ordering::SeqCst);

        crate::ReplyHandle::Unknown {
            http: &ctx.discord.http,
            interaction,
        }
    })
}

/// Prefix-specific reply function. For more details, see [`crate::send_reply`].
pub async fn send_prefix_reply<'att, U, E>(
    ctx: crate::PrefixContext<'_, U, E>,
    mut reply: crate::CreateReply<'att>,
) -> Result<Box<serenity::Message>, serenity::Error> {
    if reply.ephemeral.is_none() {
        reply.ephemeral = Some(ctx.command.ephemeral);
    }

    if reply.allowed_mentions.is_none() {
        reply.allowed_mentions = ctx.framework.options.allowed_mentions.clone()
    }

    if let Some(callback) = ctx.framework.options().reply_callback {
        callback(ctx.into(), &mut reply);
    }

    // This must only return None when we _actually_ want to reuse the existing response! There are
    // no checks later
    let lock_edit_tracker = || {
        if ctx.command.reuse_response {
            if let Some(edit_tracker) = &ctx.framework.options().prefix_options.edit_tracker {
                return Some(edit_tracker.write().unwrap());
            }
        }
        None
    };

    let existing_response = lock_edit_tracker()
        .as_mut()
        .and_then(|t| t.find_bot_response(ctx.msg.id))
        .cloned();

    Ok(Box::new(if let Some(mut response) = existing_response {
        response.edit(ctx.discord, reply.to_prefix_edit()).await?;

        // If the entry still exists after the await, update it to the new contents
        if let Some(mut edit_tracker) = lock_edit_tracker() {
            edit_tracker.set_bot_response(ctx.msg, response.clone());
        }

        response
    } else {
        let new_response = ctx
            .msg
            .channel_id
            .send_message(ctx.discord, reply.to_prefix())
            .await?;
        if let Some(track_edits) = &mut lock_edit_tracker() {
            track_edits.set_bot_response(ctx.msg, new_response.clone());
        }

        new_response
    }))
}
