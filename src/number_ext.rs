use std::time::Duration;

pub trait SNumExt {
    fn times<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self) -> ();

    fn sec(self) -> Duration;
    fn ms(self) -> Duration;
    fn hour(self) -> Duration;
}

impl SNumExt for u64 {
    fn times<F>(self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self) -> (),
    {
        for i in 1..self {
            f(i);
        }
    }

    fn sec(self) -> Duration {
        Duration::from_secs(self)
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self)
    }

    fn hour(self) -> Duration {
        Duration::from_secs(self * 3600)
    }
}

impl SNumExt for u32 {
    fn times<F>(self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self) -> (),
    {
        for i in 1..self {
            f(i);
        }
    }

    fn sec(self) -> Duration {
        u64::from(self).sec()
    }

    fn ms(self) -> Duration {
        u64::from(self).ms()
    }

    fn hour(self) -> Duration {
        u64::from(self).hour()
    }
}

impl SNumExt for i64 {
    fn times<F>(self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self) -> (),
    {
        for i in 1..self.wrapping_abs() {
            f(i);
        }
    }

    fn sec(self) -> Duration {
        (self.wrapping_abs() as u64).sec()
    }

    fn ms(self) -> Duration {
        (self.wrapping_abs() as u64).ms()
    }

    fn hour(self) -> Duration {
        (self.wrapping_abs() as u64).hour()
    }
}

impl SNumExt for i32 {
    fn times<F>(self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self) -> (),
    {
        for i in 1..self.wrapping_abs() {
            f(i);
        }
    }

    fn sec(self) -> Duration {
        (self.wrapping_abs() as u32).sec()
    }

    fn ms(self) -> Duration {
        (self.wrapping_abs() as u32).ms()
    }

    fn hour(self) -> Duration {
        (self.wrapping_abs() as u32).hour()
    }
}

#[test]
fn test_i32() {
    assert_eq!(3.sec(), Duration::new(3, 0));
    assert_eq!((-3).sec(), Duration::new(3, 0));

    assert_eq!(3.hour(), Duration::new(3 * 3600, 0));
}

#[test]
fn test_u32() {
    assert_eq!(3u32.sec(), Duration::new(3, 0));

    assert_eq!(3u32.hour(), Duration::new(3 * 3600, 0));
}

#[test]
fn test_i64() {
    assert_eq!(3i64.sec(), Duration::new(3, 0));
    assert_eq!((-3i64).sec(), Duration::new(3, 0));

    assert_eq!(3i64.hour(), Duration::new(3 * 3600, 0));
}

#[test]
fn test_u64() {
    assert_eq!(3u64.sec(), Duration::new(3, 0));

    assert_eq!(3u64.hour(), Duration::new(3 * 3600, 0));
}
