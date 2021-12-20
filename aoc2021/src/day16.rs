use std::{fs, str::FromStr};

use itertools::Itertools;

use crate::utils::{AdventError, binary};

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day16a.dat").expect("input file does not exist");
    let bits: Packet = input.parse().expect("invalid input");

    (
        bits.sum_versions(),
        bits.eval(),
    )
}

enum PacketPayload {
    Value(usize),
    Operator(u8, Vec<Packet>)
}

impl PacketPayload {
    fn eval(&self) -> usize {
        match self {
            Self::Value(val) => *val,
            Self::Operator(op, operands) => {
                match op {
                    0 => operands.iter().map(|o| o.eval()).sum(),
                    1 => operands.iter().map(|o| o.eval()).product(),
                    2 => operands.iter().map(|o| o.eval()).min().unwrap(),
                    3 => operands.iter().map(|o| o.eval()).max().unwrap(),
                    5 => if operands[0].eval() > operands[1].eval() {1} else {0},
                    6 => if operands[0].eval() < operands[1].eval() {1} else {0},
                    7 => if operands[0].eval() == operands[1].eval() {1} else {0},
                    _ => unreachable!()
                }
            }
        }
    }
}

struct Packet {
    version: u8,
    payload: PacketPayload
}

struct PacketParser {
    binary: Vec<u8>,
    idx: usize
}

impl PacketParser {
    fn new(line: &str) -> Result<Self, AdventError> {
        let binary = line.trim().chars().map(|letter| {
            let num = u8::from_str_radix(&letter.to_string(), 16);
            num.map(|n| format!("{:0>4b}", n))
        }).collect::<Result<Vec<_>, _>>()?
        .iter()
        .flat_map(|s| s.chars())
        .map(|c| if c == '0' {0} else {1})
        .collect_vec();

        Ok(PacketParser {
            binary,
            idx: 0
        })
    }

    fn parse(&mut self) -> Result<Packet, AdventError> {
        let version = &self.binary[self.idx..self.idx+3];
        self.idx += 3;
        let version = binary::to_u8(version);

        let packet_type = &self.binary[self.idx..self.idx+3];
        self.idx += 3;
        let packet_type = binary::to_u8(packet_type);

        let payload = match packet_type {
            4 => self.parse_value_payload()?,
            op => self.parse_operator_payload(op)?
        };

        Ok(Packet {
            version,
            payload
        })
    }

    fn parse_value_payload(&mut self) -> Result<PacketPayload, AdventError> {
        let mut number: Vec<u8> = Vec::new();

        loop {
            let cont_flag = self.binary[self.idx] == 0;
            let part = &self.binary[self.idx+1..self.idx+5];
            self.idx += 5;

            number.extend(part);
            if cont_flag {
                break;
            }
        }
        let value = binary::to_usize(&number);
        Ok(PacketPayload::Value(value))
    }

    fn parse_operator_payload(&mut self, op: u8) -> Result<PacketPayload, AdventError> {
        let len_type = self.binary[self.idx];
        self.idx += 1;

        let mut packets = Vec::new();

        match len_type {
            0 => {
                // len is bits
                let len = usize::from_str_radix(&self.binary[self.idx..self.idx+15].iter().join(""), 2)?;
                self.idx += 15;
                let limit = self.idx + len;

                while self.idx < limit {
                    packets.push(self.parse()?);
                }
            },
            1 => {
                // len is packets
                let len = usize::from_str_radix(&self.binary[self.idx..self.idx+11].iter().join(""), 2)?;
                self.idx += 11;

                for _ in 0..len {
                    packets.push(self.parse()?);
                }
            },
            e => return Err(AdventError::UnexpectedElement { found: e.to_string(), expected: &["0", "1"] })
        };

        Ok(PacketPayload::Operator(op, packets))
    }
}

impl Packet {
    fn sum_versions(&self) -> usize {
        let mut sum = self.version as usize;
        if let PacketPayload::Operator(_op, data) = &self.payload {
            sum += data.iter().map(|p| p.sum_versions() as usize).sum::<usize>();
        };
        sum
    }

    fn eval(&self) -> usize {
        self.payload.eval()
    }
}

impl FromStr for Packet {
    type Err = AdventError;

    fn from_str(line: &str) -> Result<Self, AdventError> {
        PacketParser::new(line)?.parse()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "D2FE28";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 6);

        let input = "38006F45291200";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 9);

        let input = "EE00D40C823060";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 14);

        let input = "8A004A801A8002F478";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 16);

        let input = "620080001611562C8802118E34";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 12);

        let input = "C0015000016115A2E0802F182340";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 23);

        let input = "A0016C880162017C3686B18A3D4780";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.sum_versions(), 31);

        let input = "C200B40A82";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 3);

        let input = "04005AC33890";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 54);

        let input = "880086C3E88112";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 7);

        let input = "CE00C43D881120";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 9);

        let input = "D8005AC2A8F0";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 1);

        let input = "F600BC2D8F";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 0);

        let input = "9C005AC2F8F0";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 0);

        let input = "9C0141080250320F1802104A08";
        let bits: Packet = input.parse().expect("invalid input");
        assert_eq!(bits.eval(), 1);
    }
}
