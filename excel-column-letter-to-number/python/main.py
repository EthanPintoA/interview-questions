def excel_column_to_number(column: str) -> int:
	"""
	Convert a excel column value to its corresponding number.
	Similar to converting from base 26 to decimal.
	Example: 'A' -> 1, 'Z' -> 26, 'AA' -> 27, 'AB' -> 28
	"""
	result = 0
	for letter in column:
		digit = ord(letter) - ord("A") + 1  # Convert 'A' -> 1, 'B' -> 2, ..., 'Z' -> 26
		result = result * 26 + digit
	return result


def main():
	assert excel_column_to_number("A") == 1
	assert excel_column_to_number("Z") == 26
	assert excel_column_to_number("AA") == 27  # 1 * 26 + 1
	assert excel_column_to_number("AZ") == 52  # 1 * 26 + 26
	assert excel_column_to_number("AYY") == 1351  # 1 * (26 * 26) + 25 * 26 + 25


if __name__ == "__main__":
	main()
