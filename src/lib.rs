pub mod gtp_v1;

mod field {
    pub type Field = ::core::ops::Range<usize>;
}

pub const MTU: usize = 1500;
