use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Terminal,
};
use tungstenite::{connect, Message};
use url::Url;

#[macro_use]
extern crate log;

enum InputMode {
    Normal,
    Editing,
}

struct AppState {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input: String::from("Test"),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    // let mut events = Events::new();
    let mut app_state = AppState::default();

    // let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = std::io::stdout();
    // let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Socket Connection
    let input: String = "localhost".into();
    let url = format!("ws://{}:7878/ws", input);
    info!("Connecting to {}", url);
    let url = Url::parse(&url[..]).expect("Invlaid Url");
    let (mut socket, response) = connect(url).expect("Can't Connect");
    info!("Successfully connected to the server");
    trace!("Response HTTP code: {}", response.status());
    trace!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        trace!("* {}", header);
    }

    /*
        let mut vec = Vec::with_capacity(50);
        let msg = Message::Text("Test Terminal client".into());
        socket.write_message(msg).unwrap();
    */

    let mut i = 0;

    loop {
        /* get back kmessage
                let res = socket.read_message().expect("Error reading message");
                info!("Got: {}", res);
                vec.push(res);
        */

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(2)
                .horizontal_margin(4)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let title = format!(" Duckception Terminal Client {} ", i);
            i = i + 1;

            let msg = vec![
                Span::raw(title),
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ];

            // Help Message
            let message = Paragraph::new(Text::from(Spans::from(msg)));
            f.render_widget(message, chunks[0]);

            // Input area
            let input = Paragraph::new(app_state.input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title(" Commands "));
            f.render_widget(input, chunks[1]);

            // Messages
            let messages: Vec<ListItem> = app_state
                .messages
                .iter()
                .enumerate()
                .map(|(i, n)| ListItem::new(vec![Spans::from(Span::raw(format!("{}: {}", i, n)))]))
                .collect();
            let messages = List::new(messages)
                .block(Block::default().title(" Messages ").borders(Borders::ALL));
            f.render_widget(messages, chunks[2]);
        })?;
    }
}
