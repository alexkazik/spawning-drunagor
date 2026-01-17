use std::fs;
use std::path::{Path, PathBuf};
use yew_bootstrap::icons::BIFiles;

fn main() -> Result<(), std::io::Error> {
    let staging_dir_path = PathBuf::from(
        std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
    );

    // copy bootstrap icons
    let path = staging_dir_path.join(BIFiles::NAME);
    if !path.is_dir() {
        fs::create_dir(&path)?;
    }
    BIFiles::copy(&path)?;

    // update the index.html
    let path = staging_dir_path.join("index.html");
    let index = fs::read_to_string(&path)?;
    let index = index
        .replace(
            "<!include-bootstrap-icons>",
            &format!(
                r#"<link rel="stylesheet" href="{}{}/bootstrap-icons.css"/>"#,
                parse_base_href(&index),
                BIFiles::NAME
            ),
        )
        .replace("<!version>", env!("CARGO_PKG_VERSION"));
    fs::write(&path, index)?;

    // copy all miniature images
    copy_miniatures(&staging_dir_path)?;

    Ok(())
}

fn parse_base_href(html: &str) -> &str {
    html.split_once("<base href=\"")
        .and_then(|html| html.1.split_once("\" />"))
        .map(|html| html.0)
        .filter(|html| {
            html.ends_with('/')
                && !html.contains('\'')
                && !html.contains('"')
                && !html.contains('&')
        })
        .map_or("", |html| html)
}

fn copy_miniatures(staging_dir_path: &Path) -> Result<(), std::io::Error> {
    let output_path = staging_dir_path.join("miniature");
    if !output_path.is_dir() {
        fs::create_dir(&output_path)?;
    }

    let source_dir_path =
        std::env::var("TRUNK_SOURCE_DIR").expect("Environment variable TRUNK_SOURCE_DIR");
    let input_path = Path::new(&source_dir_path).join("static").join("miniature");

    for (src, dst) in generated::MONSTER_IMAGES {
        fs::copy(input_path.join(src), output_path.join(dst))?;
    }

    Ok(())
}

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated_monster_image.rs"));
}
