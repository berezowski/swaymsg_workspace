use regex::Regex;
use swayipc::Workspace;

#[derive(Debug)]
pub struct WorkspaceDetails<'a> {
    pub number: u32,
    pub name: &'a str,
    pub basename: &'a str,
    // pub on_active_output: bool,
}

#[derive(Debug)]
pub struct Output<'a> {
    pub number: u32,
    pub name: &'a str,
    pub basename: &'a str,
}

impl WorkspaceDetails<'_> {
    fn cast_and_validate_fragments<'a>(
        fragments: (&'a str, &'a str),
        default_workspace_number: impl Fn() -> u32,
    ) -> (u32, &'a str) {
        let (number, name) = fragments;
        let number = match number.parse::<u32>() {
            Ok(number) => number,
            _ => default_workspace_number(),
        };
        let name = name.trim_start().trim_end();
        (number, name)
    }

    fn extract_fragments<'a>(wsname: &'a str) -> (&'a str, &'a str) {
        let capture_number_and_name = Regex::new(r"^(?<number>\d*)\s*(?<name>.*)").unwrap();
        let caps = capture_number_and_name.captures(&wsname).unwrap();
        (
            &wsname[..caps.get(1).unwrap().end()],
            &wsname[caps.get(2).unwrap().start()..],
        )
    }

    pub fn new<'a>(
        workspace: &Option<&&'a Workspace>,
        default_workspace_number: impl Fn() -> u32,
    ) -> Option<WorkspaceDetails<'a>> {
        match workspace {
            None => None,
            Some(workspace) => {
                let (number, name) = Self::cast_and_validate_fragments(
                    Self::extract_fragments(workspace.name.as_str()),
                    default_workspace_number,
                );
                Some(WorkspaceDetails {
                    number,
                    name,
                    basename: &workspace.name,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_workspace_inherits_details() {}
    #[test]
    fn new_workspace_without_a_numbert_gets_one_assigned() {}
    #[test]
    fn workspace_fragment_number_extraction_edgecases_work() {}
    #[test]
    fn workspace_fragment_name_extraction_edgecases_work() {}
    #[test]
    fn workspace_number_assignment_always_returns_the_highest_workspace_number_plus_one() {}
}
