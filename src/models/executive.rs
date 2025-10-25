#[derive(Clone, Debug, serde::Deserialize)]
pub struct ExecutiveProject {
    pub project_id: String,
    pub account_executive: String,
}

pub fn get_list_of_executives(projects: &Vec<ExecutiveProject>) -> Vec<String> {
    let mut executives = Vec::new();
    for project in projects {
        if !executives.contains(&project.account_executive) {
            executives.push(project.account_executive.clone());
        }
    }
    executives
}

pub fn get_projects_by_executive(
    executive: String,
    projects: &Vec<ExecutiveProject>,
) -> Vec<String> {
    let mut project_ids = Vec::new();
    for project in projects {
        if project.account_executive == executive {
            project_ids.push(project.project_id.clone());
        }
    }
    project_ids
}
