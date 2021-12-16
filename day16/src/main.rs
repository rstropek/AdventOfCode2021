use aoc_utils::{print_day_header, read_input_file_into_bytes};
use bitvec::prelude::*;

fn parse_input(input: &[u8]) -> Vec<u8> {
    fn char_to_byte(c: u8) -> u8 {
        match c {
            digit if (b'0'..=b'9').contains(&digit) => digit - b'0',
            digit if (b'A'..=b'F').contains(&digit) => digit - b'A' + 10,
            _ => panic!("Invalid char")
        }
    }

    let number_of_bytes = input.len() / 2;
    let mut result = Vec::<u8>::with_capacity(number_of_bytes);
    for n in 0..number_of_bytes {
        result.push(char_to_byte(input[n * 2]) << 4 | char_to_byte(input[n * 2 + 1]));
    }

    result
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Header {
    version: u8,
    type_id: u8,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Operator {
    header: Header,
    packets: Vec<Packet>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Literal {
    header: Header,
    value: u64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

fn read_header(bits: &BitSlice::<Msb0, u8>, bit_index: &mut usize) -> Header {
    let h = Header{
        version: bits[*bit_index..*bit_index + 3].load_be(),
        type_id: bits[*bit_index + 3.. *bit_index + 6].load_be(),
    };
    *bit_index += 6;
    h
}

fn read_literal(header: Header, bits: &BitSlice::<Msb0, u8>, bit_index: &mut usize) -> Packet {
    let mut value = 0u64;
    //let start_index = *bit_index;

    loop {
        // Read 4 bits of literal and combine it with existing value
        let num: u8 = bits[*bit_index + 1..*bit_index + 5].load_be();
        value = value << 4 | num as u64;
        *bit_index += 5;

        // Check if there are additional 4 bit chunks
        if !bits[*bit_index - 5] {
            // No additional chunks -> stop
            break;
        }
    }

    // Return literal
    Packet::Literal(Literal{header, value})
}

fn read_operator(header: Header, bits: &BitSlice::<Msb0, u8>, bit_index: &mut usize) -> Packet {
    // End conditions (one of them will be set)
    let mut number_of_sub_packets = 0usize;
    let mut length = 0usize;

    // Interpret length type id
    if bits[*bit_index] {
        // Number of sub-packets
        number_of_sub_packets = bits[*bit_index + 1..*bit_index + 1 + 11].load_be();
        *bit_index += 1 + 11;
    } else {
        // Length in bits
        length = bits[*bit_index + 1..*bit_index + 1 + 15].load_be();
        *bit_index += 1 + 15;
    }

    // Create variable for receiving subpackets
    let mut packets: Vec<Packet>;
    if number_of_sub_packets!= 0 {
        packets = Vec::with_capacity(number_of_sub_packets);
    } else {
        packets = Vec::new();
    }
    
    let sub_packets_start = *bit_index;
    let mut packets_read = 0usize;
    while (number_of_sub_packets == 0 || packets_read < number_of_sub_packets)
        && (length == 0 || *bit_index - sub_packets_start < length) {
        packets.push(dispatch(bits, bit_index));
        packets_read += 1;
    }

    Packet::Operator(Operator{header, packets})
}

/// Reads the header and decides based on it what to read
fn dispatch(bits: &BitSlice::<Msb0, u8>, bit_index: &mut usize) -> Packet {
    let header = read_header(bits, bit_index);
    match header.type_id {
        4 => read_literal(header, bits, bit_index),
        _ => read_operator(header, bits, bit_index)
    }
}

fn version_sum(p: &Packet) -> u32 {
    match p {
        Packet::Literal(v) => v.header.version as u32,
        Packet::Operator(o) => o.header.version as u32 + o.packets.iter().map(|o| version_sum(o)).sum::<u32>(),
    }
}

fn evaluate(p: &Packet) -> u64 {
    match p {
        Packet::Literal(v) => v.value as u64,
        Packet::Operator(o) => match o.header.type_id {
            0 => o.packets.iter().map(|o| evaluate(o)).sum::<u64>(),
            1 => o.packets.iter().map(|o| evaluate(o)).product(),
            2 => o.packets.iter().map(|o| evaluate(o)).min().unwrap(),
            3 => o.packets.iter().map(|o| evaluate(o)).max().unwrap(),
            5 => if evaluate(&o.packets[0]) > evaluate(&o.packets[1]) { 1 } else { 0 },
            6 => if evaluate(&o.packets[0]) < evaluate(&o.packets[1]) { 1 } else { 0 },
            7 => if evaluate(&o.packets[0]) == evaluate(&o.packets[1]) { 1 } else { 0 },
            _ => panic!("Invalid type id")
        },
    }
}

fn main() {
    print_day_header(16);

    let input = read_input_file_into_bytes(16);
    let values = parse_input(&input);
    let bits = BitSlice::<Msb0, u8>::from_slice(&values).unwrap();

    let mut bit_index = 0usize;
    let packet = dispatch(bits, &mut bit_index);

    // Star 1
    println!("  Result Star 1: {:?}", version_sum(&packet));

    // Star 2
    println!("  Result Star 2: {:?}", evaluate(&packet));
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    const TEST_INPUT_SHORT: &'static [u8] = b"8A004A801A8002F478";

    #[test]
    fn test_parse() {
        let result = parse_input(TEST_INPUT_SHORT);
        assert_eq!(vec![0x8A, 0x00, 0x4A, 0x80, 0x1A, 0x80, 0x02, 0xF4, 0x78], result);
    }

    
    #[test]
    fn test_header() {
        let input = parse_input(b"D2FE28");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();
        let mut bit_index = 0usize;
        let h = read_header(bits, &mut bit_index);
        assert_eq!(4, h.type_id);
        assert_eq!(6, h.version);
    }

    #[test]
    fn test_literal() {
        let input = parse_input(b"D2FE28");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let header = read_header(bits, &mut bit_index);
        let packet = read_literal(header.clone(), bits, &mut bit_index);
        assert_eq!(Packet::Literal(Literal{header, value: 2021}), packet);
        assert_eq!(6, version_sum(&packet));
    }

    macro_rules! try_parse_packet {
        ($name:ident is $tpack:ident $(if $s:stmt)*) => {
            let pack = $name;
            let $name;
            if let Packet::$tpack(p) = pack {
                $name = p;
                $($s)*
            }
            else {
                assert!(false, "unexpected packet type");
                return;
            }
        };
    }
    
    #[test]
    fn test_operator() {
        let input = parse_input(b"38006F45291200");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);

        try_parse_packet!(packet is Operator 
            if assert_eq!(6, packet.header.type_id)
            if assert_eq!(2, packet.packets.len()));
        let mut packet_iter = packet.packets.into_iter();
        let lit = packet_iter.next().unwrap();
        try_parse_packet!(lit is Literal if assert_eq!(10, lit.value));
        let lit = packet_iter.next().unwrap();
        try_parse_packet!(lit is Literal if assert_eq!(20, lit.value));
    }

    #[test]
    fn test_operator_2() {
        let input = parse_input(b"EE00D40C823060");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);

        try_parse_packet!(packet is Operator 
            if assert_eq!(7, packet.header.version)
            if assert_eq!(3, packet.header.type_id)
            if assert_eq!(3, packet.packets.len()));
        let mut packet_iter = packet.packets.into_iter();
        let lit = packet_iter.next().unwrap();
        try_parse_packet!(lit is Literal if assert_eq!(1, lit.value));
        let lit = packet_iter.next().unwrap();
        try_parse_packet!(lit is Literal if assert_eq!(2, lit.value));
        let lit = packet_iter.next().unwrap();
        try_parse_packet!(lit is Literal if assert_eq!(3, lit.value));
    }

    #[test]
    fn test_nested_operator() {
        let input = parse_input(b"8A004A801A8002F478");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);

        try_parse_packet!(packet is Operator 
            if assert_eq!(4, packet.header.version)
            if assert_eq!(1, packet.packets.len()));
        let packet = packet.packets.first().unwrap();
        try_parse_packet!(packet is Operator 
            if assert_eq!(1, packet.header.version)
            if assert_eq!(1, packet.packets.len()));
        let packet = packet.packets.first().unwrap();
        try_parse_packet!(packet is Operator 
            if assert_eq!(5, packet.header.version)
            if assert_eq!(1, packet.packets.len()));
        let lit = packet.packets.first().unwrap();
        try_parse_packet!(lit is Literal if assert_eq!(6, lit.header.version));
    }

    #[test]
    fn test_version_sum_1() {
        let input = parse_input(b"8A004A801A8002F478");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);
        assert_eq!(16, version_sum(&packet))
    }

    #[test]
    fn test_version_sum_2() {
        let input = parse_input(b"620080001611562C8802118E34");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);
        assert_eq!(12, version_sum(&packet))
    }

    #[test]
    fn test_version_sum_3() {
        let input = parse_input(b"C0015000016115A2E0802F182340");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);
        assert_eq!(23, version_sum(&packet))
    }

    #[test]
    fn test_version_sum_4() {
        let input = parse_input(b"A0016C880162017C3686B18A3D4780");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);
        assert_eq!(31, version_sum(&packet))
    }

    #[test]
    fn test_bit_load() {
        let input = vec![0b10000001, 0b10110000];
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();
        assert_eq!(27u16, bits[1..1 + 11].load_be());
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_evaluate_1() {
        let input = parse_input(b"C200B40A82");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);
        assert_eq!(3, evaluate(&packet))
    }

    #[test]
    fn test_evaluate_2() {
        let input = parse_input(b"880086C3E88112");
        let bits = BitSlice::<Msb0, u8>::from_slice(&input).unwrap();

        let mut bit_index = 0usize;
        let packet = dispatch(bits, &mut bit_index);
        assert_eq!(7, evaluate(&packet))
    }
}
