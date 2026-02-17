use anyhow::Result;
use tinyverse_ui::{
    ActionLine, Panel, StripeMode, StyledTable, SummaryFooter, Tone, default_stdout_context,
};

use crate::providers;

pub fn execute() -> Result<()> {
    let context = default_stdout_context();
    let all = providers::all();

    let mut table =
        StyledTable::new(vec!["KEY", "NAME", "COMMAND"]).with_stripe_mode(StripeMode::DimEvenRows);

    for provider in all {
        let metadata = provider.metadata();
        table = table.with_row(vec![
            metadata.key.to_owned(),
            metadata.name.to_owned(),
            provider.launch_command_template().to_owned(),
        ]);
    }

    let table_panel = Panel::new(table.render(&context))
        .with_title("Provider Table")
        .with_tone(Tone::Info)
        .render(&context);

    let outer = Panel::new(
        [
            ActionLine::new(
                "INFO",
                format!("Found {} provider(s)", all.len()),
                Tone::Info,
            )
            .render(&context),
            String::new(),
            table_panel,
            String::new(),
            SummaryFooter::new(format!("{} provider(s) available", all.len())).render(&context),
        ]
        .join("\n"),
    )
    .with_title("tinyverse providers")
    .with_tone(Tone::Info)
    .render(&context);

    println!("{outer}");
    Ok(())
}
