use std::sync::atomic::{AtomicU64, Ordering};

use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

#[derive(Component, Deref, DerefMut)]
pub struct Age(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Amount(pub AtomicU64);

impl Clone for Amount {
    fn clone(&self) -> Self {
        Self(AtomicU64::new(self.0.load(Ordering::SeqCst)))
    }
}
impl Amount {
    pub fn take(&self, mut amount: u64) -> Self {
        loop {
            let before = self.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                if x >= amount {
                    Some(x - amount)
                } else {
                    None
                }
            });

            match before {
                Ok(_) => break Self(AtomicU64::new(amount)),
                Err(x) => {
                    amount = x;
                }
            }
        }
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Amount(AtomicU64::new(value))
    }
}
