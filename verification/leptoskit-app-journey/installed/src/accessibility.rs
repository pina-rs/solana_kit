//! Read only public native accessibility captures saved by NativeTester.
use crate::{Error, Result, require};
use leptoskit_testing::native::NativeTester;
use quick_xml::{
    Reader, XmlVersion,
    events::{BytesStart, Event},
};
use std::{collections::BTreeMap, fs};

/// Select exactly one matching labelled native element in the latest capture.
pub(crate) fn value(tester: &NativeTester, identifier: &str) -> Result<String> {
    let capture = fs::read_dir(tester.artifacts())?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|extension| extension == "xml"))
        .max()
        .ok_or_else(|| Error::Response("no actual native accessibility capture".into()))?;
    let xml = fs::read_to_string(capture)?;
    parse(&xml, identifier)
}

fn parse(xml: &str, identifier: &str) -> Result<String> {
    let mut reader = Reader::from_str(xml);
    let mut values = Vec::new();

    loop {
        let event = reader
            .read_event()
            .map_err(|error| Error::Response(error.to_string()))?;

        match event {
            Event::Start(element) | Event::Empty(element) => {
                values.extend(labelled_value(&element, identifier)?);
            }
            Event::Eof => break,
            _ => {}
        }
    }

    require(values.len() == 1, "native public value must be unambiguous")?;
    Ok(values.remove(0))
}

fn labelled_value(element: &BytesStart<'_>, identifier: &str) -> Result<Option<String>> {
    let attributes = attributes(element)?;
    let selected = ["name", "identifier", "resource-id"].iter().any(|key| {
        attributes
            .get(*key)
            .is_some_and(|value| value == identifier)
    });

    if !selected {
        return Ok(None);
    }

    let text = ["value", "text", "label"]
        .into_iter()
        .find_map(|key| attributes.get(key).filter(|value| !value.is_empty()))
        .ok_or_else(|| Error::Response("labelled native element has no text".into()))?;
    Ok(Some(text.clone()))
}

fn attributes(element: &BytesStart<'_>) -> Result<BTreeMap<String, String>> {
    let mut values = BTreeMap::new();

    for attribute in element.attributes() {
        let attribute = attribute.map_err(|error| Error::Response(error.to_string()))?;
        let name = attribute.key.as_ref().to_owned();
        let value = attribute
            .normalized_value(XmlVersion::Implicit1_0)
            .map_err(|error| Error::Response(error.to_string()))?
            .into_owned();
        values.insert(name, value);
    }

    Ok(values)
}
