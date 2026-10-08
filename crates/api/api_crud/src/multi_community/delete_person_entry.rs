use super::check_multi_community_creator;
use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{context::LemmyContext, utils::check_local_user_banned_or_deleted};
use lemmy_db_schema::source::multi_community::{
  MultiCommunity,
  MultiCommunityPersonEntry,
  MultiCommunityPersonEntryForm,
};
use lemmy_db_views_community::api::CreateOrDeleteMultiCommunityPersonEntry;
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_site::api::SuccessResponse;
use lemmy_diesel_utils::traits::Crud;
use lemmy_utils::error::LemmyResult;

/// Remove a person from a multi-community (zhifou.io Lemmy fork, not federated).
pub async fn delete_multi_community_person_entry(
  Json(data): Json<CreateOrDeleteMultiCommunityPersonEntry>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<SuccessResponse>> {
  check_local_user_banned_or_deleted(&local_user_view)?;

  let multi = MultiCommunity::read(&mut context.pool(), data.id).await?;
  check_multi_community_creator(&multi, &local_user_view)?;

  let form = MultiCommunityPersonEntryForm::new(data.id, data.person_id);
  MultiCommunityPersonEntry::delete(&mut context.pool(), &form).await?;

  Ok(Json(SuccessResponse::default()))
}
