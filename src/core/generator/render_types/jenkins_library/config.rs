use std::path::Path;

use crate::generator::model_command::ExecutionModel;

pub struct JenkinsLibraryConfig<'cfg> {
    pub project_path: &'cfg Path,
    pub output_dir: &'cfg Path,
    pub package_name: &'cfg str,
    pub execution_model: ExecutionModel,
    pub json_output: bool,
    pub stubs: bool,
}

impl<'cfg> JenkinsLibraryConfig<'cfg> {
    pub fn from(
        project_path: &'cfg Path,
        output_dir: &'cfg Path,
        package_name: &'cfg str,
        execution_model: ExecutionModel,
        json_output: bool,
        stubs: bool,
    ) -> JenkinsLibraryConfig<'cfg> {
        JenkinsLibraryConfig {
            project_path,
            output_dir,
            package_name,
            execution_model,
            json_output,
            stubs,
        }
    }
}
