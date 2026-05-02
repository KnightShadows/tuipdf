// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2025
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Focus, Status};


const BG: Color = Color::Rgb(13, 13, 23);
const SURFACE: Color = Color::Rgb(22, 22, 38);
const SURFACE_ALT: Color = Color::Rgb(30, 30, 50);
const TEXT_PRIMARY: Color = Color::Rgb(230, 230, 245);
const TEXT_DIM: Color = Color::Rgb(90, 90, 120);
const TEXT_MUTED: Color = Color::Rgb(60, 60, 85);

const ACCENT_1: Color = Color::Rgb(100, 180, 255);
const ACCENT_2: Color = Color::Rgb(140, 120, 255);
const ACCENT_3: Color = Color::Rgb(200, 100, 255);

const SUCCESS: Color = Color::Rgb(70, 230, 150);
const SUCCESS_DIM: Color = Color::Rgb(40, 140, 90);
const ERROR_COLOR: Color = Color::Rgb(255, 80, 90);
const ERROR_DIM: Color = Color::Rgb(160, 50, 60);
const WARNING: Color = Color::Rgb(255, 190, 60);

const GAUGE_FILL: Color = Color::Rgb(100, 180, 255);
const GAUGE_BG: Color = Color::Rgb(35, 35, 55);

const BORDER_FOCUSED: Color = ACCENT_1;
const BORDER_UNFOCUSED: Color = Color::Rgb(45, 45, 70);
const BORDER_GLOW: Color = Color::Rgb(80, 140, 220);

const FOOTER_BG: Color = Color::Rgb(18, 18, 32);
const FOOTER_KEY: Color = ACCENT_1;
const FOOTER_SEP: Color = Color::Rgb(50, 50, 75);


const SPINNER: &[&str] = &["◜", "◠", "◝", "◞", "◡", "◟"];
const DOT: &str = "•";


pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    f.render_widget(
        Block::default().style(Style::default().bg(BG)),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(12),
            Constraint::Length(2),
        ])
        .split(area);

    draw_header(f, app, chunks[0]);
    draw_main(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}


fn draw_header(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .style(Style::default().bg(SURFACE));
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);


    let c_t = ACCENT_1;
    let c_u = Color::Rgb(120, 150, 255);
    let c_i = ACCENT_2;
    let c_p = Color::Rgb(170, 110, 255);
    let c_d = ACCENT_3;
    let c_f = Color::Rgb(230, 90, 200);

    let bold = Modifier::BOLD;


    let art: Vec<Vec<(&str, Color)>> = vec![
        vec![
            ("  ████████╗", c_t), ("██╗   ██╗", c_u), ("██╗", c_i),
            ("██████╗ ", c_p), ("██████╗ ", c_d), ("███████╗", c_f),
        ],
        vec![
            ("  ╚══██╔══╝", c_t), ("██║   ██║", c_u), ("██║", c_i),
            ("██╔══██╗", c_p), ("██╔══██╗", c_d), ("██╔════╝", c_f),
        ],
        vec![
            ("     ██║   ", c_t), ("██║   ██║", c_u), ("██║", c_i),
            ("██████╔╝", c_p), ("██║  ██║", c_d), ("█████╗  ", c_f),
        ],
        vec![
            ("     ██║   ", c_t), ("██║   ██║", c_u), ("██║", c_i),
            ("██╔═══╝ ", c_p), ("██║  ██║", c_d), ("██╔══╝  ", c_f),
        ],
        vec![
            ("     ██║   ", c_t), ("╚██████╔╝", c_u), ("██║", c_i),
            ("██║     ", c_p), ("██████╔╝", c_d), ("██║     ", c_f),
        ],
        vec![
            ("     ╚═╝    ", c_t), ("╚═════╝ ", c_u), ("╚═╝", c_i),
            ("╚═╝     ", c_p), ("╚═════╝ ", c_d), ("╚═╝     ", c_f),
        ],
    ];

    let logo_lines: Vec<Line> = art
        .iter()
        .map(|row| {
            Line::from(
                row.iter()
                    .map(|(text, color)| {
                        Span::styled(*text, Style::default().fg(*color).add_modifier(bold))
                    })
                    .collect::<Vec<Span>>(),
            )
        })
        .collect();

    f.render_widget(
        Paragraph::new(logo_lines)
            .style(Style::default().bg(SURFACE))
            .alignment(Alignment::Center),
        inner[1],
    );

    let tagline = Line::from(vec![
        Span::styled("⚙ ", Style::default().fg(ACCENT_2)),
        Span::styled("PDF Compression Tool ", Style::default().fg(TEXT_DIM)),
        Span::styled("─ ", Style::default().fg(BORDER_UNFOCUSED)),
        Span::styled("powered by ", Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC)),
        Span::styled("MuPDF", Style::default().fg(ACCENT_1).add_modifier(Modifier::BOLD)),
        Span::styled(" & ", Style::default().fg(TEXT_MUTED)),
        Span::styled("Rust", Style::default().fg(Color::Rgb(255, 120, 50)).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(tagline)
            .style(Style::default().bg(SURFACE))
            .alignment(Alignment::Center),
        inner[2],
    );

    let sep_width = area.width as usize;
    let sep_line = "─".repeat(sep_width);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            sep_line,
            Style::default().fg(BORDER_UNFOCUSED),
        )))
        .style(Style::default().bg(BG)),
        inner[3],
    );
}


fn draw_main(f: &mut Frame, app: &App, area: Rect) {
    let has_prompt = app.prompt.is_some();

    let mut constraints = vec![
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(3),
    ];

    if has_prompt {
        constraints.push(Constraint::Length(2));
    }
    constraints.push(Constraint::Min(5));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .vertical_margin(1)
        .constraints(constraints)
        .split(area);

    let mut idx = 0;

    let section_label = Line::from(vec![
        Span::styled("  ┌ ", Style::default().fg(ACCENT_2)),
        Span::styled("FILE CONFIGURATION", Style::default().fg(ACCENT_1).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(Paragraph::new(section_label), chunks[idx]);
    idx += 1;

    draw_input_field(f, app, chunks[idx]);
    idx += 1;

    draw_output_field(f, app, chunks[idx]);
    idx += 1;

    let spacer = Line::from(Span::styled(
        "  └─────────────────────────────────",
        Style::default().fg(BORDER_UNFOCUSED),
    ));
    f.render_widget(Paragraph::new(spacer), chunks[idx]);
    idx += 1;

    draw_compression_levels(f, app, chunks[idx]);
    idx += 1;

    if has_prompt {
        draw_prompt(f, app, chunks[idx]);
        idx += 1;
    }

    draw_status_area(f, app, chunks[idx]);
}


fn draw_input_field(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::InputPath;
    let border_color = if focused { BORDER_FOCUSED } else { BORDER_UNFOCUSED };

    let icon = if focused { "📂" } else { "📄" };
    let label = if focused {
        format!(" {} Input PDF (editing) ", icon)
    } else {
        format!(" {} Input PDF ", icon)
    };

    let block = Block::default()
        .title(Span::styled(
            label,
            Style::default()
                .fg(if focused { ACCENT_1 } else { TEXT_DIM })
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(SURFACE));

    let value = app.input_field.value();
    let inner_width = area.width.saturating_sub(2) as usize;

    let scroll = app.input_field.scroll();
    let display: String = value.chars().skip(scroll).take(inner_width).collect();

    let text = if value.is_empty() && !focused {
        Line::from(Span::styled(
            "  Drop or type a PDF file path…",
            Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC),
        ))
    } else if value.is_empty() && focused {
        Line::from(Span::styled(
            "  Start typing…",
            Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC),
        ))
    } else {
        Line::from(Span::styled(display, Style::default().fg(TEXT_PRIMARY)))
    };

    f.render_widget(Paragraph::new(text).block(block), area);

    if focused {
        let cursor_x = area.x + 1 + (app.input_field.cursor() - app.input_field.scroll()) as u16;
        let cursor_y = area.y + 1;
        if cursor_x < area.x + area.width - 1 {
            f.set_cursor_position((cursor_x, cursor_y));
        }
    }
}


fn draw_output_field(f: &mut Frame, app: &App, area: Rect) {
    let size_info = match app.input_file_size() {
        Some(size) => format!(" {} {} ", DOT, format_size(size)),
        None => String::new(),
    };

    let title = format!(" 💾 Output PDF{} ", size_info);

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(TEXT_DIM).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(BORDER_UNFOCUSED))
        .style(Style::default().bg(SURFACE));

    let text = if app.output_path.is_empty() {
        Line::from(Span::styled(
            "  Auto-generated from input…",
            Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC),
        ))
    } else {
        Line::from(Span::styled(
            &app.output_path,
            Style::default().fg(TEXT_PRIMARY),
        ))
    };

    f.render_widget(Paragraph::new(text).block(block), area);
}


fn draw_compression_levels(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::CompressionLevel;
    let border_color = if focused { BORDER_FOCUSED } else { BORDER_UNFOCUSED };

    let block = Block::default()
        .title(Span::styled(
            " ⚡ Compression Level ",
            Style::default()
                .fg(if focused { ACCENT_1 } else { TEXT_DIM })
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(SURFACE));

    let levels = [
        (crate::app::CompressionLevel::Low, "Low", "Quality", ACCENT_1),
        (crate::app::CompressionLevel::Medium, "Medium", "Balanced", ACCENT_2),
        (crate::app::CompressionLevel::High, "High", "Max Save", ACCENT_3),
    ];

    let mut spans: Vec<Span> = Vec::new();
    for (i, (level, name, desc, color)) in levels.iter().enumerate() {
        let is_selected = app.compression_level == *level;

        if i > 0 {
            spans.push(Span::styled("   ", Style::default()));
        }

        if is_selected {
            spans.push(Span::styled(
                format!(" ● {} ", name),
                Style::default()
                    .fg(BG)
                    .bg(*color)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {}", desc),
                Style::default().fg(*color).add_modifier(Modifier::ITALIC),
            ));
        } else {
            spans.push(Span::styled(
                format!(" ○ {} ", name),
                Style::default().fg(TEXT_DIM),
            ));
        }
    }

    if focused {
        spans.push(Span::styled(
            "  ← Enter to toggle →",
            Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC),
        ));
    }

    f.render_widget(
        Paragraph::new(Line::from(spans))
            .block(block)
            .alignment(Alignment::Center),
        area,
    );
}


fn draw_prompt(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref prompt) = app.prompt {
        let text = Line::from(vec![
            Span::styled("  💡 ", Style::default()),
            Span::styled(
                "Instruction: ",
                Style::default().fg(ACCENT_2).add_modifier(Modifier::ITALIC),
            ),
            Span::styled(prompt, Style::default().fg(TEXT_PRIMARY)),
        ]);
        f.render_widget(Paragraph::new(text), area);
    }
}


fn draw_status_area(f: &mut Frame, app: &App, area: Rect) {
    match &app.status {
        Status::Idle => draw_idle_action(f, app, area),
        Status::Compressing => draw_compressing(f, app, area),
        Status::Done { original, compressed } => draw_done(f, *original, *compressed, area),
        Status::Error(msg) => draw_error(f, msg, area),
    }
}

fn draw_idle_action(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::ActionButton;

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    if focused {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .border_style(Style::default().fg(BORDER_GLOW))
            .style(Style::default().bg(SURFACE_ALT));

        let btn_text = Line::from(vec![
            Span::styled(" ▶ ", Style::default().fg(ACCENT_1).add_modifier(Modifier::BOLD)),
            Span::styled(
                " COMPRESS ",
                Style::default()
                    .fg(ACCENT_1)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ⏎ ", Style::default().fg(TEXT_DIM)),
        ]);

        f.render_widget(
            Paragraph::new(btn_text).block(block).alignment(Alignment::Center),
            v_chunks[1],
        );
    } else {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .border_style(Style::default().fg(BORDER_UNFOCUSED))
            .style(Style::default().bg(SURFACE));

        let btn_text = Line::from(Span::styled(
            "Tab → Compress Button → Enter",
            Style::default().fg(TEXT_MUTED),
        ));

        f.render_widget(
            Paragraph::new(btn_text).block(block).alignment(Alignment::Center),
            v_chunks[1],
        );
    }
}

fn draw_compressing(f: &mut Frame, app: &App, area: Rect) {
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(v_chunks[1]);

    let spinner_idx = (app.tick / 3) % SPINNER.len();
    let spinner = SPINNER[spinner_idx];
    let pct = (app.progress * 100.0) as u32;

    let phase_text = if pct < 20 {
        "Reading PDF…"
    } else if pct < 80 {
        "Optimizing images & fonts…"
    } else {
        "Writing output…"
    };

    let status_line = Line::from(vec![
        Span::styled(format!("  {} ", spinner), Style::default().fg(ACCENT_1).add_modifier(Modifier::BOLD)),
        Span::styled(phase_text, Style::default().fg(TEXT_PRIMARY)),
    ]);
    f.render_widget(Paragraph::new(status_line), inner[0]);

    let label = format!(" {}% ", pct);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled(
                    label,
                    Style::default().fg(ACCENT_1).add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .border_style(Style::default().fg(ACCENT_2))
                .style(Style::default().bg(SURFACE)),
        )
        .gauge_style(Style::default().fg(GAUGE_FILL).bg(GAUGE_BG))
        .ratio(app.progress as f64);

    f.render_widget(gauge, inner[2]);
}

fn draw_done(f: &mut Frame, original: u64, compressed: u64, area: Rect) {
    let saved = original.saturating_sub(compressed);
    let pct_of_original = if original > 0 {
        (compressed as f64 / original as f64) * 100.0
    } else {
        100.0
    };
    let pct_saved = if original > 0 {
        (saved as f64 / original as f64) * 100.0
    } else {
        0.0
    };

    let (verdict, verdict_color) = if pct_saved > 50.0 {
        ("Excellent", SUCCESS)
    } else if pct_saved > 25.0 {
        ("Great", SUCCESS)
    } else if pct_saved > 10.0 {
        ("Good", ACCENT_1)
    } else if pct_saved > 0.0 {
        ("Minimal", WARNING)
    } else {
        ("No reduction", TEXT_DIM)
    };

    let height = 12u16;
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE)
        .border_style(Style::default().fg(SUCCESS_DIM))
        .style(Style::default().bg(SURFACE));

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✅ ", Style::default().fg(SUCCESS)),
            Span::styled(
                "Compression Complete",
                Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  — {}", verdict), Style::default().fg(verdict_color).add_modifier(Modifier::ITALIC)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("   📦 Original    ", Style::default().fg(TEXT_DIM)),
            Span::styled(
                format_size(original),
                Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("   📁 Compressed  ", Style::default().fg(TEXT_DIM)),
            Span::styled(
                format_size(compressed),
                Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("   📊 Ratio       ", Style::default().fg(TEXT_DIM)),
            Span::styled(
                format!("{:.1}%", pct_of_original),
                Style::default().fg(ACCENT_1).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  (saved {})", format_size(saved)),
                Style::default().fg(SUCCESS),
            ),
        ]),
        Line::from(""),
        Line::from(build_savings_bar(pct_saved, area.width.saturating_sub(8) as usize)),
        Line::from(""),
        Line::from(Span::styled(
            "  Press Enter to compress again  •  q to quit",
            Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC),
        )),
    ];

    f.render_widget(
        Paragraph::new(lines).block(block).wrap(Wrap { trim: false }),
        v_chunks[1],
    );
}

fn build_savings_bar(pct_saved: f64, width: usize) -> Vec<Span<'static>> {
    let bar_width = width.saturating_sub(14);
    let filled = ((pct_saved / 100.0) * bar_width as f64).round() as usize;
    let empty = bar_width.saturating_sub(filled);

    vec![
        Span::styled("   ", Style::default()),
        Span::styled(
            "█".repeat(filled),
            Style::default().fg(SUCCESS),
        ),
        Span::styled(
            "░".repeat(empty),
            Style::default().fg(GAUGE_BG),
        ),
        Span::styled(
            format!(" {:.1}% saved", pct_saved),
            Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
        ),
    ]
}

fn draw_error(f: &mut Frame, msg: &str, area: Rect) {
    let height = 8u16;
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(ERROR_DIM))
        .style(Style::default().bg(SURFACE));

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ❌ ", Style::default().fg(ERROR_COLOR)),
            Span::styled(
                "Compression Failed",
                Style::default().fg(ERROR_COLOR).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", msg),
            Style::default().fg(TEXT_PRIMARY),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Press any key to dismiss",
            Style::default().fg(TEXT_MUTED).add_modifier(Modifier::ITALIC),
        )),
    ];

    f.render_widget(Paragraph::new(lines).block(block), v_chunks[1]);
}


fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let sep_width = area.width as usize;
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(sep_width),
            Style::default().fg(BORDER_UNFOCUSED),
        )))
        .style(Style::default().bg(FOOTER_BG)),
        inner[0],
    );

    let is_compressing = matches!(app.status, Status::Compressing);

    let shortcuts = vec![
        Span::styled(" ⏎ ", Style::default().fg(FOOTER_KEY).add_modifier(Modifier::BOLD)),
        Span::styled(
            if is_compressing { "…" } else { "Compress" },
            Style::default().fg(TEXT_DIM),
        ),
        Span::styled(format!("  {}  ", DOT), Style::default().fg(FOOTER_SEP)),
        Span::styled("⇥ ", Style::default().fg(FOOTER_KEY).add_modifier(Modifier::BOLD)),
        Span::styled("Navigate", Style::default().fg(TEXT_DIM)),
        Span::styled(format!("  {}  ", DOT), Style::default().fg(FOOTER_SEP)),
        Span::styled("Esc ", Style::default().fg(FOOTER_KEY).add_modifier(Modifier::BOLD)),
        Span::styled("Quit", Style::default().fg(TEXT_DIM)),
    ];

    f.render_widget(
        Paragraph::new(Line::from(shortcuts))
            .style(Style::default().bg(FOOTER_BG))
            .alignment(Alignment::Center),
        inner[1],
    );
}


fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}