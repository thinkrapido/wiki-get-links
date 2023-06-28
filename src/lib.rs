use async_parse_wiki_text::Node;

pub fn get_links<'a>(vec: &'a mut Vec<String>, nodes: &'a Vec<Node<'a>>) {

    for node in nodes {
        match node {
            Node::Link { target, text, ..} => {
                vec.push(target.to_string());
                get_links(vec, text);
            },
            Node::Category { ordinal, ..} => {
                get_links(vec, ordinal);
            },
            Node::ExternalLink { nodes, ..} => {
                get_links(vec, nodes);
            },
            Node::Heading { nodes, ..} => {
                get_links(vec, nodes);
            },
            Node::Image { text, ..} => {
                get_links(vec, text);
            },
            Node::Parameter { default, name, ..} => {
                if let Some(default) = default {
                    get_links(vec, default);
                }
                get_links(vec, name);
            },
            Node::Preformatted { nodes, ..} => {
                get_links(vec, nodes);
            },
            Node::Table { attributes, ..} => {
                get_links(vec, attributes);
            },
            Node::Tag { nodes, ..} => {
                get_links(vec, nodes);
            },
            _ => {}
        }
    }

}
