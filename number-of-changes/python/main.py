def count_changes(string: str) -> int:
	string = string.lower()
	prev_curr = zip(string, string[1:])
	return sum(1 for prev, curr in prev_curr if prev != curr)


def main():
	assert count_changes("a") == 0
	assert count_changes("ab") == 1
	assert count_changes("aA") == 0
	assert count_changes("aAa") == 0
	assert count_changes("aBbc") == 2
	assert count_changes("abCdE") == 4


if __name__ == "__main__":
	main()
