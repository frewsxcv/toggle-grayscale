use std::process;
use sysbar;

// https://gist.github.com/silesky/2da3a6bef1703e38d901ec9c623eecbd
static SCRIPT: &str = r#"
tell application "System Preferences"
        activate
        set the current pane to pane id "com.apple.preference.universalaccess"
        delay 1 #needs time to open universal access
        tell application "System Events" to tell process "System Preferences" to tell window "Accessibility"
                select row 5 of table 1 of scroll area 1 #open display preferences
                click radio button "Color Filters" of tab group 1 of group 1
                click checkbox "Enable Color Filters" of tab group 1 of group 1
        end tell
end tell
tell application "System Preferences" to quit
"#;

fn main() {
    let mut bar = sysbar::Sysbar::new("G");

    bar.add_item(
        "Toggle grayscale",
        Box::new(move || {
            toggle_grayscale();
        }),
    );

    bar.add_quit_item("Quit");

    bar.display();
}

fn toggle_grayscale() {
    let mut command = process::Command::new("osascript");
    for line in SCRIPT.trim().lines() {
        command.arg("-e");
        command.arg(line.trim());
    }
    command.output().unwrap();
}
