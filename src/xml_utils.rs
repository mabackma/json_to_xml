use crate::error::ConversionError;
use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesStart, BytesEnd, BytesText, Event};
use std::io::Cursor;

/// Write XML declaration
pub fn write_declaration(writer: &mut Writer<Cursor<Vec<u8>>>, xml_version: &str, encoding: Option<&str>) -> Result<(), ConversionError> {
    writer
        .write_event(Event::Decl(BytesDecl::new(xml_version, encoding, None)))?;
    Ok(())
}

/// Write Comment
pub fn write_comment(writer: &mut Writer<Cursor<Vec<u8>>>, version: &str) -> Result<(), ConversionError> {
    writer
        .write_event(
            Event::Comment(BytesText::new(version)))?;
    Ok(())
}

/// Write start tag
pub fn write_start_tag(writer: &mut Writer<Cursor<Vec<u8>>>, element: &BytesStart<'_>) -> Result<(), ConversionError> {
    writer
        .write_event(Event::Start(element.to_owned()))?;
    Ok(())
}

/// Write empty tag
pub fn write_empty_tag(writer: &mut Writer<Cursor<Vec<u8>>>, element: &BytesStart<'_>) -> Result<(), ConversionError> {
    writer
        .write_event(Event::Empty(element.to_owned()))?;
    Ok(())
}

/// Write end tag
pub fn write_end_tag(writer: &mut Writer<Cursor<Vec<u8>>>, element: &BytesEnd<'_>) -> Result<(), ConversionError> {
    writer
        .write_event(Event::End(element.to_owned()))?;
    Ok(())
}

/// Write text content between tags
pub fn write_content(writer: &mut Writer<Cursor<Vec<u8>>>, s: &str) -> Result<(), ConversionError> {
    writer
        .write_event(Event::Text(BytesText::new(s)))?;
    Ok(())
}