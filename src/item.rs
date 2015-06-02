// Copyright 2015 Corey Farwell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use xml::Element;

use ::{Category, ElementUtils, ViaXml};


/// [RSS 2.0 Specification § Elements of `<item>`]
/// (http://cyber.law.harvard.edu/rss/rss.html#hrelementsOfLtitemgt)
///
/// # Examples
///
/// ```
/// use rss::Item;
///
/// let item = Item {
///     title: Some(String::from("A blog post title")),
///     description: Some(String::from("This is a description of the blog post")),
///     ..Default::default()
/// };
/// ```
#[derive(Default, Debug, Clone)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub categories: Vec<Category>,
    pub comments: Option<String>,
    // pub enclosure
    // pub guid
    pub pub_date: Option<String>,  // add a custom String type to parse this date?
    // pub source
}


impl ViaXml for Item {
    fn to_xml(&self) -> Element {
        let mut item = Element::new("item".to_string(), None, vec![]);

        item.tag_with_optional_text("title", &self.title);
        item.tag_with_optional_text("link", &self.link);
        item.tag_with_optional_text("description", &self.description);
        item.tag_with_optional_text("author", &self.author);
        item.tag_with_optional_text("comments", &self.comments);
        item.tag_with_optional_text("pubDate", &self.pub_date);

        for category in &self.categories {
            item.tag(category.to_xml());
        }

        item
    }

    fn from_xml(elem: Element) -> Result<Self, &'static str> {
        let title = elem.get_child("title", None).map(Element::content_str);
        let link = elem.get_child("link", None).map(Element::content_str);
        let description = elem.get_child("description", None).map(Element::content_str);
        let author = elem.get_child("author", None).map(Element::content_str);
        let comments = elem.get_child("comments", None).map(Element::content_str);
        let pub_date = elem.get_child("pubDate", None).map(Element::content_str);

        let categories = elem.get_children("category", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        Ok(Item {
            title: title,
            link: link,
            description: description,
            categories: categories,
            author: author,
            comments: comments,
            pub_date: pub_date,
        })
    }
}
