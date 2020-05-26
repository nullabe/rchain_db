use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::node::{NeighbourServer, Node, Runner};

const FIELDS_COUNT: usize = 2;
const FIELDS: [&str; self::FIELDS_COUNT] = ["uuid", "url"];

enum NodeField {
    Uuid,
    Url,
}

struct NodeVisitor;

struct NodeFieldVisitor;

impl<'de> Visitor<'de> for NodeFieldVisitor {
    type Value = NodeField;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("'uuid' and 'url'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "uuid" => Ok(NodeField::Uuid),
            "url" => Ok(NodeField::Url),

            _ => Err(de::Error::unknown_field(value, &self::FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for NodeField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(NodeFieldVisitor)
    }
}

impl<'de> Visitor<'de> for NodeVisitor {
    type Value = Node<NeighbourServer>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct Transaction")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut uuid = None;
        let mut url = None;

        while let Some(key) = map.next_key()? {
            match key {
                NodeField::Uuid => {
                    if uuid.is_some() {
                        return Err(de::Error::duplicate_field("uuid"));
                    }

                    uuid = Some(map.next_value()?);
                }
                NodeField::Url => {
                    if url.is_some() {
                        return Err(de::Error::duplicate_field("url"));
                    }

                    url = Some(map.next_value()?);
                }
            }
        }

        let uuid: &str = uuid.ok_or_else(|| de::Error::missing_field("uuid"))?;
        let url: &str = url.ok_or_else(|| de::Error::missing_field("url"))?;

        let mut node = Node::new(NeighbourServer);

        node.set_uuid(&uuid).set_url(&url);

        Ok(node)
    }
}

impl<'de> Deserialize<'de> for Node<NeighbourServer> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Node", &self::FIELDS, NodeVisitor)
    }
}

impl<T> Serialize for Node<T>
where
    T: Runner,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut node = serializer.serialize_struct("Node", self::FIELDS_COUNT)?;

        node.serialize_field("uuid", &self.get_uuid()).ok();

        node.serialize_field("url", &self.get_url()).ok();

        node.end()
    }
}
