pub mod ip;

use data::space::Space;
use self::ip::Ip;

pub struct Program {
    space: Space,
    ips: Vec<Ip>,
}
