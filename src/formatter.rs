use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::ListItem,
};

use crate::model::ThreadPost;

pub fn format_default(str: &str) -> String {
    format!(" {}", str)
}

pub fn format_post(post: &ThreadPost, no: usize, short: bool) -> ListItem {
    let mut lines = vec![Spans::from("")];
    let mut header: Vec<Span> = vec![];

    if !post.sub.is_empty() {
        header.push(Span::styled(
            format_default(&post.sub),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
    }
    header.push(Span::raw(" "));
    header.push(Span::styled(
        format!(
            "{} {} No.{}",
            htmlescape::decode_html(&post.name).unwrap(),
            format_time(post.time),
            post.no
        ),
        Style::default().add_modifier(Modifier::ITALIC | Modifier::UNDERLINED),
    ));

    if !short {
        header.push(Span::styled(
            format_default(&format!("#{}", no)),
            Style::default().fg(Color::Yellow),
        ));
    }

    if post.sticky == 1 {
        header.push(Span::styled(format_default("📌"), Style::default()));
    }

    if post.closed == 1 {
        header.push(Span::styled(format_default("🔓"), Style::default()));
    }

    lines.push(Spans::from(header));

    if post.filename.is_some() && post.ext.is_some() {
        lines.push(Spans::from(Span::styled(
            format_default(&format!(
                "{}{}",
                post.filename.as_ref().unwrap(),
                post.ext.as_ref().unwrap()
            )),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )));
    }

    const LEN: usize = 110;
    const LIMIT_SHORT: usize = 10;
    const LIMIT_LONG: usize = 100;

    let cut_com =
        format_post_contents(&post.com, LEN, if short { LIMIT_SHORT } else { LIMIT_LONG });
    for span in cut_com {
        lines.push(span);
    }

    if short {
        lines.push(Spans::from(Span::styled(
            format_default(&format!("{} Replies", &post.replies)),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::ITALIC),
        )));
    }

    lines.push(Spans::from(""));
    ListItem::new(Text::from(lines)).style(Style::default())
}

fn format_post_contents(string: &str, sub_len: usize, line_limit: usize) -> Vec<Spans> {
    let string = htmlescape::decode_html(string).unwrap();
    let split = string.split("<br>");
    let vec_str: Vec<&str> = split.collect();

    let mut subs = Vec::with_capacity(sub_len * line_limit);
    let mut i = 0;

    'lineloop: for line in vec_str {
        use voca_rs::*;
        let line = strip::strip_tags(&line);

        let mut iter = line.chars();
        let strlen = line.len();
        let mut pos = 0;

        if strlen == 0 {
            subs.push(Spans::from(""));

            i += 1;

            if i == line_limit {
                break;
            }
        }

        while pos < strlen {
            let mut greentext = false;
            let mut reply = false;

            for (j, char) in line.chars().enumerate() {
                if j == 0 && char == '>' {
                    greentext = true;
                }

                if j == 1 && char == '>' && greentext {
                    reply = true;
                    greentext = false;
                    break;
                }
            }

            let mut len = 0;
            for ch in iter.by_ref().take(sub_len) {
                len += ch.len_utf8();
            }
            let mut style = Style::default();
            if reply {
                style = style.fg(Color::Yellow);
            } else if greentext {
                style = style.fg(Color::Green);
            }

            if i == line_limit {
                let mut lim = 5;
                if pos + len <= 5 {
                    lim = 5 - len + pos;
                }
                subs.push(Spans::from(vec![
                    Span::styled(format_default(&line[pos..pos + len - lim]), style),
                    Span::styled("[...]", Style::default().fg(Color::Magenta)),
                ]));
                break 'lineloop;
            } else {
                subs.push(Spans::from(Span::styled(
                    format_default(&line[pos..pos + len]),
                    style,
                )));
            }

            pos += len;
            i += 1;
        }
    }

    subs
}

fn format_time(timestamp: u64) -> String {
    let st = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Utc>::from(st);

    datetime.format("%m/%d/%y(%a)%H:%M:%S").to_string()
}
