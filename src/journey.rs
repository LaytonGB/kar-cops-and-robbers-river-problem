use std::fmt;
struct Journey(pub [u8; 2]);

impl fmt::Debug for Journey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        let debug_names = ["cop", "robber"];

        for i in 0..2 {
            if self.0[i] != 0 {
                debug_str.push_str(format!("{}: ", debug_names[i]).as_str());
                match self.0[i] {
                    2 => debug_str.push_str("2 move Left"),
                    1 => debug_str.push_str("1 moves Left"),
                    n if n == !0 => debug_str.push_str("1 moves Right"),
                    n if n == !1 => debug_str.push_str("2 move Right"),
                    _ => return Err(fmt::Error),
                }
                debug_str.push_str(", ");
            }
        }

        write!(f, "{}", debug_str)
    }
}
