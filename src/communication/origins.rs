use core::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Origin {
    name: &'static str,
}

impl Origin {
    pub const fn register(name: &'static str) -> Self {
        Self {
            name,
        }
    }
}

impl PartialEq for Origin {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for Origin {}

impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
