use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Table {
    pub(crate) rows: Vec<HashMap<String, String>>,
}

pub(crate) fn extract_table(markdown: &str, heading: &str) -> Option<Table> {
    let mut in_section = false;
    let mut lines = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed == heading {
            in_section = true;
            continue;
        }
        if in_section && trimmed.starts_with("## ") {
            break;
        }
        if in_section && trimmed.starts_with('|') {
            lines.push(trimmed.to_string());
        }
    }

    parse_table(&lines)
}

pub(crate) fn parse_table(lines: &[String]) -> Option<Table> {
    if lines.len() < 2 {
        return None;
    }

    let headers = split_row(&lines[0]);
    if headers.is_empty() {
        return None;
    }

    let rows = lines
        .iter()
        .skip(2)
        .map(|line| split_row(line))
        .filter(|cells| !cells.is_empty())
        .map(|cells| {
            headers
                .iter()
                .cloned()
                .zip(cells)
                .collect::<HashMap<String, String>>()
        })
        .collect();

    Some(Table { rows })
}

pub(crate) fn tables_with_headers(
    markdown: &str,
    required_headers: &[&str],
) -> Vec<HashMap<String, String>> {
    let mut rows = Vec::new();
    let mut table_lines = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('|') {
            table_lines.push(trimmed.to_string());
            continue;
        }

        extend_matching_rows(&mut rows, &table_lines, required_headers);
        table_lines.clear();
    }

    extend_matching_rows(&mut rows, &table_lines, required_headers);
    rows
}

fn extend_matching_rows(
    rows: &mut Vec<HashMap<String, String>>,
    table_lines: &[String],
    required_headers: &[&str],
) {
    if let Some(table) = parse_table(table_lines) {
        rows.extend(table.rows.into_iter().filter(|row| {
            required_headers
                .iter()
                .all(|header| row.contains_key(*header))
        }));
    }
}

fn split_row(line: &str) -> Vec<String> {
    line.trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

pub(crate) fn value(row: &HashMap<String, String>, key: &str) -> String {
    row.get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .unwrap_or_else(|| "TBD".to_string())
}

pub(crate) fn md_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

pub(crate) fn first_present(row: &HashMap<String, String>, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|key| row.get(*key).filter(|value| !value.is_empty()).cloned())
        .unwrap_or_else(|| "TBD".to_string())
}

pub(crate) fn parse_minutes_field(row: &HashMap<String, String>, keys: &[&str]) -> Option<u32> {
    keys.iter()
        .find_map(|key| row.get(*key))
        .and_then(|raw| parse_minutes(raw))
}

fn parse_minutes(raw: &str) -> Option<u32> {
    let digits: String = raw.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

pub(crate) fn append_rows_to_section(
    markdown: &str,
    heading: &str,
    rows: &[String],
) -> Result<String, String> {
    let lines: Vec<&str> = markdown.lines().collect();
    let heading_index = lines
        .iter()
        .position(|line| line.trim() == heading)
        .ok_or_else(|| format!("missing section heading: {heading}"))?;

    let section_end = lines
        .iter()
        .enumerate()
        .skip(heading_index + 1)
        .find(|(_, line)| line.trim().starts_with("## "))
        .map(|(index, _)| index)
        .unwrap_or(lines.len());

    let mut output = Vec::new();
    output.extend(lines[..section_end].iter().map(|line| line.to_string()));
    output.extend(rows.iter().cloned());
    if section_end < lines.len() {
        output.extend(lines[section_end..].iter().map(|line| line.to_string()));
    }

    Ok(format!("{}\n", output.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::{append_rows_to_section, md_cell, tables_with_headers};

    #[test]
    fn escapes_markdown_cells() {
        assert_eq!(md_cell("a|b\nc"), "a\\|b c");
    }

    #[test]
    fn appends_rows_to_section() {
        let markdown =
            "# Playtest\n\n## Beat timing evidence\n\n| A | B |\n|---|---|\n\n## Scores\n";
        let updated = append_rows_to_section(
            markdown,
            "## Beat timing evidence",
            &["| x | y |".to_string()],
        )
        .expect("section exists");
        assert!(updated.contains("| x | y |\n## Scores"));
    }

    #[test]
    fn extracts_rows_from_tables_with_required_headers() {
        let markdown = "\
# Catalog

## A

| ID | Item | Unit cost band |
|---|---|---:|
| MECH-001 | latch | `$5-25` |

## Notes

| Not ID | Item |
|---|---|
| x | ignored |

## B

| ID | Item | Unit cost band |
|---|---|---:|
| ELEC-001 | LED strip | `$10-35` |
";
        let rows = tables_with_headers(markdown, &["ID", "Item"]);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get("ID").map(String::as_str), Some("MECH-001"));
        assert_eq!(rows[1].get("Item").map(String::as_str), Some("LED strip"));
    }
}
