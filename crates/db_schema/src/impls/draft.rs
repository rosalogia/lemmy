use crate::{
  newtypes::DraftId,
  schema::draft::dsl::{
    draft,
  },
  source::draft::{
    Draft,
    DraftInsertForm,
    DraftUpdateForm,
  },
  traits::Crud,
  utils::{get_conn, DbPool},
};
use diesel::{dsl::insert_into, result::Error, QueryDsl};
use diesel_async::RunQueryDsl;

#[async_trait]
impl Crud for Draft {
  type InsertForm = DraftInsertForm;
  type UpdateForm = DraftUpdateForm;
  type IdType = DraftId;
  async fn read(pool: &DbPool, draft_id: DraftId) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    draft.find(draft_id).first::<Self>(conn).await
  }

  async fn delete(pool: &DbPool, draft_id: DraftId) -> Result<usize, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(draft.find(draft_id)).execute(conn).await
  }

  async fn create(pool: &DbPool, form: &Self::InsertForm) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    insert_into(draft)
      .values(form)
      .get_result::<Self>(conn)
      .await
  }

  async fn update(
    pool: &DbPool,
    draft_id: DraftId,
    new_draft: &Self::UpdateForm,
  ) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(draft.find(draft_id))
      .set(new_draft)
      .get_result::<Self>(conn)
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    source::{
      community::{Community, CommunityInsertForm},
      instance::Instance,
      person::{Person, PersonInsertForm},
      draft::{
        Draft,
        DraftInsertForm,
        DraftUpdateForm,
      },
    },
    traits::Crud,
    utils::build_db_pool_for_tests,
  };
  use serial_test::serial;

  #[tokio::test]
  #[serial]
  async fn test_crud() {
    let pool = &build_db_pool_for_tests().await;

    let inserted_instance = Instance::read_or_create(pool, "my_domain.tld".to_string())
      .await
      .unwrap();

    let new_person = PersonInsertForm::builder()
      .name("jim".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_person = Person::create(pool, &new_person).await.unwrap();

    let new_community = CommunityInsertForm::builder()
      .name("test community_3".to_string())
      .title("nada".to_owned())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_community = Community::create(pool, &new_community).await.unwrap();

    let new_post = DraftInsertForm::builder()
      .name("A test post".into())
      .creator_id(inserted_person.id)
      .community_id(inserted_community.id)
      .build();

    let inserted_draft = Draft::create(pool, &new_post).await.unwrap();

    let expected_draft = Draft {
      id: inserted_draft.id,
      name: "A test post".into(),
      url: None,
      body: None,
      creator_id: inserted_person.id,
      community_id: inserted_community.id,
      nsfw: false,
      embed_title: None,
      embed_description: None,
      embed_video_url: None,
      thumbnail_url: None,
      language_id: Default::default(),
    };

    let read_draft = Draft::read(pool, inserted_draft.id).await.unwrap();

    let new_draft_update = DraftUpdateForm::builder()
      .name(Some("A test post".into()))
      .build();
    let updated_draft = Draft::update(pool, inserted_draft.id, &new_draft_update)
      .await
      .unwrap();

    let num_deleted = Draft::delete(pool, inserted_draft.id).await.unwrap();
    Community::delete(pool, inserted_community.id)
      .await
      .unwrap();
    Person::delete(pool, inserted_person.id).await.unwrap();
    Instance::delete(pool, inserted_instance.id).await.unwrap();

    assert_eq!(expected_draft, read_draft);
    assert_eq!(expected_draft, inserted_draft);
    assert_eq!(expected_draft, updated_draft);
    assert_eq!(1, num_deleted);
  }
}
