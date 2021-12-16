use std::{fs, str::FromStr};

use itertools::Itertools;

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day16a.dat").expect("input file does not exist");
    let bits: Packet = input.parse().expect("invalid input");

    (
        bits.sum_versions(),
        0
    )
}

enum PacketPayload {
    Value(usize),
    Operator(char, Vec<Packet>)
}

struct Packet {
    version: u8,
    packet_type: u8,
    payload: PacketPayload
}

struct PacketParser {
    binary: Vec<char>,
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
        .collect_vec();

        Ok(PacketParser {
            binary,
            idx: 0
        })
    }

    fn parse(&mut self) -> Result<Packet, AdventError> {
        let version = self.binary[self.idx..self.idx+3].iter().join("");
        self.idx += 3;
        let version = u8::from_str_radix(&version, 2)?;

        let packet_type = self.binary[self.idx..self.idx+3].iter().join("");
        self.idx += 3;
        let packet_type = u8::from_str_radix(&packet_type, 2)?;

        let payload = match packet_type {
            4 => self.parse_value_payload()?,
            _ => self.parse_operator_payload()?
        };

        Ok(Packet {
            version,
            packet_type,
            payload
        })
    }

    fn parse_value_payload(&mut self) -> Result<PacketPayload, AdventError> {
        let mut number: Vec<char> = Vec::new();

        loop {
            let cont_flag = self.binary[self.idx] == '0';
            let part = &self.binary[self.idx+1..self.idx+5];
            self.idx += 5;

            number.extend(part);
            if cont_flag {
                break;
            }
        }
        let value = usize::from_str_radix(&number.iter().join(""), 2)?;
        Ok(PacketPayload::Value(value))
    }

    fn parse_operator_payload(&mut self) -> Result<PacketPayload, AdventError> {
        let len_type = self.binary[self.idx];
        self.idx += 1;

        let mut packets = Vec::new();

        match len_type {
            '0' => {
                // len is bits
                let len = usize::from_str_radix(&self.binary[self.idx..self.idx+15].iter().join(""), 2)?;
                self.idx += 15;
                let limit = self.idx + len;


                while self.idx < limit {
                    packets.push(self.parse()?);
                }
            },
            '1' => {
                // len is packets
                let len = usize::from_str_radix(&self.binary[self.idx..self.idx+11].iter().join(""), 2)?;
                self.idx += 11;


                for _ in 0..len {
                    packets.push(self.parse()?);
                }
            },
            e => return Err(AdventError::UnexpectedElement { found: e.to_string(), expected: vec!["0".to_string(), "1".to_string()] })
        };
        Ok(PacketPayload::Operator('x', packets))
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
    }
}
