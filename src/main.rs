use std::{error::Error, fs::read_to_string, io::stdout};
use xmltree::{self, Element, XMLNode};

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string("rezepte.xml")?;
    let root = Element::parse(file.as_bytes())?;
    let mut duration_sum = 0.;
    let mut duration_count = 0;
    let Some(XMLNode::Element(mut root)) = process_node(
        XMLNode::Element(root),
        0,
        &mut duration_sum,
        &mut duration_count,
    ) else {
        panic!("");
    };

    let duration_avg = duration_sum / duration_count as f64;
    let mut average_elem = Element::new("durchschnitt");
    average_elem.children.push(XMLNode::Text(duration_avg.to_string()));

    root.children.push(XMLNode::Element(average_elem));

    root.write(stdout()).unwrap();
    Ok(())
}

fn process_node(
    node: XMLNode,
    level: usize,
    duration_sum: &mut f64,
    duration_count: &mut usize,
) -> Option<XMLNode> {
    let indent = "  ".repeat(level);
    match node {
        XMLNode::Text(ref txt) => {
            println!("  {}text: {}", indent, txt.trim());
            Some(node)
        }
        XMLNode::Element(mut elem) => {
            println!("{}node: {}", indent, elem.name);
            let mut remove = elem.get_child("zutaten").is_none();
            elem.children = elem
                .children
                .into_iter()
                .filter_map(|child| {
                    let Some(child) = process_node(child, level + 1, duration_sum, duration_count)
                    else {
                        remove = true;
                        return None;
                    };
                    Some(child)
                })
                .collect();

            match (elem.name.as_str(), remove) {
                ("zutaten", _) => match elem.children.len() {
                    3.. => Some(XMLNode::Element(elem)),
                    _ => None,
                },
                ("zubereitung", _) => {
                    (*duration_sum, *duration_count) = match elem
                        .attributes
                        .get("dauer")
                        .unwrap_or(&"0".to_string())
                        .split_whitespace()
                        .collect::<Vec<_>>()[..]
                    {
                        [t, "min"] => (
                            *duration_sum + t.parse::<f64>().unwrap() * 60.,
                            *duration_count + 1,
                        ),
                        [t, "s"] => (
                            *duration_sum + t.parse::<f64>().unwrap(),
                            *duration_count + 1,
                        ),
                        _ => (*duration_sum, *duration_count),
                    };

                    Some(XMLNode::Element(elem))
                }
                ("rezept", true) => None,
                _ => Some(XMLNode::Element(elem)),
            }
        }
        _ => Some(node),
    }
}
