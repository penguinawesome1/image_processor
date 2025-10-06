use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Channel(pub u8);

impl Channel {
    pub fn from_normal(n: f32) -> Self {
        Self((n * 255.0 + 0.5).clamp(0.0, 255.0) as u8)
    }

    pub fn to_normal(&self) -> f32 {
        self.0 as f32 / 255.0
    }
}

impl ops::Add for Channel {
    type Output = Channel;

    fn add(self, other: Channel) -> Channel {
        Channel(self.0.saturating_add(other.0))
    }
}

impl ops::Sub for Channel {
    type Output = Channel;

    fn sub(self, other: Channel) -> Channel {
        Channel(self.0.saturating_sub(other.0))
    }
}

impl ops::Mul for Channel {
    type Output = Channel;

    fn mul(self, other: Channel) -> Channel {
        Channel::from_normal(self.to_normal() * other.to_normal())
    }
}

impl ops::Add<u8> for Channel {
    type Output = Channel;

    fn add(self, n: u8) -> Channel {
        Channel(self.0.saturating_add(n))
    }
}

impl ops::Sub<u8> for Channel {
    type Output = Channel;

    fn sub(self, n: u8) -> Channel {
        Channel(self.0.saturating_sub(n))
    }
}

impl ops::Mul<f32> for Channel {
    type Output = Channel;

    fn mul(self, n: f32) -> Channel {
        Channel::from_normal(self.to_normal() * n)
    }
}
