#[derive(strum::FromRepr, Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Permission {
    CreateInvite,
    KickMembers,
    BanMembers,
    Admin,
    ManageChannels,
    ManageGuild,
    AddReactions,
    ViewAuditLog,
    PrioritySpeaker,
    Stream,
    ViewChannel,
    SendMessages,
    SendTTSMessages,
    ManageMessages,
    EmbedLinks,
    AttachFiles,
    ReadMessageHistory,
    MentionEveryone,
    UseExternalEmojis,
    ViewGuildInsights,
    Connect,
    Speak,
    MuteMembers,
    DeafenMembers,
    MoveMembers,
    UseVad,
    ChangeNickName,
    ManageNickNames,
    ManageRoles,
    ManageWebHooks,
    ManageEmojiAndStickers,
    UseSlashCommands,
    RequestToSpeak,
    ManageEvents,
    ManageThreads,
    CreatePublicThreads,
    CreatePrivateThreads,
    UseExternalStickers,
    SendMessageInThreads,
    UseEmbeddedActivities,
    ModerateMembers,
    Size,
}
use Permission::*;
impl Permission {
    /**
     * Gets all permissions that the specified bitfield represents
     *
     * ```
     * use disrust::permissions::Permission::{*, self};
     *
     * let bitfield = 0b00000000000000000000000000000000000111111;
     * let permissions = Permission::get_permissions(bitfield);
     * assert!(permissions == vec! {
     *    CreateInvite,
     *    KickMembers,
     *    BanMembers,
     *    Admin,
     *    ManageChannels,
     *    ManageGuild,
     * });
     * ```
     */
    pub fn get_permissions(bitfield: u64) -> Vec<Permission> {
        let mut result = vec![];
        for i in 0..Size as u8 {
            if bitfield & (1 << i) > 0 {
                result.push(Permission::from_repr(i).unwrap());
            }
        }
        result
    }
}
