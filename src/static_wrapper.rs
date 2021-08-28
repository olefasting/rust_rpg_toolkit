pub struct StaticWrapper<T> {
    pub data: Option<T>,
}

impl<T> StaticWrapper<T> {
    pub fn init(&mut self, data: T) {
        if self.data.is_none() {
            self.data = Some(data);
        }
    }

    pub fn get_data(&'static mut self) -> &'static mut T {
        self.data.as_mut().unwrap()
    }

    pub fn is_initiated(&self) -> bool {
        self.data.is_some()
    }
}
