use anyhow::Result;
use tinyverse_lib::{TinyverseHomeSource, resolve_tinyverse_paths};
use tinyverse_ui::{ActionLine, DetailSection, LabeledField, Panel, Tone, default_stdout_context};

pub fn execute() -> Result<()> {
    let paths = resolve_tinyverse_paths(None)?;
    let source = format_source(&paths.source);

    let context = default_stdout_context();

    let details = DetailSection::new("Resolved Paths")
        .with_field(LabeledField::new(
            "Home Directory",
            paths.home_dir.display().to_string(),
        ))
        .with_field(LabeledField::new(
            "Database",
            paths.db_path.display().to_string(),
        ))
        .with_field(LabeledField::new("Source", source))
        .render(&context);

    let header = ActionLine::new("INFO", "Resolved TinyVerse paths", Tone::Info).render(&context);

    let output = Panel::new([header, String::new(), details].join("\n"))
        .with_title("TinyVerse: Path")
        .with_tone(Tone::Info)
        .render(&context);

    println!("{output}");
    Ok(())
}

fn format_source(source: &TinyverseHomeSource) -> &'static str {
    match source {
        TinyverseHomeSource::ArgOverride => "Argument Override",
        TinyverseHomeSource::EnvOverride => "Environment Override",
        TinyverseHomeSource::RepoLocal => "Repo Local",
        TinyverseHomeSource::CwdLocal => "CWD Local",
        TinyverseHomeSource::UserHome => "User Home",
    }
}
