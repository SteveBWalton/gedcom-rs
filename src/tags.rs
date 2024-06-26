// Modulus to define the 'Tags' class.
// Rust does not have classes!

// Application modules.
use crate::tag;
use tag::Tag;



// Member variables for the 'Tags' class.
pub struct Tags {
    // The tags in this collection.
    pub tags: Vec<Tag>
}



// Functions for the Tags structure.
impl Tags {
    // Create a tags object from the specified gedcom lines.
    pub fn new(level: usize, gedcom: &Vec<String>) -> Tags {
        // Start a collection to hold these tags.
        let mut tags = Vec::new();

        // Loop through the gedcom lines.
        let mut object_lines: Vec<String> = Vec::new();
        let mut current_tag: String = "".to_string();
        let mut current_value: String = "".to_string();
        let line_start = level.to_string();
        for line in gedcom {
            // println!("\t{}", line);
            if line.starts_with(&line_start)
            {
                if current_tag != ""
                {
                    let tag = Tag::new(level, current_tag, current_value, &object_lines);
                    tags.push(tag);
                }

                // Start a new tag.
                // This is not accounting for UTF-8 characters.
                let mut space_pos = 2;
                while line.as_bytes()[space_pos] != 32
                {
                    space_pos += 1;
                    if space_pos == line.len()
                    {
                        break;
                    }
                }
                if space_pos == line.len()
                {
                    current_tag = line[2..space_pos].to_string();
                    current_value = "".to_string();
                }
                else
                {
                    current_tag = line[2..space_pos].to_string();
                    current_value = line[space_pos+1..].to_string();
                }
                object_lines = Vec::new();
            }
            else
            {
                // Add to sub tag lines.
                object_lines.push(line.to_string());
            }
        }

        // The final tag.
        if current_tag != ""
        {
            let tag = Tag::new(level, current_tag, current_value, &object_lines);
            tags.push(tag);
        }

        // Build the Tags object.
        Tags { tags: tags }
    }

    // Create an empty tags object.
    pub fn new_empty() -> Tags {
        Tags { tags: Vec::new() }
    }

    // Returns the collection of tags in lines for a gedcom file.
    pub fn to_gedcom_file(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

        // Add the lines for each tag.
        for tag in &self.tags {
            for child_lines in tag.to_gedcom_file()
            {
                lines.push(child_lines);
            }
        }

        // Return the built lines.
        return lines;
    }

    // Returns the collection of tags in decorated html.
    pub fn to_decorated_html(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

        // Add the lines for each tag.
        for tag in &self.tags {
            for child_lines in tag.to_decorated_html()
            {
                lines.push(child_lines);
            }
        }

        return lines;
    }

    // Return one tag than matches the specified key.
    pub fn find_one(&self, key: &str) -> Option<&Tag> {
        for tag in &self.tags {
            if tag.key == key {
                return Some(tag);
            }
        }
        None
    }
}

