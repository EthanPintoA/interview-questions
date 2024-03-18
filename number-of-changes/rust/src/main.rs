fn count_changes(string: &str) -> usize {
	let string = string.to_ascii_lowercase();
	string
		.chars()
		.zip(string.chars().skip(1))
		.filter(|(prev, curr)| prev != curr)
		.count()
}

fn main() {
	assert_eq!(count_changes("a"), 0);
	assert_eq!(count_changes("ab"), 1);
	assert_eq!(count_changes("aAa"), 0);
	assert_eq!(count_changes("aBbc"), 2);
	assert_eq!(count_changes("abCdE"), 4);
}
