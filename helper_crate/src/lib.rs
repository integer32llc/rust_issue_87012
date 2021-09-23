pub async fn entry_point() {
    let x = X9::new();
    x.f().await;
}

async fn term_async() -> u32 {
    1
}

fn term_sync() -> u32 {
    1
}

struct X0 {}
impl X0 {
    fn new() -> Self {
        Self {}
    }

    async fn f(&self) -> u32 {
        term_sync() + term_async().await
    }
}

struct X1 {
    x: X0,
}
impl X1 {
    fn new() -> Self {
        Self {
            x: X0::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X2 {
    x: X1,
}
impl X2 {
    fn new() -> Self {
        Self {
            x: X1::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X3 {
    x: X2,
}
impl X3 {
    fn new() -> Self {
        Self {
            x: X2::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X4 {
    x: X3,
}
impl X4 {
    fn new() -> Self {
        Self {
            x: X3::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X5 {
    x: X4,
}
impl X5 {
    fn new() -> Self {
        Self {
            x: X4::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X6 {
    x: X5,
}
impl X6 {
    fn new() -> Self {
        Self {
            x: X5::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X7 {
    x: X6,
}
impl X7 {
    fn new() -> Self {
        Self {
            x: X6::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X8 {
    x: X7,
}
impl X8 {
    fn new() -> Self {
        Self {
            x: X7::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}

struct X9 {
    x: X8,
}
impl X9 {
    fn new() -> Self {
        Self {
            x: X8::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
