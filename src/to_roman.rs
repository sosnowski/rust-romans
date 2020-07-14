struct RomanLiteral (usize, &'static str);

const ROMAN_SYMBOLS: [RomanLiteral; 13] = [
    RomanLiteral(1000, "M"),
    RomanLiteral(900, "CM"),
    RomanLiteral(500, "D"),
    RomanLiteral(400, "CD"),
    RomanLiteral(100, "C"),
    RomanLiteral(90, "XC"),
    RomanLiteral(50, "L"),
    RomanLiteral(40, "XL"),
    RomanLiteral(10, "X"),
    RomanLiteral(9, "IX"),
    RomanLiteral(5, "V"),
    RomanLiteral(4, "IV"),
    RomanLiteral(1, "I"),
];

pub fn numeric_to_roman(numeric: usize) -> String {
    let mut result = String::new();
    let mut number = numeric;

    for roman in ROMAN_SYMBOLS.iter() {
        while number >= roman.0 {
            result.push_str(roman.1);
            number -= roman.0;
        }
    }

    return result;
} 

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_first_three() {
        assert_eq!(numeric_to_roman(1), "I");
        assert_eq!(numeric_to_roman(2), "II");
        assert_eq!(numeric_to_roman(3), "III");
    }

    #[test]
    fn test_four_and_five() {
        assert_eq!(numeric_to_roman(4), "IV");
        assert_eq!(numeric_to_roman(5), "V");
    }

    #[test]
    fn test_six_to_eight() {
        assert_eq!(numeric_to_roman(6), "VI");
        assert_eq!(numeric_to_roman(7), "VII");
        assert_eq!(numeric_to_roman(8), "VIII");
    }

    #[test]
    fn test_nine_to_ten() {
        assert_eq!(numeric_to_roman(9), "IX");
        assert_eq!(numeric_to_roman(10), "X");
    }

    #[test]
    fn test_ten_to_twenty() {
        assert_eq!(numeric_to_roman(13), "XIII");
        assert_eq!(numeric_to_roman(14), "XIV");
        assert_eq!(numeric_to_roman(15), "XV");
        assert_eq!(numeric_to_roman(18), "XVIII");
        assert_eq!(numeric_to_roman(19), "XIX");
        assert_eq!(numeric_to_roman(20), "XX");
    }

    #[test]
    fn test_complicated() {
        assert_eq!(numeric_to_roman(1984), "MCMLXXXIV");
    }
}