use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Map, MapResolution, Points},
    },
    Frame,
};

use crate::app::{App, AppState};

const ASCII_ART: &[&str] = &[
    " ██╗██████╗     ███╗   ███╗ ██████╗  ██████╗ ",
    " ██║██╔══██╗    ████╗ ████║██╔═══██╗██╔════╝ ",
    " ██║██████╔╝    ██╔████╔██║██║   ██║██║  ███╗",
    " ██║██╔═══╝     ██║╚██╔╝██║██║   ██║██║   ██║",
    " ██║██║         ██║ ╚═╝ ██║╚██████╔╝╚██████╔╝",
    " ╚═╝╚═╝         ╚═╝     ╚═╝ ╚═════╝  ╚═════╝ ",
];

pub fn render(frame: &mut Frame, app: &App) {
    match &app.state {
        AppState::Loading { frame: tick } => render_loading(frame, *tick),
        AppState::Loaded { ip_info } => render_loaded(frame, ip_info),
        AppState::Error { message } => render_error(frame, message),
    }
}

fn render_loading(frame: &mut Frame, tick: u64) {
    let area = frame.area();

    let art_height = ASCII_ART.len() as u16;
    let total_height = art_height + 3; // art + gap + loading text
    let y_offset = area.height.saturating_sub(total_height) / 2;

    // Build ASCII art lines with left-to-right reveal + color gradient
    let chars_per_line = ASCII_ART[0].chars().count();
    let reveal = ((tick as usize) * 2).min(chars_per_line);

    let art_lines: Vec<Line> = ASCII_ART
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut spans = Vec::new();

            for (i, &ch) in chars.iter().enumerate() {
                if i < reveal {
                    let progress = i as f64 / chars_per_line as f64;
                    let color = gradient_cyan_magenta(progress);
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            Line::from(spans)
        })
        .collect();

    let art = Paragraph::new(art_lines).alignment(Alignment::Center);

    let art_rect = Rect::new(area.x, y_offset, area.width, art_height);
    frame.render_widget(art, art_rect);

    // Loading dots animation
    let dots_count = ((tick / 8) % 4) as usize;
    let dots = ".".repeat(dots_count);
    let loading_text = format!("Fetching your IP{dots}");
    let loading = Paragraph::new(loading_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    let loading_rect = Rect::new(area.x, y_offset + art_height + 1, area.width, 1);
    frame.render_widget(loading, loading_rect);
}

fn render_loaded(frame: &mut Frame, ip_info: &crate::ip::IpInfo) {
    let area = frame.area();

    let outer = Layout::vertical([
        Constraint::Length(1), // title bar
        Constraint::Min(10),  // main content
        Constraint::Length(1), // keybindings
    ])
    .split(area);

    // Title bar
    let title = Paragraph::new(" IP MOG")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(title, outer[0]);

    // Main area: map (left) + info (right)
    let main = Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(outer[1]);

    // Map canvas
    let lat = ip_info.latitude;
    let lon = ip_info.longitude;
    let span = 30.0;

    let map_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" MAP ");

    let canvas = Canvas::default()
        .block(map_block)
        .x_bounds([lon - span * 1.5, lon + span * 1.5])
        .y_bounds([lat - span, lat + span])
        .paint(move |ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::DarkGray,
            });
            // Location marker
            ctx.draw(&Points {
                coords: &[(lon, lat)],
                color: Color::Red,
            });
            ctx.print(
                lon + 1.5,
                lat + 1.5,
                Line::from(Span::styled(
                    format!("◉ {}", ip_info.city),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
            );
        });
    frame.render_widget(canvas, main[0]);

    // Info panel
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" INFO ");

    let asn_str = ip_info.asn.to_string();
    let lat_str = format!("{:.4}", ip_info.latitude);
    let lon_str = format!("{:.4}", ip_info.longitude);

    let info_lines = vec![
        info_line("IP", &ip_info.ip, Color::Cyan),
        Line::raw(""),
        info_line("City", &ip_info.city, Color::Green),
        info_line("Region", &ip_info.region, Color::Green),
        info_line("Postal", &ip_info.postal_code, Color::Green),
        info_line("Country", &ip_info.country, Color::Green),
        Line::raw(""),
        info_line("ISP", &ip_info.isp, Color::Magenta),
        info_line("ASN", &asn_str, Color::Magenta),
        Line::raw(""),
        info_line("Timezone", &ip_info.timezone, Color::Yellow),
        Line::raw(""),
        info_line("Lat", &lat_str, Color::Blue),
        info_line("Lon", &lon_str, Color::Blue),
    ];

    let info = Paragraph::new(info_lines).block(info_block);
    frame.render_widget(info, main[1]);

    // Bottom keybindings
    let keys = Paragraph::new(Line::from(vec![
        Span::styled(" r", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": reload  "),
        Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": quit"),
    ]))
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(keys, outer[2]);
}

fn render_error(frame: &mut Frame, message: &str) {
    let area = frame.area();

    let art_height = ASCII_ART.len() as u16;
    let total_height = art_height + 5;
    let y_offset = area.height.saturating_sub(total_height) / 2;

    let art_lines: Vec<Line> = ASCII_ART
        .iter()
        .map(|line| {
            Line::from(Span::styled(
                *line,
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ))
        })
        .collect();

    let art = Paragraph::new(art_lines).alignment(Alignment::Center);
    frame.render_widget(art, Rect::new(area.x, y_offset, area.width, art_height));

    let err = Paragraph::new(message)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(
        err,
        Rect::new(area.x, y_offset + art_height + 1, area.width, 2),
    );

    let hint = Paragraph::new("Press r to retry or q to quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(
        hint,
        Rect::new(area.x, y_offset + art_height + 3, area.width, 1),
    );
}

fn info_line<'a>(label: &'a str, value: &'a str, color: Color) -> Line<'a> {
    Line::from(vec![
        Span::styled(
            format!(" {label:<9}"),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(value, Style::default().fg(color)),
    ])
}

fn gradient_cyan_magenta(t: f64) -> Color {
    // Cyan (0, 255, 255) → Magenta (255, 0, 255)
    let r = (t * 255.0) as u8;
    let g = ((1.0 - t) * 255.0) as u8;
    let b = 255u8;
    Color::Rgb(r, g, b)
}
