/// Convert a excel column value to its corresponding number.
/// Similar to converting from base 26 to decimal.
/// Example: 'A' -> 1, 'Z' -> 26, 'AA' -> 27, 'AB' -> 28
fn excel_column_to_number(column: &str) -> u32 {
	column
		.chars()
		// Convert each letter to a number (A -> 1, B -> 2, ..., Z -> 26)
		.map(|letter| (letter as u32) - 'A' as u32 + 1)
		.fold(0, |acc, digit| acc * 26 + digit)
}

fn main() {
	assert_eq!(excel_column_to_number("A"), 1);
	assert_eq!(excel_column_to_number("Z"), 26);
	assert_eq!(excel_column_to_number("AA"), 27); // 1 * 26 + 1
	assert_eq!(excel_column_to_number("AZ"), 52); // 1 * 26 + 26
	assert_eq!(excel_column_to_number("AYY"), 1351); // 1 * (26 * 26) + 25 * 26 + 25
}
