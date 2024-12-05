/// Finds the positions, if any, of the starting character of
/// any matches in any direction.
/// `word_search` is expected to be a 2d array with all rows being the same length.
/// The resulting vec may have duplicates depending on if the starting character there
/// has valid words in multiple directions.
pub fn find_word_in_word_search(word: &str, word_search: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
	let mut matches = vec![];

	// edge cases
	if word.len() == 0 || word_search.len() == 0 || word_search[0].len() == 0 {
		return vec![];
	}

	let word_chars = word.chars().collect::<Vec<char>>();
	let word_len = word_chars.len() as isize;
	let word_search_width = word_search[0].len() as isize;
	let word_search_height = word_search.len() as isize;

	// loop through every coord in word_search
	for y in 0..word_search_height {
		for x in 0..word_search_width {
			// if character isnt start of word, continue
			if word_search[y as usize][x as usize] != word_chars[0] {
				continue;
			}

			// loop through every direction that a word can go in
			let deltas = [
				(-1, -1),
				(-1, 0),
				(-1, 1),
				(0, -1),
				(0, 1),
				(1, -1),
				(1, 0),
				(1, 1),
			];
			'deltas_loop: for (dx, dy) in deltas {
				// check if direction would not end up oob
				let end_x = x + dx * (word_len - 1);
				let end_y = y + dy * (word_len - 1);
				if end_x >= word_search_width
					|| end_y >= word_search_height
					|| end_x < 0 || end_y < 0
				{
					continue;
				}

				// loop through every character in the direction
				// and see if any of them don't match up with the word
				for i in 0..word_len {
					let xprime = x + dx * i;
					let yprime = y + dy * i;
					if word_search[yprime as usize][xprime as usize] != word_chars[i as usize] {
						continue 'deltas_loop;
					}
				}

				// word found! add to matches list
				matches.push((x as usize, y as usize));
			}
		}
	}

	return matches;
}
