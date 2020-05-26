pub trait Runner {}

#[derive(Debug, Default)]
pub struct Node<T> {
    uuid: Option<String>,
    url: Option<String>,
    pub server: T,
}

impl<T> Node<T>
where
    T: Runner,
{
    pub fn new(server: T) -> Self {
        Node {
            uuid: None,
            url: None,
            server,
        }
    }

    pub fn get_uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }

    pub fn set_uuid(&mut self, uuid: &str) -> &mut Self {
        self.uuid = Some(uuid.to_string());

        self
    }

    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());

        self
    }
}

#[derive(Clone, Debug)]
pub struct NeighbourServer;

impl Runner for NeighbourServer {}
