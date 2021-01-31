use crate::Buffer;

#[derive(Debug, Clone)]
pub struct Subscription {
    pub url: String,
}

impl From<Vec<Subscription>> for Buffer {
    fn from(items: Vec<Subscription>) -> Self {
        Buffer {
            cx: 1,
            cy: 1,
            rows: items.iter().map(|x| x.url.clone()).collect::<Vec<_>>(),
        }
    }
}
