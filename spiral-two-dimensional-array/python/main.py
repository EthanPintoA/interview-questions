Vec2 = tuple[int, int]

DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def out_of_bounds(position: Vec2, bounds: tuple[Vec2, Vec2]) -> bool:
	"""Return True if the position is out of bounds."""
	pos = position
	min, max = bounds  # Shadowing built-in function
	in_bounds = (min[0] <= pos[0] < max[0]) and (min[1] <= pos[1] < max[1])
	return not in_bounds


def decrease_bounds(bounds: tuple[Vec2, Vec2], turned_to: Vec2) -> tuple[Vec2, Vec2]:
	"""
	Decrease the matrix size based on the direction we're turning.
	E.g. if we turn down, we decrease the top row by 1.
	"""
	min, max = bounds  # Shadowing built-in function
	min_x, min_y = min
	max_x, max_y = max
	match turned_to:
		case (0, 1):
			min_y += 1
		case (1, 0):
			min_x += 1
		case (0, -1):
			max_y -= 1
		case (-1, 0):
			max_x -= 1
		case _:
			raise ValueError(f"Invalid direction: {turned_to}")

	return ((min_x, min_y), (max_x, max_y))


def spiral(matrix: list[list[int]]) -> list[int]:
	"""Return the elements of the matrix in a spiral order."""
	result = [matrix[0][0]]

	bounds = ((0, 0), (len(matrix), len(matrix[0])))
	pos = (0, 0)
	dir_idx = 0
	dir = DIRECTIONS[dir_idx]

	while True:
		next_pos = (pos[0] + dir[0], pos[1] + dir[1])

		if out_of_bounds(next_pos, bounds):
			dir_idx = (dir_idx + 1) % 4
			dir = DIRECTIONS[dir_idx]
			next_pos = (pos[0] + dir[0], pos[1] + dir[1])

			# If the next position is also out of bounds, then we're done
			if out_of_bounds(next_pos, bounds):
				return result

			bounds = decrease_bounds(bounds, dir)

		pos = next_pos
		result.append(matrix[pos[0]][pos[1]])


def main():
	# fmt: off
	assert spiral(
		[
			[1, 2, 3],
			[4, 5, 6],
			[7, 8, 9],
		]
	) == [1, 2, 3, 6, 9, 8, 7, 4, 5]

	assert spiral(
		[
			[ 1,  2,  3,  4],
			[ 5,  6,  7,  8],
			[ 9, 10, 11, 12],
			[13, 14, 15, 16],
		]
	) == [1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]

	assert spiral(
		[
			[ 1,  2,  3,  4,  5],
			[ 6,  7,  8,  9, 10],
			[11, 12, 13, 14, 15],
			[16, 17, 18, 19, 20],
		]
	) == [1, 2, 3, 4, 5, 10, 15, 20, 19, 18, 17, 16, 11, 6, 7, 8, 9, 14, 13, 12]
	# fmt: on


if __name__ == "__main__":
	main()
