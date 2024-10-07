use std::{fs, io::{Read, Write}, borrow::Cow, env, path::{Path, PathBuf}};
use rust_embed::Embed;

use crate::Result;

use liquid::Object;

#[derive(Embed)]
#[folder = "src/templates"]
struct Templates;

pub async fn game(identifier: &String, name: &String, template: &Option<String>) -> Result<()> {
    let template_key = template.clone().unwrap_or("hello-world".to_owned());
    
    let paths = Templates::iter()
        .filter(|p| p.starts_with(&template_key))
        .collect::<Vec<_>>();

    let project_root_path = env::current_dir().ok().unwrap();
    
    let globals = liquid::object!({
        "identifier": identifier,
        "name": name
    });

    template_files(&template_key, project_root_path.as_path(), paths, &globals)?;

    Ok(())
}


pub fn template_files(template_key: &str, project_root: &Path, paths: Vec<Cow<'static, str>>, globals: &Object) -> crate::Result<()> {
    for path in paths {
        let contents = Templates::get(path.as_ref()).unwrap();
        let relative_path = path.strip_prefix(&format!("{template_key}/")).unwrap();
        let destination_path = project_root.join(PathBuf::from(relative_path));
        let mut file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&destination_path.clone())?;
        
        let template = liquid::ParserBuilder::with_stdlib()
            .build().unwrap()
            .parse(&std::str::from_utf8(&contents.data).unwrap())
            .unwrap();

        let contents = template.render(&globals).unwrap();

        file.write_all(contents.as_bytes())?;
    }

    Ok(())
}
