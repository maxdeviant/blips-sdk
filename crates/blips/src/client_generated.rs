impl crate::BlipsClient {
    pub async fn board(
        &self,
        variables: crate::graphql::board::Variables,
    ) -> Result<crate::graphql::board::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Board>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn boards(
        &self,
        variables: crate::graphql::boards::Variables,
    ) -> Result<crate::graphql::boards::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Boards>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn container(
        &self,
        variables: crate::graphql::container::Variables,
    ) -> Result<crate::graphql::container::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Container>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn current_user(
        &self,
        variables: crate::graphql::current_user::Variables,
    ) -> Result<crate::graphql::current_user::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CurrentUser>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn diary(
        &self,
        variables: crate::graphql::diary::Variables,
    ) -> Result<crate::graphql::diary::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Diary>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn me(
        &self,
        variables: crate::graphql::me::Variables,
    ) -> Result<crate::graphql::me::ResponseData, reqwest::Error> {
        let response_body = self.post_graphql::<crate::graphql::Me>(variables).await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn note(
        &self,
        variables: crate::graphql::note::Variables,
    ) -> Result<crate::graphql::note::ResponseData, reqwest::Error> {
        let response_body = self.post_graphql::<crate::graphql::Note>(variables).await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn notes(
        &self,
        variables: crate::graphql::notes::Variables,
    ) -> Result<crate::graphql::notes::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Notes>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn project(
        &self,
        variables: crate::graphql::project::Variables,
    ) -> Result<crate::graphql::project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Project>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn project_columns(
        &self,
        variables: crate::graphql::project_columns::Variables,
    ) -> Result<crate::graphql::project_columns::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectColumns>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn projects(
        &self,
        variables: crate::graphql::projects::Variables,
    ) -> Result<crate::graphql::projects::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Projects>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn search(
        &self,
        variables: crate::graphql::search::Variables,
    ) -> Result<crate::graphql::search::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Search>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn tags(
        &self,
        variables: crate::graphql::tags::Variables,
    ) -> Result<crate::graphql::tags::ResponseData, reqwest::Error> {
        let response_body = self.post_graphql::<crate::graphql::Tags>(variables).await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn tasks(
        &self,
        variables: crate::graphql::tasks::Variables,
    ) -> Result<crate::graphql::tasks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Tasks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn archive_board(
        &self,
        variables: crate::graphql::archive_board::Variables,
    ) -> Result<crate::graphql::archive_board::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ArchiveBoard>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn complete_project(
        &self,
        variables: crate::graphql::complete_project::Variables,
    ) -> Result<crate::graphql::complete_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CompleteProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn complete_task(
        &self,
        variables: crate::graphql::complete_task::Variables,
    ) -> Result<crate::graphql::complete_task::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CompleteTask>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_board(
        &self,
        variables: crate::graphql::create_board::Variables,
    ) -> Result<crate::graphql::create_board::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateBoard>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_boards(
        &self,
        variables: crate::graphql::create_boards::Variables,
    ) -> Result<crate::graphql::create_boards::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateBoards>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_groups(
        &self,
        variables: crate::graphql::create_groups::Variables,
    ) -> Result<crate::graphql::create_groups::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateGroups>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_note(
        &self,
        variables: crate::graphql::create_note::Variables,
    ) -> Result<crate::graphql::create_note::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateNote>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_project(
        &self,
        variables: crate::graphql::create_project::Variables,
    ) -> Result<crate::graphql::create_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_project_column(
        &self,
        variables: crate::graphql::create_project_column::Variables,
    ) -> Result<crate::graphql::create_project_column::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateProjectColumn>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_projects(
        &self,
        variables: crate::graphql::create_projects::Variables,
    ) -> Result<crate::graphql::create_projects::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateProjects>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn create_tasks(
        &self,
        variables: crate::graphql::create_tasks::Variables,
    ) -> Result<crate::graphql::create_tasks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CreateTasks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn delete_board(
        &self,
        variables: crate::graphql::delete_board::Variables,
    ) -> Result<crate::graphql::delete_board::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DeleteBoard>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn delete_group(
        &self,
        variables: crate::graphql::delete_group::Variables,
    ) -> Result<crate::graphql::delete_group::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DeleteGroup>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn delete_note(
        &self,
        variables: crate::graphql::delete_note::Variables,
    ) -> Result<crate::graphql::delete_note::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DeleteNote>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn delete_project(
        &self,
        variables: crate::graphql::delete_project::Variables,
    ) -> Result<crate::graphql::delete_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DeleteProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn delete_task(
        &self,
        variables: crate::graphql::delete_task::Variables,
    ) -> Result<crate::graphql::delete_task::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DeleteTask>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn delete_tasks(
        &self,
        variables: crate::graphql::delete_tasks::Variables,
    ) -> Result<crate::graphql::delete_tasks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DeleteTasks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn enable_otp(
        &self,
        variables: crate::graphql::enable_otp::Variables,
    ) -> Result<crate::graphql::enable_otp::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::EnableOtp>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn generate_new_otp(
        &self,
        variables: crate::graphql::generate_new_otp::Variables,
    ) -> Result<crate::graphql::generate_new_otp::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::GenerateNewOtp>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn move_tasks(
        &self,
        variables: crate::graphql::move_tasks::Variables,
    ) -> Result<crate::graphql::move_tasks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::MoveTasks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn persist_group_order(
        &self,
        variables: crate::graphql::persist_group_order::Variables,
    ) -> Result<crate::graphql::persist_group_order::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PersistGroupOrder>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn persist_priority_order(
        &self,
        variables: crate::graphql::persist_priority_order::Variables,
    ) -> Result<crate::graphql::persist_priority_order::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PersistPriorityOrder>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn persist_project_column_order(
        &self,
        variables: crate::graphql::persist_project_column_order::Variables,
    ) -> Result<crate::graphql::persist_project_column_order::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PersistProjectColumnOrder>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn persist_project_order(
        &self,
        variables: crate::graphql::persist_project_order::Variables,
    ) -> Result<crate::graphql::persist_project_order::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PersistProjectOrder>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn persist_task_order(
        &self,
        variables: crate::graphql::persist_task_order::Variables,
    ) -> Result<crate::graphql::persist_task_order::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PersistTaskOrder>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn prioritize_tasks(
        &self,
        variables: crate::graphql::prioritize_tasks::Variables,
    ) -> Result<crate::graphql::prioritize_tasks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PrioritizeTasks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn register_user(
        &self,
        variables: crate::graphql::register_user::Variables,
    ) -> Result<crate::graphql::register_user::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::RegisterUser>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn spring_project(
        &self,
        variables: crate::graphql::spring_project::Variables,
    ) -> Result<crate::graphql::spring_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::SpringProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn tag_task(
        &self,
        variables: crate::graphql::tag_task::Variables,
    ) -> Result<crate::graphql::tag_task::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::TagTask>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn unarchive_board(
        &self,
        variables: crate::graphql::unarchive_board::Variables,
    ) -> Result<crate::graphql::unarchive_board::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UnarchiveBoard>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn uncomplete_project(
        &self,
        variables: crate::graphql::uncomplete_project::Variables,
    ) -> Result<crate::graphql::uncomplete_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UncompleteProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn uncomplete_task(
        &self,
        variables: crate::graphql::uncomplete_task::Variables,
    ) -> Result<crate::graphql::uncomplete_task::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UncompleteTask>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn unprioritize_tasks(
        &self,
        variables: crate::graphql::unprioritize_tasks::Variables,
    ) -> Result<crate::graphql::unprioritize_tasks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UnprioritizeTasks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn unspring_project(
        &self,
        variables: crate::graphql::unspring_project::Variables,
    ) -> Result<crate::graphql::unspring_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UnspringProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_board(
        &self,
        variables: crate::graphql::update_board::Variables,
    ) -> Result<crate::graphql::update_board::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateBoard>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_container(
        &self,
        variables: crate::graphql::update_container::Variables,
    ) -> Result<crate::graphql::update_container::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateContainer>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_diary(
        &self,
        variables: crate::graphql::update_diary::Variables,
    ) -> Result<crate::graphql::update_diary::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateDiary>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_group(
        &self,
        variables: crate::graphql::update_group::Variables,
    ) -> Result<crate::graphql::update_group::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateGroup>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_note(
        &self,
        variables: crate::graphql::update_note::Variables,
    ) -> Result<crate::graphql::update_note::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateNote>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_project(
        &self,
        variables: crate::graphql::update_project::Variables,
    ) -> Result<crate::graphql::update_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_project_column(
        &self,
        variables: crate::graphql::update_project_column::Variables,
    ) -> Result<crate::graphql::update_project_column::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateProjectColumn>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_task(
        &self,
        variables: crate::graphql::update_task::Variables,
    ) -> Result<crate::graphql::update_task::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateTask>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }

    pub async fn update_user_settings(
        &self,
        variables: crate::graphql::update_user_settings::Variables,
    ) -> Result<crate::graphql::update_user_settings::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UpdateUserSettings>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}
