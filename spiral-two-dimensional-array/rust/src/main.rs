use glam::IVec2;

/// Represents the direction we're moving in:
/// Right, Down, Left, Up
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

/// Returns true if the position is out of bounds.
fn out_of_bounds(position: IVec2, bounds: [IVec2; 2]) -> bool {
	let pos = position;
	let [min, max] = bounds;
	let in_bounds = (min.x..=max.x).contains(&pos.x) && (min.y..=max.y).contains(&pos.y);
	!in_bounds
}

/// Decreases the bounds based on the direction we're turning.
/// E.g. if we turn down, we decrease the top row by 1.
fn decrease_bounds(bounds: &mut [IVec2; 2], turned_to: (i32, i32)) {
	let [mut min, mut max] = bounds;
	match turned_to {
		(0, 1) => min.y += 1,
		(1, 0) => min.x += 1,
		(0, -1) => max.y -= 1,
		(-1, 0) => max.x -= 1,
		_ => panic!("Invalid direction"),
	}
	*bounds = [min, max];
}

fn spiral<const N: usize, const M: usize>(matrix: [[i32; M]; N]) -> Vec<i32> {
	let mut result = Vec::with_capacity(N * M);
	result.push(matrix[0][0]);

	let mut bounds = [IVec2::ZERO, IVec2::new((N - 1) as i32, (M - 1) as i32)];
	let mut pos = IVec2::ZERO;
	let mut dir_idx = 0;
	let mut dir = DIRECTIONS[dir_idx];

	loop {
		let mut next_pos = pos + IVec2::from(dir);

		if out_of_bounds(next_pos, bounds) {
			dir_idx = (dir_idx + 1) % 4;
			dir = DIRECTIONS[dir_idx];
			next_pos = pos + IVec2::from(dir);

			// If the next position is also out of bounds, then we're done
			if out_of_bounds(next_pos, bounds) {
				return result;
			}
			decrease_bounds(&mut bounds, dir);
		}

		pos = next_pos;
		result.push(matrix[pos.x as usize][pos.y as usize]);
	}
}

#[rustfmt::skip]
fn main() {
	assert_eq!(
		spiral([
			[1, 2, 3],
			[4, 5, 6],
			[7, 8, 9],
		]),
		[1, 2, 3, 6, 9, 8, 7, 4, 5]
	);
	assert_eq!(
		spiral([
			[ 1,  2,  3,  4],
			[ 5,  6,  7,  8],
			[ 9, 10, 11, 12],
			[13, 14, 15, 16],
		]),
		[1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
	);
	assert_eq!(
		spiral([
			[ 1,  2,  3,  4,  5],
			[ 6,  7,  8,  9, 10],
			[11, 12, 13, 14, 15],
			[16, 17, 18, 19, 20],
		]),
		[1, 2, 3, 4, 5, 10, 15, 20, 19, 18, 17, 16, 11, 6, 7, 8, 9, 14, 13, 12]
	);
}
