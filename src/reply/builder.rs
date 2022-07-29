//! The builder to create a new reply

use crate::serenity_prelude as serenity;

/// Message builder that abstracts over prefix and application command responses
#[derive(Default, Clone)]
pub struct CreateReply<'att> {
    /// Message content.
    pub content: Option<String>,
    /// Embeds, if present.
    pub embeds: Vec<serenity::CreateEmbed>,
    /// Message attachments.
    pub attachments: Vec<serenity::AttachmentType<'att>>,
    /// Whether the message is ephemeral (only has an effect in application commands)
    pub ephemeral: Option<bool>,
    /// Message components, that is, buttons and select menus.
    pub components: Option<serenity::CreateComponents>,
    /// The allowed mentions for the message.
    pub allowed_mentions: Option<serenity::CreateAllowedMentions>,
    /// The reference message this message is a reply to.
    pub reference_message: Option<serenity::MessageReference>,
}

impl<'att> CreateReply<'att> {
    /// Set the content of the message.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Adds an embed to the message.
    ///
    /// Existing embeds are kept.
    pub fn embed(mut self, embed: serenity::CreateEmbed) -> Self {
        self.embeds.push(embed);
        self
    }

    /// Set components (buttons and select menus) for this message.
    ///
    /// Any previously set components will be overwritten.
    pub fn components(mut self, components: serenity::CreateComponents) -> Self {
        self.components = Some(components);
        self
    }

    /// Add an attachment.
    ///
    /// This will not have an effect in a slash command's initial response!
    pub fn attachment(mut self, attachment: serenity::AttachmentType<'att>) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Toggles whether the message is an ephemeral response (only invoking user can see it).
    ///
    /// This only has an effect in slash commands!
    ///
    /// If this is the initial response and this response
    /// has previously been deferred, the ephemerality is decided by the defer operation. I.e.
    /// if you deferred the response without enabling ephemeral, the initial response will not be
    /// ephemeral.
    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = Some(ephemeral);
        self
    }

    /// Set the allowed mentions for the message.
    ///
    /// See [`serenity::CreateAllowedMentions`] for more information.
    pub fn allowed_mentions(mut self, allowed_mentions: serenity::CreateAllowedMentions) -> Self {
        self.allowed_mentions = Some(allowed_mentions);
        self
    }

    /// Set the reference message this message is a reply to.
    pub fn reference_message(mut self, reference: impl Into<serenity::MessageReference>) -> Self {
        self.reference_message = Some(reference.into());
        self
    }
}

/// Methods to create a message builder from any type from this [`CreateReply`]. Used by poise
/// internally to actually send a response to Discord
impl<'att> CreateReply<'att> {
    /// Serialize this response builder to a [`serenity::CreateInteractionResponseData`]
    pub fn to_slash_initial_response(self) -> serenity::CreateInteractionResponseData<'att> {
        let crate::CreateReply {
            content,
            embeds,
            attachments,
            components,
            ephemeral,
            allowed_mentions,
            reference_message: _, // can't reply to a message in interactions
        } = self;

        let mut builder = serenity::CreateInteractionResponseData::default();
        if let Some(content) = content {
            builder = builder.content(content);
        }
        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions);
        }
        if let Some(components) = components {
            builder = builder.components(components);
        }

        builder
            .embeds(embeds)
            .add_files(attachments)
            .ephemeral(ephemeral.unwrap_or_default())
    }

    /// Serialize this response builder to a [`serenity::CreateInteractionResponseFollowup`]
    pub fn to_slash_followup_response(self) -> serenity::CreateInteractionResponseFollowup<'att> {
        let crate::CreateReply {
            content,
            embeds,
            attachments,
            components,
            ephemeral,
            allowed_mentions,
            reference_message: _,
        } = self;

        let mut builder = serenity::CreateInteractionResponseFollowup::default();
        if let Some(content) = content {
            builder = builder.content(content);
        }
        if let Some(components) = components {
            builder = builder.components(components)
        }
        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions)
        }

        builder
            .embeds(embeds)
            .add_files(attachments)
            .ephemeral(ephemeral.unwrap_or_default())
    }

    /// Serialize this response builder to a [`serenity::EditInteractionResponse`]
    pub fn to_slash_initial_response_edit(self) -> serenity::EditInteractionResponse {
        let crate::CreateReply {
            content,
            embeds,
            attachments: _, // no support for attachment edits in serenity yet
            components,
            ephemeral: _, // can't edit ephemerality in retrospect
            allowed_mentions,
            reference_message: _,
        } = self;

        let mut builder = serenity::EditInteractionResponse::default();
        if let Some(content) = content {
            builder = builder.content(content);
        }
        if let Some(components) = components {
            builder = builder.components(components)
        }
        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions)
        }

        builder.embeds(embeds)
    }

    /// Serialize this response builder to a [`serenity::EditMessage`]
    pub fn to_prefix_edit(self) -> serenity::EditMessage<'att> {
        let crate::CreateReply {
            content,
            embeds,
            attachments,
            components,
            ephemeral: _, // not supported in prefix
            allowed_mentions,
            reference_message: _, // can't edit reference message afterwards
        } = self;

        let mut builder = serenity::EditMessage::default();
        for attachment in attachments {
            builder = builder.attachment(attachment);
        }

        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions);
        }

        builder
            // Empty string resets content (happens when user replaces text with embed)
            .content(content.unwrap_or_default())
            // When components is None, this will still be run to reset the components.
            .components(components.unwrap_or_default())
            .embeds(embeds)
    }

    /// Serialize this response builder to a [`serenity::CreateMessage`]
    pub fn to_prefix(self) -> serenity::CreateMessage<'att> {
        let crate::CreateReply {
            content,
            embeds,
            attachments,
            components,
            ephemeral: _, // not supported in prefix
            allowed_mentions,
            reference_message,
        } = self;

        let mut builder = serenity::CreateMessage::default();
        if let Some(content) = content {
            builder = builder.content(content);
        }
        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions);
        }
        if let Some(components) = components {
            builder = builder.components(components)
        }
        if let Some(reference_message) = reference_message {
            builder = builder.reference_message(reference_message);
        }

        builder.embeds(embeds).add_files(attachments)
    }
}
