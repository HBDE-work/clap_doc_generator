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
