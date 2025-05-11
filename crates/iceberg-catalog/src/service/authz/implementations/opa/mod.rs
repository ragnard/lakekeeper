mod client;

use async_trait::async_trait;
use client::{Client, OPAInput};
use iceberg_ext::catalog::rest::{ErrorModel, IcebergErrorResponse};
use axum::Router;
use utoipa::OpenApi;

use crate:: {
    api::{iceberg::v1::Result, ApiContext},
    request_metadata::RequestMetadata,
    service::{
        authn::UserId,
        authz::{
            Authorizer, CatalogNamespaceAction, CatalogProjectAction, CatalogRoleAction,
            CatalogServerAction, CatalogTableAction, CatalogUserAction, CatalogViewAction,
            CatalogWarehouseAction, ListProjectsResponse, NamespaceParent,
            implementations::{Authorizers},
        },
        health::{Health, HealthExt},
        Actor, Catalog, NamespaceIdentUuid, ProjectId, RoleId, SecretStore, State, TableIdentUuid,
        ViewIdentUuid, WarehouseIdent,
    },
};

pub(crate) type OPAResult<T> = Result<T, OPAError>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum OPAError {
}

impl From<OPAError> for ErrorModel {
    fn from(_err: OPAError) -> Self {
        ErrorModel::default()
    }
}

impl From<OPAError> for IcebergErrorResponse {
    fn from(err: OPAError) -> Self {
        let err_model = ErrorModel::from(err);
        err_model.into()
    }
}


pub(crate) async fn new_authorizer_from_config() -> OPAResult<Authorizers> {
    Ok(Authorizers::OPA(OPAAuthorizer{
        client: Client::new()
    }))
}

pub type UnauthenticatedOPAAuthorizer = OPAAuthorizer;

#[derive(Clone, Debug)]
pub struct OPAAuthorizer {
    client: Client,
    // health: Arc<RwLock<Vec<Health>>>,
}

#[async_trait]
impl HealthExt for OPAAuthorizer {
    async fn health(&self) -> Vec<Health> {
        vec![]
    }
    async fn update_health(&self) {
        // Do nothing
    }
}


#[derive(Debug, OpenApi)]
#[openapi()]
pub(super) struct ApiDoc;


#[async_trait]
impl Authorizer for OPAAuthorizer {
    fn api_doc() -> utoipa::openapi::OpenApi {
        ApiDoc::openapi()
    }

    fn new_router<C: Catalog, S: SecretStore>(&self) -> Router<ApiContext<State<Self, C, S>>> {
        Router::new()
    }

    async fn check_actor(&self, _actor: &Actor) -> Result<()> {
        Ok(())
    }

    async fn can_bootstrap(&self, _metadata: &RequestMetadata) -> Result<()> {
        Ok(())
    }

    async fn bootstrap(&self, _metadata: &RequestMetadata, _is_operator: bool) -> Result<()> {
        Ok(())
    }

    async fn list_projects(&self, _metadata: &RequestMetadata) -> Result<ListProjectsResponse> {
        Ok(ListProjectsResponse::All)
    }

    async fn can_search_users(&self, _metadata: &RequestMetadata) -> Result<bool> {
        Ok(true)
    }

    async fn is_allowed_user_action(
        &self,
        _metadata: &RequestMetadata,
        _user_id: &UserId,
        _action: CatalogUserAction,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn is_allowed_role_action(
        &self,
        _metadata: &RequestMetadata,
        _role_id: RoleId,
        _action: CatalogRoleAction,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn is_allowed_server_action(
        &self,
        _metadata: &RequestMetadata,
        _action: CatalogServerAction,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn is_allowed_project_action(
        &self,
        metadata: &RequestMetadata,
        _project_id: &ProjectId,
        _action: CatalogProjectAction,
    ) -> Result<bool> {
        let input = OPAInput{
            actor: metadata.actor().into(),
        };

        self.client.check(input).await
    }

    async fn is_allowed_warehouse_action(
        &self,
        metadata: &RequestMetadata,
        _warehouse_id: WarehouseIdent,
        _action: CatalogWarehouseAction,
    ) -> Result<bool> {
        let input = OPAInput{
            actor: metadata.actor().into(),
        };

        self.client.check(input).await
    }

    async fn is_allowed_namespace_action<A>(
        &self,
        _metadata: &RequestMetadata,
        _namespace_id: NamespaceIdentUuid,
        _action: A,
    ) -> Result<bool>
    where
        A: From<CatalogNamespaceAction> + std::fmt::Display + Send,
    {
        Ok(true)
    }

    async fn is_allowed_table_action<A>(
        &self,
        _metadata: &RequestMetadata,
        _table_id: TableIdentUuid,
        _action: A,
    ) -> Result<bool>
    where
        A: From<CatalogTableAction> + std::fmt::Display + Send,
    {
        Ok(true)
    }

    async fn is_allowed_view_action<A>(
        &self,
        _metadata: &RequestMetadata,
        _view_id: ViewIdentUuid,
        _action: A,
    ) -> Result<bool>
    where
        A: From<CatalogViewAction> + std::fmt::Display + Send,
    {
        Ok(true)
    }

    async fn delete_user(&self, _metadata: &RequestMetadata, _user_id: UserId) -> Result<()> {
        Ok(())
    }

    async fn create_role(
        &self,
        _metadata: &RequestMetadata,
        _role_id: RoleId,
        _parent_project_id: ProjectId,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_role(&self, _metadata: &RequestMetadata, _role_id: RoleId) -> Result<()> {
        Ok(())
    }

    async fn create_project(
        &self,
        _metadata: &RequestMetadata,
        _project_id: &ProjectId,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_project(
        &self,
        _metadata: &RequestMetadata,
        _project_id: ProjectId,
    ) -> Result<()> {
        Ok(())
    }

    async fn create_warehouse(
        &self,
        _metadata: &RequestMetadata,
        _warehouse_id: WarehouseIdent,
        _parent_project_id: &ProjectId,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_warehouse(
        &self,
        _metadata: &RequestMetadata,
        _warehouse_id: WarehouseIdent,
    ) -> Result<()> {
        Ok(())
    }

    async fn create_namespace(
        &self,
        _metadata: &RequestMetadata,
        _namespace_id: NamespaceIdentUuid,
        _parent: NamespaceParent,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_namespace(
        &self,
        _metadata: &RequestMetadata,
        _namespace_id: NamespaceIdentUuid,
    ) -> Result<()> {
        Ok(())
    }

    async fn create_table(
        &self,
        _metadata: &RequestMetadata,
        _table_id: TableIdentUuid,
        _parent: NamespaceIdentUuid,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_table(&self, _table_id: TableIdentUuid) -> Result<()> {
        Ok(())
    }

    async fn create_view(
        &self,
        _metadata: &RequestMetadata,
        _view_id: ViewIdentUuid,
        _parent: NamespaceIdentUuid,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_view(&self, _view_id: ViewIdentUuid) -> Result<()> {
        Ok(())
    }
}
