use std::{env, fs, path::Path};

use crate::Result;

use super::common::template_files_recursively;

pub async fn game(identifier: &String, name: &String, template: &Option<String>) -> Result<()> {
    let template_key = template.clone().unwrap_or("hello-world".to_owned());
    
    let template_path = format!("src/templates/{template_key}");

    let template_root_path = env::current_exe().ok().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join(&template_path);
    let project_root_path = env::current_dir().ok().unwrap();
    
    let globals = liquid::object!({
        "identifier": identifier,
        "name": name
    });

    template_files_recursively(project_root_path.as_path(), &template_root_path, &template_root_path, &globals)?;

    Ok(())
}
