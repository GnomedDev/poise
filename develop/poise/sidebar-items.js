initSidebarItems({"attr":[["command","This macro transforms plain functions into poise bot commands."]],"derive":[["SlashChoiceParameter",""]],"enum":[["ApplicationCommand","A view into a leaf of an application command tree. Not an owned type!"],["ApplicationCommandOrAutocompleteInteraction","Abstracts over a refernce to an application command interaction or autocomplete interaction"],["ApplicationCommandTree","Defines any application command, including subcommands if supported by the application command type"],["CodeBlockError","Error that can be returned from parsing a [`CodeBlock`] ([`CodeBlock::pop_from`])"],["CommandErrorContext","Context of an error in user code"],["CommandErrorLocation","Used for command errors to store the specific operation in a command’s execution where an error occured"],["CommandRef","A reference to either a prefix or application command."],["Context","Wrapper around either [`crate::ApplicationContext`] or [`crate::PrefixContext`]"],["ContextMenuCommandAction","Possible actions that a context menu entry can have"],["ErrorContext","Contains the location of the error with location-specific context"],["Event","This enum stores every possible event that a [`serenity::EventHandler`] can receive."],["MaybeEmptyError","When attempting to parse a string, it can fail either because it’s empty, or because it’s invalid in some way. This error type covers both cases"],["Prefix","Possible ways to define a command prefix"],["ReplyHandle","Returned from [`send_reply`] to retrieve the sent message object."],["SlashArgError","Possible errors when parsing slash command arguments"],["SlashCommandMeta","A single slash command or slash command group"]],"fn":[["dispatch_message","Manually dispatches a message with the prefix framework."],["say_reply","Shorthand of [`send_reply`] for text-only messages"],["send_application_reply","Send a response to an interaction (slash command or context menu command invocation)."],["send_prefix_reply","Prefix-specific reply function. For more details, see [`crate::send_reply`]."],["send_reply","Send a message in the given context: normal message if prefix command, interaction response if application command."]],"macro":[["parse_prefix_args","Macro for parsing an argument string into multiple parameter types."],["parse_prefix_args","Macro for parsing an argument string into multiple parameter types."],["parse_slash_args","Macro for extracting and parsing slash command arguments out of an array of [`serenity::ApplicationCommandInteractionDataOption`]."],["parse_slash_args","Macro for extracting and parsing slash command arguments out of an array of [`serenity::ApplicationCommandInteractionDataOption`]."]],"mod":[["builtins","Building blocks for common commands like help commands or application command registration"],["samples","See [`builtins`]"],["serenity_prelude","This module re-exports a bunch of items from all over serenity. Useful if you can’t remember the full paths of serenity items."]],"struct":[["ApplicationCommandErrorContext","Application command specific context to an error in user code"],["ApplicationCommandOptions","Application command specific configuration of a framework command"],["ApplicationContext","Application command specific context passed to command invocations."],["ApplicationFrameworkOptions","Application command specific configuration for the framework"],["ArgString","Type used throughout the prefix parameter parsing code in this code to store the raw string input."],["ArgumentParseError","The error type returned from [parse_prefix_args!]. It contains a `Box<dyn Error>`"],["AutocompleteChoice","A single autocomplete choice, displayed in Discord UI"],["CodeBlock","A command parameter type for Discord code blocks"],["CommandBuilder","Builder struct to add a command to the framework"],["CommandDefinition","Type returned from `#[poise::command]` annotated functions, which contains all of the generated prefix and application commands"],["CommandDefinitionRef","A view into a command definition with its different implementations"],["CommandId","This struct holds all data shared across different command types of the same implementation."],["ContextMenuCommand","Fully defines a context menu command in the framework"],["CooldownConfig","Configuration struct for [`Cooldowns`]"],["Cooldowns","Handles cooldowns for a single command"],["CreateReply","Message builder that abstracts over prefix and application command responses"],["EditTracker","Stores messages and the associated bot responses in order to implement poise’s edit tracking feature."],["EmptyArgs","Error type when trying to parse a string parameter but the input is empty"],["EventWrapper","A [`serenity::EventHandler`] implementation that wraps every received event into the [`Event`] enum and propagates it to a callback."],["Framework","The main framework struct which stores all data and handles message and interaction dispatch."],["FrameworkBuilder","A builder to configure and run a framework."],["FrameworkOptions","Framework configuration"],["InvalidChoice","Emitted when the user enters a string that is not recognized by a SlashChoiceParameter-derived enum"],["KeyValueArgs","A command parameter type for key-value args"],["PartialContext","Trimmed down, more general version of [`Context`]"],["PrefixCommand","Definition of a single command, excluding metadata which doesn’t affect the command itself such as category."],["PrefixCommandErrorContext","Context passed alongside the error value to error handlers"],["PrefixCommandMeta","Includes a command, plus metadata like associated sub-commands or category."],["PrefixContext","Prefix-specific context passed to command invocations."],["PrefixFrameworkOptions","Prefix-specific framework configuration"],["SlashCommand","Fully defines a single slash command in the framework"],["SlashCommandParameter","A single parameter of a slash command"],["TooManyArguments","Error thrown if user passes too many arguments to a command"]],"trait":[["Autocompletable","Types that can be marked autocompletable in a slash command parameter."],["ContextMenuParameter","Implemented for all types that can be used in a context menu command"],["PopArgument","Superset of [`PopArgumentAsync`] without Discord context available and no async support."],["PopArgumentAsync","Parse a value out of a string by popping off the front of the string. Discord message context is available for parsing, and IO may be done as part of the parsing."],["SlashArgument","Implement this trait on types that you want to use as a slash command parameter."]],"type":[["BoxFuture","Shorthand for a wrapped async future with a lifetime, used by many parts of this framework."]]});