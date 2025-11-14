use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::io::Cursor;

pub fn write_start_tag(writer: &mut Writer<Cursor<Vec<u8>>>, element: &BytesStart<'_>) {
    writer
        .write_event(Event::Start(element.to_owned()))
        .expect("Unable to write start tag");
}

pub fn write_empty_tag(writer: &mut Writer<Cursor<Vec<u8>>>, element: &BytesStart<'_>) {
    writer
        .write_event(Event::Empty(element.to_owned()))
        .expect("Unable to write self-closing tag");
}

pub fn write_end_tag(writer: &mut Writer<Cursor<Vec<u8>>>, element: &BytesEnd<'_>) {
    writer
        .write_event(Event::End(element.to_owned()))
        .expect("Unable to write end tag");
}

pub fn write_content(writer: &mut Writer<Cursor<Vec<u8>>>, s: &str) {
    writer
        .write_event(Event::Text(BytesText::new(s)))
        .expect("Unable to write text");
}