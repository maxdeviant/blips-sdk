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
}
