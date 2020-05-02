#[derive(Debug, Default)]
pub struct Node<T> {
    uuid: Option<String>,
    pub server: T,
}

impl<T> Node<T>
where
    T: Runner,
{
    pub fn new(server: T) -> Self {
        Node { uuid: None, server }
    }

    pub fn get_uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }

    pub fn set_uuid(&mut self, uuid: &str) -> &mut Self {
        self.uuid = Some(uuid.to_string());

        self
    }
}

pub trait Runner {}
