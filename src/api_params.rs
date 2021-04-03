use crate::objects::*;

#[derive(Debug)]
pub enum ChatIdEnum {
    IsizeVariant(isize),
    StringVariant(String),
}

#[derive(Debug)]
pub enum ReplyMarkupEnum {
    InlineKeyboardMarkupVariant(InlineKeyboardMarkup),
    ReplyKeyboardMarkupVariant(ReplyKeyboardMarkup),
    ReplyKeyboardRemoveVariant(ReplyKeyboardRemove),
    ForceReplyVariant(ForceReply),
}

#[derive(Debug)]
pub enum FromChatIdEnum {
    IsizeVariant(isize),
    StringVariant(String),
}

#[derive(Debug)]
pub enum PhotoEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum AudioEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum DocumentEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum VideoEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum AnimationEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum VoiceEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum VideoNoteEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum MediaEnum {
    InputMediaAudioVariant(InputMediaAudio),
    InputMediaDocumentVariant(InputMediaDocument),
    InputMediaPhotoVariant(InputMediaPhoto),
    InputMediaVideoVariant(InputMediaVideo),
}

#[derive(Debug)]
pub enum StickerEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum PngStickerEnum {
    InputFileVariant(InputFile),
    StringVariant(String),
}

#[derive(Debug)]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudioVariant(InlineQueryResultCachedAudio),
    InlineQueryResultCachedDocumentVariant(InlineQueryResultCachedDocument),
    InlineQueryResultCachedGifVariant(InlineQueryResultCachedGif),
    InlineQueryResultCachedMpeg4GifVariant(InlineQueryResultCachedMpeg4Gif),
    InlineQueryResultCachedPhotoVariant(InlineQueryResultCachedPhoto),
    InlineQueryResultCachedStickerVariant(InlineQueryResultCachedSticker),
    InlineQueryResultCachedVideoVariant(InlineQueryResultCachedVideo),
    InlineQueryResultCachedVoiceVariant(InlineQueryResultCachedVoice),
    InlineQueryResultArticleVariant(InlineQueryResultArticle),
    InlineQueryResultAudioVariant(InlineQueryResultAudio),
    InlineQueryResultContactVariant(InlineQueryResultContact),
    InlineQueryResultGameVariant(InlineQueryResultGame),
    InlineQueryResultDocumentVariant(InlineQueryResultDocument),
    InlineQueryResultGifVariant(InlineQueryResultGif),
    InlineQueryResultLocationVariant(InlineQueryResultLocation),
    InlineQueryResultMpeg4GifVariant(InlineQueryResultMpeg4Gif),
    InlineQueryResultPhotoVariant(InlineQueryResultPhoto),
    InlineQueryResultVenueVariant(InlineQueryResultVenue),
    InlineQueryResultVideoVariant(InlineQueryResultVideo),
    InlineQueryResultVoiceVariant(InlineQueryResultVoice),
}

#[derive(Debug)]
pub enum InputMedia {
    InputMediaAnimationVariant(InputMediaAnimation),
    InputMediaDocumentVariant(InputMediaDocument),
    InputMediaAudioVariant(InputMediaAudio),
    InputMediaPhotoVariant(InputMediaPhoto),
    InputMediaVideoVariant(InputMediaVideo),
}

#[derive(Debug)]
pub enum PassportElementError {
    PassportElementErrorDataFieldVariant(PassportElementErrorDataField),
    PassportElementErrorFrontSideVariant(PassportElementErrorFrontSide),
    PassportElementErrorReverseSideVariant(PassportElementErrorReverseSide),
    PassportElementErrorSelfieVariant(PassportElementErrorSelfie),
    PassportElementErrorFileVariant(PassportElementErrorFile),
    PassportElementErrorFilesVariant(PassportElementErrorFiles),
    PassportElementErrorTranslationFileVariant(PassportElementErrorTranslationFile),
    PassportElementErrorTranslationFilesVariant(PassportElementErrorTranslationFiles),
    PassportElementErrorUnspecifiedVariant(PassportElementErrorUnspecified),
}

#[derive(Debug)]
pub struct GetUpdatesParams {
    offset: Option<isize>,
    limit: Option<isize>,
    timeout: Option<isize>,
    allowed_updates: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct SetWebhookParams {
    url: String,
    certificate: Option<InputFile>,
    ip_address: Option<String>,
    max_connections: Option<isize>,
    allowed_updates: Option<Vec<String>>,
    drop_pending_updates: Option<bool>,
}

#[derive(Debug)]
pub struct DeleteWebhookParams {
    drop_pending_updates: Option<bool>,
}

#[derive(Debug)]
pub struct SendMessageParams {
    chat_id: ChatIdEnum,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct ForwardMessageParams {
    chat_id: ChatIdEnum,
    from_chat_id: FromChatIdEnum,
    disable_notification: Option<bool>,
    message_id: isize,
}

#[derive(Debug)]
pub struct CopyMessageParams {
    chat_id: ChatIdEnum,
    from_chat_id: FromChatIdEnum,
    message_id: isize,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendPhotoParams {
    chat_id: ChatIdEnum,
    photo: PhotoEnum,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendAudioParams {
    chat_id: ChatIdEnum,
    audio: AudioEnum,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<isize>,
    performer: Option<String>,
    title: Option<String>,
    thumb: Option<ThumbEnum>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendDocumentParams {
    chat_id: ChatIdEnum,
    document: DocumentEnum,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_content_type_detection: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendVideoParams {
    chat_id: ChatIdEnum,
    video: VideoEnum,
    duration: Option<isize>,
    width: Option<isize>,
    height: Option<isize>,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    supports_streaming: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendAnimationParams {
    chat_id: ChatIdEnum,
    animation: AnimationEnum,
    duration: Option<isize>,
    width: Option<isize>,
    height: Option<isize>,
    thumb: Option<ThumbEnum>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendVoiceParams {
    chat_id: ChatIdEnum,
    voice: VoiceEnum,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    duration: Option<isize>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendVideoNoteParams {
    chat_id: ChatIdEnum,
    video_note: VideoNoteEnum,
    duration: Option<isize>,
    length: Option<isize>,
    thumb: Option<ThumbEnum>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendMediaGroupParams {
    chat_id: ChatIdEnum,
    media: Vec<MediaEnum>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
}

#[derive(Debug)]
pub struct SendLocationParams {
    chat_id: ChatIdEnum,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<isize>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct EditMessageLiveLocationParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    heading: Option<isize>,
    proximity_alert_radius: Option<isize>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct StopMessageLiveLocationParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct SendVenueParams {
    chat_id: ChatIdEnum,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendContactParams {
    chat_id: ChatIdEnum,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendPollParams {
    chat_id: ChatIdEnum,
    question: String,
    options: Vec<String>,
    is_anonymous: Option<bool>,
    type_field: Option<String>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<isize>,
    explanation: Option<String>,
    explanation_parse_mode: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<isize>,
    close_date: Option<isize>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendDiceParams {
    chat_id: ChatIdEnum,
    emoji: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct SendChatActionParams {
    chat_id: ChatIdEnum,
    action: String,
}

#[derive(Debug)]
pub struct GetUserProfilePhotosParams {
    user_id: isize,
    offset: Option<isize>,
    limit: Option<isize>,
}

#[derive(Debug)]
pub struct GetFileParams {
    file_id: String,
}

#[derive(Debug)]
pub struct KickChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    until_date: Option<isize>,
    revoke_messages: Option<bool>,
}

#[derive(Debug)]
pub struct UnbanChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    only_if_banned: Option<bool>,
}

#[derive(Debug)]
pub struct RestrictChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    permissions: ChatPermissions,
    until_date: Option<isize>,
}

#[derive(Debug)]
pub struct PromoteChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    is_anonymous: Option<bool>,
    can_manage_chat: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_manage_voice_chats: Option<bool>,
    can_restrict_members: Option<bool>,
    can_promote_members: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
}

#[derive(Debug)]
pub struct SetChatAdministratorCustomTitleParams {
    chat_id: ChatIdEnum,
    user_id: isize,
    custom_title: String,
}

#[derive(Debug)]
pub struct SetChatPermissionsParams {
    chat_id: ChatIdEnum,
    permissions: ChatPermissions,
}

#[derive(Debug)]
pub struct ExportChatInviteLinkParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct CreateChatInviteLinkParams {
    chat_id: ChatIdEnum,
    expire_date: Option<isize>,
    member_limit: Option<isize>,
}

#[derive(Debug)]
pub struct EditChatInviteLinkParams {
    chat_id: ChatIdEnum,
    invite_link: String,
    expire_date: Option<isize>,
    member_limit: Option<isize>,
}

#[derive(Debug)]
pub struct RevokeChatInviteLinkParams {
    chat_id: ChatIdEnum,
    invite_link: String,
}

#[derive(Debug)]
pub struct SetChatPhotoParams {
    chat_id: ChatIdEnum,
    photo: InputFile,
}

#[derive(Debug)]
pub struct DeleteChatPhotoParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct SetChatTitleParams {
    chat_id: ChatIdEnum,
    title: String,
}

#[derive(Debug)]
pub struct SetChatDescriptionParams {
    chat_id: ChatIdEnum,
    description: Option<String>,
}

#[derive(Debug)]
pub struct PinChatMessageParams {
    chat_id: ChatIdEnum,
    message_id: isize,
    disable_notification: Option<bool>,
}

#[derive(Debug)]
pub struct UnpinChatMessageParams {
    chat_id: ChatIdEnum,
    message_id: Option<isize>,
}

#[derive(Debug)]
pub struct UnpinAllChatMessagesParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct LeaveChatParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct GetChatParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct GetChatAdministratorsParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct GetChatMembersCountParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct GetChatMemberParams {
    chat_id: ChatIdEnum,
    user_id: isize,
}

#[derive(Debug)]
pub struct SetChatStickerSetParams {
    chat_id: ChatIdEnum,
    sticker_set_name: String,
}

#[derive(Debug)]
pub struct DeleteChatStickerSetParams {
    chat_id: ChatIdEnum,
}

#[derive(Debug)]
pub struct AnswerCallbackQueryParams {
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<isize>,
}

#[derive(Debug)]
pub struct SetMyCommandsParams {
    commands: Vec<BotCommand>,
}

#[derive(Debug)]
pub struct EditMessageTextParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct EditMessageCaptionParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct EditMessageMediaParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct EditMessageReplyMarkupParams {
    chat_id: Option<ChatIdEnum>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct StopPollParams {
    chat_id: ChatIdEnum,
    message_id: isize,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct DeleteMessageParams {
    chat_id: ChatIdEnum,
    message_id: isize,
}

#[derive(Debug)]
pub struct SendStickerParams {
    chat_id: ChatIdEnum,
    sticker: StickerEnum,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkupEnum>,
}

#[derive(Debug)]
pub struct GetStickerSetParams {
    name: String,
}

#[derive(Debug)]
pub struct UploadStickerFileParams {
    user_id: isize,
    png_sticker: InputFile,
}

#[derive(Debug)]
pub struct CreateNewStickerSetParams {
    user_id: isize,
    name: String,
    title: String,
    png_sticker: Option<PngStickerEnum>,
    tgs_sticker: Option<InputFile>,
    emojis: String,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}

#[derive(Debug)]
pub struct AddStickerToSetParams {
    user_id: isize,
    name: String,
    png_sticker: Option<PngStickerEnum>,
    tgs_sticker: Option<InputFile>,
    emojis: String,
    mask_position: Option<MaskPosition>,
}

#[derive(Debug)]
pub struct SetStickerPositionInSetParams {
    sticker: String,
    position: isize,
}

#[derive(Debug)]
pub struct DeleteStickerFromSetParams {
    sticker: String,
}

#[derive(Debug)]
pub struct SetStickerSetThumbParams {
    name: String,
    user_id: isize,
    thumb: Option<ThumbEnum>,
}

#[derive(Debug)]
pub struct AnswerInlineQueryParams {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    cache_time: Option<isize>,
    is_personal: Option<bool>,
    next_offset: Option<String>,
    switch_pm_text: Option<String>,
    switch_pm_parameter: Option<String>,
}

#[derive(Debug)]
pub struct SendInvoiceParams {
    chat_id: isize,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    start_parameter: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<isize>,
    photo_width: Option<isize>,
    photo_height: Option<isize>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct AnswerShippingQueryParams {
    shipping_query_id: String,
    ok: bool,
    shipping_options: Option<Vec<ShippingOption>>,
    error_message: Option<String>,
}

#[derive(Debug)]
pub struct AnswerPreCheckoutQueryParams {
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}

#[derive(Debug)]
pub struct SetPassportDataErrorsParams {
    user_id: isize,
    errors: Vec<PassportElementError>,
}

#[derive(Debug)]
pub struct SendGameParams {
    chat_id: isize,
    game_short_name: String,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<isize>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug)]
pub struct SetGameScoreParams {
    user_id: isize,
    score: isize,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<isize>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
}

#[derive(Debug)]
pub struct GetGameHighScoresParams {
    user_id: isize,
    chat_id: Option<isize>,
    message_id: Option<isize>,
    inline_message_id: Option<String>,
}
