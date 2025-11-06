pub const LOGO: &str = "    ██╗  ██╗██╗██╗     ██╗     ██████╗  ██████╗ ████████╗
    ██║ ██╔╝██║██║     ██║     ██╔══██╗██╔═══██╗╚══██╔══╝
    █████╔╝ ██║██║     ██║     ██████╔╝██║   ██║   ██║
    ██╔═██╗ ██║██║     ██║     ██╔══██╗██║   ██║   ██║
    ██║  ██╗██║███████╗███████╗██████╔╝╚██████╔╝   ██║
    ╚═╝  ╚═╝╚═╝╚══════╝╚══════╝╚═════╝  ╚═════╝    ╚═╝";

#[derive(Debug)]
pub struct Page {
    pub label: &'static str,
    pub content: &'static str
}

pub const PAGES: [Page; 3] = [
    Page {
        label: "INDEX - INDEX - INDEX - INDEX - INDEX - INDEX - INDEX - INDEX",
        content: "You probably want to use `killbot help` instead."
    },
    Page {
        label: "HELP - HELP - HELP - HELP - HELP - HELP - HELP - HELP - HELP",
        content: "All commands:
- killbot
the landing page.

- killbot help
leads this page.

- killbot compile [1]
1: The file path.
compiles the CITB (.citb) file into KILLCODE (.kill).

- killbot run [1]
1: The file path.
runs the KILLCODE."
    },
    Page {
        label: "INFO - INFO - INFO - INFO - INFO - INFO - INFO - INFO - INFO",
        content: "Explains a topic.

Valid topics:
citb
killbot
killcode"
    }
];
