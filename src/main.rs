#![recursion_limit = "256"]

trait Trait {
    #[must_use]
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}
pub struct S0;
impl Trait for S0 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S1;
impl Trait for S1 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S2;
impl Trait for S2 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S3;
impl Trait for S3 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S4;
impl Trait for S4 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S5;
impl Trait for S5 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S6;
impl Trait for S6 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S7;
impl Trait for S7 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S8;
impl Trait for S8 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S9;
impl Trait for S9 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S10;
impl Trait for S10 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S11;
impl Trait for S11 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S12;
impl Trait for S12 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S13;
impl Trait for S13 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S14;
impl Trait for S14 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S15;
impl Trait for S15 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S16;
impl Trait for S16 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S17;
impl Trait for S17 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S18;
impl Trait for S18 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S19;
impl Trait for S19 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S20;
impl Trait for S20 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S21;
impl Trait for S21 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S22;
impl Trait for S22 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S23;
impl Trait for S23 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S24;
impl Trait for S24 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S25;
impl Trait for S25 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S26;
impl Trait for S26 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S27;
impl Trait for S27 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S28;
impl Trait for S28 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
pub struct S29;
impl Trait for S29 {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                entry_point().await;
            };
        })
    }
}
fn main() {}

pub async fn entry_point() {
    let x = X23::new();
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
struct X10 {
    x: X9,
}
impl X10 {
    fn new() -> Self {
        Self {
            x: X9::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X11 {
    x: X10,
}
impl X11 {
    fn new() -> Self {
        Self {
            x: X10::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X12 {
    x: X11,
}
impl X12 {
    fn new() -> Self {
        Self {
            x: X11::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X13 {
    x: X12,
}
impl X13 {
    fn new() -> Self {
        Self {
            x: X12::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X14 {
    x: X13,
}
impl X14 {
    fn new() -> Self {
        Self {
            x: X13::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X15 {
    x: X14,
}
impl X15 {
    fn new() -> Self {
        Self {
            x: X14::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X16 {
    x: X15,
}
impl X16 {
    fn new() -> Self {
        Self {
            x: X15::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X17 {
    x: X16,
}
impl X17 {
    fn new() -> Self {
        Self {
            x: X16::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X18 {
    x: X17,
}
impl X18 {
    fn new() -> Self {
        Self {
            x: X17::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X19 {
    x: X18,
}
impl X19 {
    fn new() -> Self {
        Self {
            x: X18::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X20 {
    x: X19,
}
impl X20 {
    fn new() -> Self {
        Self {
            x: X19::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X21 {
    x: X20,
}
impl X21 {
    fn new() -> Self {
        Self {
            x: X20::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X22 {
    x: X21,
}
impl X22 {
    fn new() -> Self {
        Self {
            x: X21::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
struct X23 {
    x: X22,
}
impl X23 {
    fn new() -> Self {
        Self {
            x: X22::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}
