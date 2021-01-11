use super::Runtime;

pub trait Connect<Rt>: crate::Connect<Rt>
where
    Rt: Runtime,
{
    fn connect(url: &str) -> crate::Result<Self>
    where
        Self: Sized;
}

// TODO: impl Connect for Pool { ... }
// TODO: impl Connect for PgConnection { ... }
