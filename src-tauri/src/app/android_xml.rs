pub mod axml {
    use axmldecoder::{Cdata, Element, Node};

    pub fn extract_xml(content: Vec<u8>) -> Result<String,String> {
        let xml = axmldecoder::parse(content.as_slice()).map_err(|e|e.to_string())?;
        let root = xml.get_root().as_ref().unwrap();
        let mut xml_as_string = String::new();
        format_xml(root, 0_usize, &mut xml_as_string);
        Ok(xml_as_string)
    }

    fn format_xml(e: &Node, level: usize, output: &mut String) {
        match e {
            Node::Element(e) => {
                output.push_str(&format!(
                    "{:indent$}{}\n",
                    "",
                    &format_start_element(e),
                    indent = level * 2
                ));

                for child in e.get_children() {
                    format_xml(child, level + 1, output)
                }

                if !e.get_children().is_empty() {
                    output.push_str(&format!(
                        "{:indent$}{}\n",
                        "",
                        format_end_element(e),
                        indent = level * 2
                    ));
                }
            }
            Node::Cdata(e) => {
                output.push_str(&format!(
                    "{:indent$}{}\n",
                    "",
                    &format_cdata(e, level),
                    indent = level * 2
                ));
            }
        }
    }

    fn format_cdata(e: &Cdata, level: usize) -> String {
        let indent = format!("{:indent$}", "", indent = level * 2);
        let mut s = String::new();
        s.push_str("<![CDATA[");
        s.push_str(&e.get_data().replace('\n', &format!("\n{}", &indent)));
        s.push_str("]]>");
        s
    }

    fn format_start_element(e: &Element) -> String {
        let mut s = String::new();
        s.push('<');
        s.push_str(e.get_tag());

        if e.get_tag() == "manifest" {
            s.push(' ');
            s.push_str("xmlns:android=\"http://schemas.android.com/apk/res/android\"");
        }

        for (key, val) in e.get_attributes().iter() {
            s.push(' ');
            s.push_str(key);
            s.push('=');
            s.push('"');
            s.push_str(val);
            s.push('"');
        }

        if e.get_children().is_empty() {
            s.push('/');
        }

        s.push('>');

        s
    }

    fn format_end_element(e: &Element) -> String {
        let mut s = String::new();
        s.push('<');
        s.push('/');
        s.push_str(e.get_tag());
        s.push('>');
        s
    }
}
