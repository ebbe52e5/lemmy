use activitypub_federation::config::Data;
use lemmy_api_utils::{
  context::LemmyContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_multi_community_creator,
};
use lemmy_db_schema::source::{multi_community::MultiCommunity, person::Person};
use lemmy_utils::error::LemmyResult;

pub mod create;
pub mod create_entry;
// zhifou.io Lemmy fork: people as multi-community entries
pub mod create_person_entry;
pub mod delete_entry;
pub mod delete_person_entry;
pub mod list;
pub mod update;

fn send_federation_update(
  multi: MultiCommunity,
  person: Person,
  context: &Data<LemmyContext>,
) -> LemmyResult<()> {
  ActivityChannel::submit_activity(
    SendActivityData::UpdateMultiCommunity(multi, person),
    context,
  )
}
