use actix_web::web::{Data, Json};
use lemmy_api_utils::{context::LemmyContext, utils::check_local_user_banned_or_deleted};
use lemmy_db_schema::source::person::{PersonFollow, PersonFollowForm};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_person::{
  PersonView,
  api::{FollowPerson, PersonResponse},
};
use lemmy_utils::error::{LemmyErrorType, LemmyResult};

/// Follow or unfollow a person. This is local only and isn't federated.
///
/// Unfollowing keeps the follow row and only marks it inactive.
pub async fn follow_person(
  Json(data): Json<FollowPerson>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<PersonResponse>> {
  check_local_user_banned_or_deleted(&local_user_view)?;
  let target_id = data.person_id;
  let my_person_id = local_user_view.person.id;
  let local_instance_id = local_user_view.person.instance_id;

  if target_id == my_person_id {
    return Err(LemmyErrorType::CantFollowYourself.into());
  }

  if data.follow {
    let form = PersonFollowForm::new(my_person_id, target_id);
    PersonFollow::follow(&mut context.pool(), &form).await?;
  } else {
    PersonFollow::unfollow(&mut context.pool(), my_person_id, target_id).await?;
  }

  let person_view = PersonView::read(
    &mut context.pool(),
    target_id,
    Some(my_person_id),
    local_instance_id,
    false,
  )
  .await?;
  Ok(Json(PersonResponse { person_view }))
}
