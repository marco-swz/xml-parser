use anyhow::Result;
use xml_parser::parse_duration;
use std::fs::{read_to_string, File};

use xmltree::{self, Element, XMLNode};

fn main() -> Result<()> {
    let file = read_to_string("rezepte.xml")?;
    let root = Element::parse(file.as_bytes())?;
    let Some((XMLNode::Element(mut root), duration_sum, duration_count)) =
        process_node(XMLNode::Element(root), 0)
    else {
        panic!("unexpected return")
    };

    let duration_avg = duration_sum / duration_count as f64;
    let mut average_elem = Element::new("durchschnitt");
    average_elem
        .children
        .push(XMLNode::Text(duration_avg.to_string() + " s"));

    root.children.push(XMLNode::Element(average_elem));

    let file = File::options()
        .write(true)
        .create_new(true)
        .open("out.xml")?;
    root.write(file)?;
    Ok(())
}

fn process_node(node: XMLNode, level: usize) -> Option<(XMLNode, f64, usize)> {
    let indent = "  ".repeat(level);
    match node {
        XMLNode::Text(ref txt) => {
            println!("  {}text: {}", indent, txt.trim());
            Some((node, 0., 0))
        }
        XMLNode::Element(mut elem) => {
            println!("{}node: {}", indent, elem.name);

            let mut duration_sum = 0.;
            let mut duration_count = 0;
            let mut remove = elem.get_child("zutaten").is_none();
            elem.children = elem
                .children
                .into_iter()
                .filter_map(|child| {
                    let Some((child, dur_sum, dur_count)) = process_node(child, level + 1) else {
                        remove = true;
                        return None;
                    };
                    if dur_count > 0 {
                        duration_sum += dur_sum;
                        duration_count += dur_count;
                    }
                    Some(child)
                })
                .collect();

            match (elem.name.as_str(), remove) {
                ("zutaten", _) => match elem.children.len() {
                    3.. => Some((XMLNode::Element(elem), duration_sum, duration_count)),
                    _ => None,
                },
                ("zubereitung", _) => {
                    if let Some(duration_text) = elem.attributes.get("dauer") {
                        if let Ok(duration) = parse_duration(duration_text) {
                            return Some((
                                XMLNode::Element(elem),
                                duration_sum + duration,
                                duration_count + 1,
                            ));
                        }
                    }

                    Some((XMLNode::Element(elem), duration_sum, duration_count))
                }
                ("rezept", true) => None,
                _ => Some((XMLNode::Element(elem), duration_sum, duration_count)),
            }
        }
        _ => Some((node, 0., 0)),
    }
}
