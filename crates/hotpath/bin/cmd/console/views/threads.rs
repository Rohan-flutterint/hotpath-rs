use super::common_styles;
use crate::cmd::console::widgets::formatters::truncate_left;
use hotpath::format_bytes;
use hotpath::threads::ThreadMetrics;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Cell, HighlightSpacing, Paragraph, Row, Table, TableState},
    Frame,
};

fn format_bytes_signed(bytes: i64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    let prefix = if bytes < 0 { "-" } else { "+" };
    let abs_bytes = bytes.unsigned_abs();
    format!("{}{}", prefix, format_bytes(abs_bytes))
}

/// Renders the threads table with thread metrics
#[cfg_attr(feature = "hotpath", hotpath::measure)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_threads_panel(
    threads: &[ThreadMetrics],
    area: Rect,
    frame: &mut Frame,
    table_state: &mut TableState,
    thread_position: usize,
    total_threads: usize,
    rss_bytes: Option<u64>,
) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).split(area);
    let info_area = chunks[0];
    let table_area = chunks[1];

    let alloc_enabled = threads.iter().any(|t| t.alloc_bytes.is_some());

    let pid = std::process::id();
    let rss_str = rss_bytes
        .map(format_bytes)
        .unwrap_or_else(|| "-".to_string());

    let mut spans = vec![
        Span::raw(" PID: "),
        Span::styled(
            pid.to_string(),
            ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
        ),
    ];

    if alloc_enabled {
        let total_mem: i64 = threads.iter().filter_map(|t| t.mem_diff).sum();
        let tracked_str = format_bytes(total_mem.unsigned_abs());
        spans.push(Span::raw("  Alloc - Dealloc: "));
        spans.push(Span::styled(
            tracked_str,
            ratatui::style::Style::default().fg(ratatui::style::Color::Green),
        ));
    } else {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            "Enable 'hotpath-alloc' to track memory usage",
            ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
        ));
    }

    spans.push(Span::raw("  RSS: "));
    spans.push(Span::styled(
        rss_str,
        ratatui::style::Style::default().fg(ratatui::style::Color::Cyan),
    ));

    let info_line = Line::from(spans);
    let info_paragraph = Paragraph::new(info_line);
    frame.render_widget(info_paragraph, info_area);

    let available_width = table_area.width.saturating_sub(10);
    let thread_width = ((available_width as f32 * 0.22) as usize).max(10);

    let header = Row::new(vec![
        Cell::from("Thread"),
        Cell::from("TID"),
        Cell::from("CPU %"),
        Cell::from("User"),
        Cell::from("Sys"),
        Cell::from("Alloc"),
        Cell::from("Dealloc"),
        Cell::from("Diff"),
    ])
    .style(common_styles::HEADER_STYLE)
    .height(1);

    let rows: Vec<Row> = threads
        .iter()
        .map(|thread| {
            let cpu_percent_str = match thread.cpu_percent {
                Some(pct) => format!("{:.1}%", pct),
                None => "-".to_string(),
            };

            let (alloc_str, dealloc_str, diff_str) = if alloc_enabled {
                (
                    thread
                        .alloc_bytes
                        .map(format_bytes)
                        .unwrap_or_else(|| "-".to_string()),
                    thread
                        .dealloc_bytes
                        .map(format_bytes)
                        .unwrap_or_else(|| "-".to_string()),
                    thread
                        .mem_diff
                        .map(format_bytes_signed)
                        .unwrap_or_else(|| "-".to_string()),
                )
            } else {
                ("N/A".to_string(), "N/A".to_string(), "N/A".to_string())
            };

            Row::new(vec![
                Cell::from(truncate_left(&thread.name, thread_width)),
                Cell::from(thread.os_tid.to_string()),
                Cell::from(cpu_percent_str),
                Cell::from(format!("{:.2}s", thread.cpu_user)),
                Cell::from(format!("{:.2}s", thread.cpu_sys)),
                Cell::from(alloc_str),
                Cell::from(dealloc_str),
                Cell::from(diff_str),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(22), // Thread name
        Constraint::Percentage(8),  // TID
        Constraint::Percentage(10), // CPU %
        Constraint::Percentage(10), // User
        Constraint::Percentage(10), // Sys
        Constraint::Percentage(13), // Alloc
        Constraint::Percentage(13), // Dealloc
        Constraint::Percentage(14), // Diff
    ];

    let table_block = Block::bordered()
        .title(format!(" [{}/{}] ", thread_position, total_threads))
        .border_set(border::THICK);

    let table = Table::new(rows, widths)
        .header(header)
        .block(table_block)
        .column_spacing(1)
        .row_highlight_style(common_styles::SELECTED_ROW_STYLE)
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(table, table_area, table_state);
}
