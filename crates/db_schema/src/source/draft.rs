use crate::newtypes::{CommunityId, DbUrl, LanguageId, PersonId, DraftId};
#[cfg(feature = "full")]
use crate::schema::draft;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[cfg(feature = "full")]
use ts_rs::TS;
use typed_builder::TypedBuilder;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = draft))]
#[cfg_attr(feature = "full", ts(export))]
/// An unpublished draft of a post.
pub struct Draft {
  pub id: DraftId,
  pub name: String,
  #[cfg_attr(feature = "full", ts(type = "string"))]
  /// An optional link / url for the post.
  pub url: Option<DbUrl>,
  /// An optional post body, in markdown.
  pub body: Option<String>,
  pub creator_id: PersonId,
  pub community_id: CommunityId,
  /// Whether the post is NSFW.
  pub nsfw: bool,
  /// A title for the link.
  pub embed_title: Option<String>,
  /// A description for the link.
  pub embed_description: Option<String>,
  #[cfg_attr(feature = "full", ts(type = "string"))]
  /// A thumbnail picture url.
  pub thumbnail_url: Option<DbUrl>,
  #[cfg_attr(feature = "full", ts(type = "string"))]
  /// A video url for the link.
  pub embed_video_url: Option<DbUrl>,
  pub language_id: LanguageId,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = draft))]
pub struct DraftInsertForm {
  #[builder(!default)]
  pub name: String,
  #[builder(!default)]
  pub creator_id: PersonId,
  #[builder(!default)]
  pub community_id: CommunityId,
  pub nsfw: Option<bool>,
  pub url: Option<DbUrl>,
  pub body: Option<String>,
  pub embed_title: Option<String>,
  pub embed_description: Option<String>,
  pub embed_video_url: Option<DbUrl>,
  pub thumbnail_url: Option<DbUrl>,
  pub language_id: Option<LanguageId>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = draft))]
pub struct DraftUpdateForm {
  pub name: Option<String>,
  pub nsfw: Option<bool>,
  pub url: Option<Option<DbUrl>>,
  pub body: Option<Option<String>>,
  pub embed_title: Option<Option<String>>,
  pub embed_description: Option<Option<String>>,
  pub embed_video_url: Option<Option<DbUrl>>,
  pub thumbnail_url: Option<Option<DbUrl>>,
  pub language_id: Option<LanguageId>,
}