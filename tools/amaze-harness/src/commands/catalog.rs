use std::fs;

use crate::cli::{InventoryArgs, PackagesArgs};
use crate::domain::CatalogDoc;
#[cfg(test)]
use crate::domain::{BeatPackage, InventoryItem};
use crate::markdown::md_cell;

pub(crate) fn inventory_catalog(args: InventoryArgs) -> Result<(), String> {
    let markdown = fs::read_to_string(&args.catalog)
        .map_err(|err| format!("failed to read {}: {err}", args.catalog.display()))?;
    let query = args.query.as_ref().map(|raw| raw.to_lowercase());
    let mut rows = CatalogDoc::from_markdown(&markdown).inventory_items;

    if let Some(query) = query {
        rows.retain(|row| row.matches_query(&query));
    }

    println!("# AMAZE Inventory");
    println!();
    println!("Catalog: {}", args.catalog.display());
    if let Some(query) = &args.query {
        println!("Query: {query}");
    }
    println!("Items: {}", rows.len());
    println!();
    println!("| ID | Item | Common sources | Unit cost band | Durability | Typical use |");
    println!("|---|---|---|---:|---|---|");
    for row in rows {
        println!(
            "| {} | {} | {} | {} | {} | {} |",
            row.id,
            row.item,
            row.common_sources,
            row.unit_cost_band,
            row.durability,
            row.typical_use
        );
    }

    Ok(())
}

pub(crate) fn packages_catalog(args: PackagesArgs) -> Result<(), String> {
    let markdown = fs::read_to_string(&args.catalog)
        .map_err(|err| format!("failed to read {}: {err}", args.catalog.display()))?;
    let query = args.query.as_ref().map(|raw| raw.to_lowercase());
    let mut rows = CatalogDoc::from_markdown(&markdown).beat_packages;

    if let Some(query) = query {
        rows.retain(|row| row.matches_query(&query));
    }

    println!("# AMAZE Beat Packages");
    println!();
    println!("Catalog: {}", args.catalog.display());
    if let Some(query) = &args.query {
        println!("Query: {query}");
    }
    println!("Packages: {}", rows.len());
    println!();
    println!(
        "| Package ID | Beat package | Use when | Techniques | Default devices | Evidence hooks |"
    );
    println!("|---|---|---|---|---|---|");
    for row in rows {
        println!(
            "| {} | {} | {} | {} | {} | {} |",
            md_cell(&row.package_id),
            md_cell(&row.beat_package),
            md_cell(&row.use_when),
            md_cell(&row.techniques),
            md_cell(&row.default_devices),
            md_cell(&row.evidence_hooks)
        );
    }

    Ok(())
}

#[cfg(test)]
pub(crate) fn package_rows(markdown: &str) -> Vec<BeatPackage> {
    CatalogDoc::from_markdown(markdown).beat_packages
}

#[cfg(test)]
pub(crate) fn inventory_rows(markdown: &str) -> Vec<InventoryItem> {
    CatalogDoc::from_markdown(markdown).inventory_items
}

#[cfg(test)]
mod tests {
    use super::{inventory_rows, package_rows};

    #[test]
    fn extracts_package_rows_from_catalog() {
        let markdown = "\
# Packages

| Package ID | Beat package | Use when | Techniques | Default devices | Evidence hooks |
|---|---|---|---|---|---|
| PKG-SOCKET-PROOF | Socket proof | keyed proof needed | `TECH-FIT-001` | `DEV-SOCKET-001` | bench socket fit |

| Not package | Beat package |
|---|---|
| ignored | ignored |
";
        let rows = package_rows(markdown);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].package_id, "PKG-SOCKET-PROOF");
    }

    #[test]
    fn extracts_inventory_rows_from_catalog() {
        let markdown = "\
# Inventory

| ID | Item | Common sources | Unit cost band | Durability | Typical use |
|---|---|---|---:|---|---|
| DEV-SOCKET-001 | socket | hardware | `$5-25` | B | proof |

| Not inventory | Item |
|---|---|
| ignored | ignored |
";
        let rows = inventory_rows(markdown);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "DEV-SOCKET-001");
    }
}
