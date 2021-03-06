use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AssociatedPermissions {
    pub associatedPermissions: Permissions,
}

#[allow(dead_code)]
impl Into<String> for AssociatedPermissions {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable,Debug, Clone)]
pub struct PermissionsBody {
    pub canAccessIntegration: Option<String>,
    pub canCreateProjectDashboard: Option<String>,
    pub canManageComment: Option<String>,
    pub canExecute: Option<String>,
    pub canInitData: Option<String>,
    pub canManageIntegration: Option<String>,
    pub canCreateExecutionContext: Option<String>,
    pub canManageFolder: Option<String>,
    pub canInviteUserToProject: Option<String>,
    pub canCreateTableDataLoad: Option<String>,
    pub canCreateDomain: Option<String>,
    pub canSeeOtherUserDetails: Option<String>,
    pub canManageACL: Option<String>,
    pub canCreateRole: Option<String>,
    pub canCreateProjectTemplates: Option<String>,
    pub canCreateHelp: Option<String>,
    pub canManageDomain: Option<String>,
    pub canManageAttributeLabel: Option<String>,
    pub canCreateColumn: Option<String>,
    pub canManageReport: Option<String>,
    pub canManageDataSet: Option<String>,
    pub canSetUserVariables: Option<String>,
    pub canCreateAttributeGroup: Option<String>,
    pub canSetStyle: Option<String>,
    pub canValidateProject: Option<String>,
    pub canMaintainProject: Option<String>,
    pub canCreateETLFile: Option<String>,
    pub canCreateScheduledMail: Option<String>,
    pub canSuspendUserFromProject: Option<String>,
    pub canManageIsProduction: Option<String>,
    pub canMaintainUserFilterRelation: Option<String>,
    pub canManageAttribute: Option<String>,
    pub canManageReportDefinition: Option<String>,
    pub canCreateReport: Option<String>,
    pub canMaintainUserFilter: Option<String>,
    pub canSetLocale: Option<String>,
    pub canEnrichData: Option<String>,
    pub canUploadNonProductionCSV: Option<String>,
    pub canManageETLFile: Option<String>,
    pub canCreateComment: Option<String>,
    pub canCreateDataSet: Option<String>,
    pub canManageExecutionContext: Option<String>,
    pub canCreateTable: Option<String>,
    pub canManageTableDataLoad: Option<String>,
    pub canCreateMetric: Option<String>,
    pub canRefreshData: Option<String>,
    pub canManageFilterSettings: Option<String>,
    pub canManageProjectDashboard: Option<String>,
    pub canManageProject: Option<String>,
    pub canManagePrompt: Option<String>,
    pub canAccessWorkbench: Option<String>,
    pub canCreateAttributeLabel: Option<String>,
    pub canManageColumn: Option<String>,
    pub canCreatePrompt: Option<String>,
    pub canManagePublicAccessCode: Option<String>,
    pub canListUsersInProject: Option<String>,
    pub canManageAttributeGroup: Option<String>,
    pub canManageMetric: Option<String>,
    pub canManageHelp: Option<String>,
    pub canManageTable: Option<String>,
    pub canSetProjectVariables: Option<String>,
    pub canCreateFolder: Option<String>,
    pub canManageFact: Option<String>,
    pub canListInvitationsInProject: Option<String>,
    pub canManageScheduledMail: Option<String>,
    pub canCreateFilterSettings: Option<String>,
    pub canExecuteRaw: Option<String>,
    pub canSeePublicAccessCode: Option<String>,
    pub canCreateReportDefinition: Option<String>,
    pub canCreateFact: Option<String>,
    pub canCreateAttribute: Option<String>,
    pub canAssignUserWithRole: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Permissions {
    permissions: PermissionsBody,
}

#[allow(dead_code)]
impl Into<String> for Permissions {
    fn into(self) -> String {
        json::as_pretty_json(&self).to_string()
    }
}
