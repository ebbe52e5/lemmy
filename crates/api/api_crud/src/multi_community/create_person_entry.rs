use super::check_multi_community_creator;
use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{context::LemmyContext, utils::check_local_user_banned_or_deleted};
use lemmy_db_schema::source::{
  multi_community::{MultiCommunity, MultiCommunityPersonEntry, MultiCommunityPersonEntryForm},
  person::Person,
};
use lemmy_db_views_community::api::CreateOrDeleteMultiCommunityPersonEntry;
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_person::{PersonView, api::PersonResponse};
use lemmy_diesel_utils::traits::Crud;
use lemmy_utils::error::{LemmyErrorType, LemmyResult};

/// Add a local person to a multi-community, so their posts show in its feed. Only in the
/// zhifou.io Lemmy fork: the entry isn't federated, so remote instances only see the
/// multi-community's communities.
pub async fn create_multi_community_person_entry(
  Json(data): Json<CreateOrDeleteMultiCommunityPersonEntry>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<PersonResponse>> {
  check_local_user_banned_or_deleted(&local_user_view)?;

  let multi = MultiCommunity::read(&mut context.pool(), data.id).await?;
  check_multi_community_creator(&multi, &local_user_view)?;

  let person = Person::read(&mut context.pool(), data.person_id).await?;
  if !person.local {
    return Err(LemmyErrorType::MultiCommunityPersonNotLocal.into());
  }
  if person.deleted {
    return Err(LemmyErrorType::NotFound.into());
  }

  MultiCommunityPersonEntry::check_entry_limit(&mut context.pool(), data.id).await?;

  let form = MultiCommunityPersonEntryForm::new(data.id, person.id);
  MultiCommunityPersonEntry::create(&mut context.pool(), &form).await?;

  let person_view = PersonView::read(
    &mut context.pool(),
    person.id,
    Some(local_user_view.person.id),
    local_user_view.person.instance_id,
    false,
  )
  .await?;
  Ok(Json(PersonResponse { person_view }))
}
