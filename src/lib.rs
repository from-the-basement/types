#![feature(split_array)]
pub mod array;
pub mod bitvec;
pub mod primitive;
pub mod scalar;

#[cfg(test)]
mod tests {
    use crate::array::{Array, OptionListArray};

    pub type UInt8Field = OptionListArray<u8>;

    #[test]
    fn rate_field() {
        let mut array = UInt8Field::new(32);
        array.push((0..32).map(Some).collect::<Vec<_>>().into());
        let row = array.get(0).unwrap();
        let lhs = row.slice(1..32);
        let rhs = row.slice(0..31);
        dbg!(lhs - rhs);
    }
}
