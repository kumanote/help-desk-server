use crate::{
    model::{
        Agent, AgentId, AgentName, AgentRoles, Email, Group, GroupDescription, GroupId,
        GroupMember, GroupName, GroupRoles, HashedPassword, Locale, Role, RoleForGroup,
        RoleForGroupId, RoleForGroupName, RoleId, RoleName, Scope, ScopeForGroup, Workspace,
        WorkspaceId, WorkspaceName,
    },
    repository::{
        AgentRepository, GroupRepository, RoleForGroupRepository, RoleRepository,
        WorkspaceRepository,
    },
    Error, Result,
};
use chrono::Utc;
use logger::prelude::*;
use std::str::FromStr;

pub struct InitWorkspaceUseCaseInput {
    pub workspace_name: String,
    pub first_agent_email: String,
    pub first_agent_password: String,
    pub first_agent_name: String,
    pub first_agent_locale: Locale,
}

pub type InitWorkspaceUseCaseOutput = Workspace;

pub trait InitWorkspaceUseCase: Send + Sync + 'static {
    type Transaction;
    type WorkspaceRepository: WorkspaceRepository<Err = Error, Transaction = Self::Transaction>;
    type AgentRepository: AgentRepository<Err = Error, Transaction = Self::Transaction>;
    type GroupRepository: GroupRepository<Err = Error, Transaction = Self::Transaction>;
    type RoleRepository: RoleRepository<Err = Error, Transaction = Self::Transaction>;
    type RoleForGroupRepository: RoleForGroupRepository<
        Err = Error,
        Transaction = Self::Transaction,
    >;

    /// initialize workspace.
    /// * create workspace.
    /// * initialize predefined roles.
    /// * create first agent.
    /// * create admin group and let first agent be an owner of the group.
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: InitWorkspaceUseCaseInput,
    ) -> Result<InitWorkspaceUseCaseOutput>;
}

pub struct InitWorkspaceUseCaseImpl<
    WR: WorkspaceRepository<Err = Error>,
    AR: AgentRepository<Err = Error>,
    GR: GroupRepository<Err = Error>,
    RR: RoleRepository<Err = Error>,
    RGR: RoleForGroupRepository<Err = Error>,
> {
    workspace_repository: WR,
    agent_repository: AR,
    group_repository: GR,
    role_repository: RR,
    role_for_group_repository: RGR,
}

impl<
        WR: WorkspaceRepository<Err = Error>,
        AR: AgentRepository<Err = Error>,
        GR: GroupRepository<Err = Error>,
        RR: RoleRepository<Err = Error>,
        RGR: RoleForGroupRepository<Err = Error>,
    > InitWorkspaceUseCaseImpl<WR, AR, GR, RR, RGR>
{
    pub fn new(
        workspace_repository: WR,
        agent_repository: AR,
        group_repository: GR,
        role_repository: RR,
        role_for_group_repository: RGR,
    ) -> Self {
        Self {
            workspace_repository,
            agent_repository,
            group_repository,
            role_repository,
            role_for_group_repository,
        }
    }
}

impl<
        TX,
        WR: WorkspaceRepository<Err = Error, Transaction = TX>,
        AR: AgentRepository<Err = Error, Transaction = TX>,
        GR: GroupRepository<Err = Error, Transaction = TX>,
        RR: RoleRepository<Err = Error, Transaction = TX>,
        RGR: RoleForGroupRepository<Err = Error, Transaction = TX>,
    > InitWorkspaceUseCase for InitWorkspaceUseCaseImpl<WR, AR, GR, RR, RGR>
{
    type Transaction = TX;
    type WorkspaceRepository = WR;
    type AgentRepository = AR;
    type GroupRepository = GR;
    type RoleRepository = RR;
    type RoleForGroupRepository = RGR;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: InitWorkspaceUseCaseInput,
    ) -> Result<InitWorkspaceUseCaseOutput> {
        // check if there is no workspace in the DB.
        if self.workspace_repository.get(tx)?.is_some() {
            warn!("Initialize workspace process has been requested...although the system has an initialized workspace.");
            return Err(Error::InvalidRequest);
        }

        // init workspace
        let workspace = Workspace {
            id: WorkspaceId::generate(),
            name: WorkspaceName::from_str(&params.workspace_name)?,
            created_at: Utc::now().naive_utc(),
        };
        self.workspace_repository.create(tx, &workspace)?;

        // init predefined roles
        let predefined_roles = create_predefined_roles();
        for role in &predefined_roles {
            self.role_repository.create(tx, role)?;
        }
        let predefined_roles_for_group = create_predefined_roles_for_group();
        for role in &predefined_roles_for_group {
            self.role_for_group_repository.create(tx, role)?;
        }

        // create the first agent.
        let agent_id = AgentId::generate();
        let email = Email::from_str(&params.first_agent_email)?;
        let hashed_password = HashedPassword::new_from_plain_text(&params.first_agent_password)?;
        let name = AgentName::from_str(&params.first_agent_name)?;
        let first_agent = Agent {
            id: agent_id,
            email,
            hashed_password,
            name,
            locale: params.first_agent_locale,
            is_active: true,
        };
        self.agent_repository.create(tx, &first_agent)?;

        // grant profile role to the first agent
        let mut first_agent_roles = AgentRoles::new(first_agent.id.clone());
        let profile_role = predefined_roles
            .iter()
            .find(|role| role.has_scope(&Scope::Profile))
            .expect("predefined role must contain profile role.");
        first_agent_roles.add_role(profile_role.clone());
        self.role_repository
            .update_agent_roles(tx, &first_agent_roles)?;

        // create admin group
        let admin_group = Group {
            id: GroupId::generate(),
            name: GroupName::from("Admin Group".to_owned()),
            description: GroupDescription::from(None),
            created_at: Utc::now().naive_utc(),
        };
        self.group_repository.create(tx, &admin_group)?;

        // grant admin role to admin group
        let mut admin_group_roles = GroupRoles::new(admin_group.id.clone());
        let admin_role = predefined_roles
            .iter()
            .find(|role| role.has_scope(&Scope::AdminWorkspace))
            .expect("predefined role must contain admin role.");
        admin_group_roles.add_role(admin_role.clone());
        self.role_repository
            .update_group_roles(tx, &admin_group_roles)?;

        // let the first agent be the member of admin group
        let group_owner_role = predefined_roles_for_group
            .iter()
            .find(|role| role.scope == ScopeForGroup::GroupOwner)
            .expect("predefined role must contain group owner role.");
        let group_member = GroupMember::new(
            admin_group.id.clone(),
            first_agent.clone(),
            group_owner_role.clone(),
        );
        self.group_repository
            .create_group_member(tx, &group_member)?;
        Ok(workspace)
    }
}

fn create_predefined_roles() -> Vec<Role> {
    vec![
        Role {
            id: RoleId::generate(),
            name: RoleName::from("Profile".to_owned()),
            scopes: vec![Scope::Profile].into_iter().collect(),
        },
        Role {
            id: RoleId::generate(),
            name: RoleName::from("Admin".to_owned()),
            scopes: vec![
                Scope::AdminFaq,
                Scope::AdminAnnouncement,
                Scope::AdminInquiry,
                Scope::AdminWorkspace,
            ]
            .into_iter()
            .collect(),
        },
        Role {
            id: RoleId::generate(),
            name: RoleName::from("Editor".to_owned()),
            scopes: vec![
                Scope::WriteFaq,
                Scope::WriteAnnouncement,
                Scope::WriteInquiry,
            ]
            .into_iter()
            .collect(),
        },
        Role {
            id: RoleId::generate(),
            name: RoleName::from("Readonly".to_owned()),
            scopes: vec![Scope::ReadFaq, Scope::ReadAnnouncement, Scope::ReadInquiry]
                .into_iter()
                .collect(),
        },
    ]
}

fn create_predefined_roles_for_group() -> Vec<RoleForGroup> {
    vec![
        RoleForGroup {
            id: RoleForGroupId::generate(),
            name: RoleForGroupName::from("Group member".to_owned()),
            scope: ScopeForGroup::GroupMember,
        },
        RoleForGroup {
            id: RoleForGroupId::generate(),
            name: RoleForGroupName::from("Group admin".to_owned()),
            scope: ScopeForGroup::GroupAdmin,
        },
        RoleForGroup {
            id: RoleForGroupId::generate(),
            name: RoleForGroupName::from("Group owner".to_owned()),
            scope: ScopeForGroup::GroupOwner,
        },
    ]
}
