use std::{fs, io::{Read, Write}, path::Path};

use liquid::Object;

pub fn template_files_recursively(project_root: &Path, template_root: &Path, current_dir: &Path, globals: &Object) -> crate::Result<()> {
    if current_dir.is_dir() {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                template_files_recursively(project_root, template_root, &path, globals)?;
            } else if path.is_file() {
                let relative_path = path.strip_prefix(template_root).ok().unwrap();
                let destination_path = project_root.join(relative_path);
                let mut contents = fs::read_to_string(path).unwrap();
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(&destination_path.clone())?;
                
                let template = liquid::ParserBuilder::with_stdlib()
                    .build().unwrap()
                    .parse(&contents)
                    .unwrap();

                contents = template.render(&globals).unwrap();

                file.write_all(contents.as_bytes())?;
            }
        }
    }

    Ok(())
}
