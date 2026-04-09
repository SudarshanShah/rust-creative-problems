/*
Problem 2: The Word Frequency DJ 🎵

  Given a multi-line string of song lyrics (hardcode your favourite song lines or use lorem ipsum),
  find the top 5 most repeated words, ignoring punctuation and case. You must use iterator chains —
  no manual loops are allowed. Output a sorted list of (word, count) tuples.
  Also write it to a file named top_words.txt.

  The Story
    You're building a backstage tool for a music festival. The DJ wants to know which words dominate the
    vibe of their setlist announcements — basically a word frequency analyzer for song lyrics / announcements.
    Your job: parse the text, count every word, and surface the top 5 most repeated ones. Then save the results
    to a file for the DJ's records.
*/
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, Write};

const SONG: &str = "I love the night, the night loves me.
Dancing in the dark, dark as the night.
Loves is blind, but I can see the stars.
Stars in the night sky, dancing and dreaming.
I dream of love, love of the dance.
The dance goes on and on and on.
Night after night, I dance and dream and love.";

/*
-> Split on whitespace
-> Strip punctuation from each word (., ,, !, ? etc.) — use .trim_matches or similar
-> Convert to lowercase
-> Count occurrences in a HashMap
-> No manual for loops — use iterator chains with closures
*/
fn count_words(text: &str) -> HashMap<String, u32> {
    let result = text
        .split_whitespace()
        .map(|word| {
            word.trim_matches(|c| c == '.' || c == ',' || c == '!' || c == '?')
                .to_lowercase()
        })
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });

    result
}

/*
-> Takes a reference to the map (doesn't consume it)
-> Returns top n entries sorted by count descending
-> If two words have the same count, sort alphabetically as a tiebreaker
-> Again, iterator chains only — no manual loops
*/
fn top_n(map: &HashMap<String, u32>, n: usize) -> Vec<(&str, u32)> {
    // let mut result = map.iter().fold(Vec::new(), |mut acc, e| {
    //     acc.push((&e.0[0..], *e.1));
    //     acc
    // });

    let mut result: Vec<(&str, u32)> = map.iter().map(|(k, v)| (k.as_str(), *v)).collect();

    result.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));

    result.into_iter().take(n).collect()
}

/*
-> Writes to the given file path
-> Each line formatted as: `the: 8`
-> Returns the IO error if file writing fails
*/
fn write_results(results: &Vec<(&str, u32)>, path: &str) -> Result<(), Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    results.iter().try_for_each(|e| {
        writeln!(writer, "{}: {}", e.0, e.1)
    })?;
    writer.into_inner().expect("Flush failed");

    Ok(())
}

/*
4. Wire it all up in `main`:**
- Call `count_words` → `top_n` with n=5 → print to console → `write_results` to `top_words.txt`
- Handle the file write result with `match` — print success or error message

---

## Expected Output (console)
```
🎵 Top 5 Words in the DJ's Set:
1. the  — 8 times
2. night — 6 times
3. and  — 5 times
4. i    — 4 times
5. love — 4 times

✅ Results saved to top_words.txt
```

*(exact counts depend on your punctuation stripping — close to these is fine)*

---

## `top_words.txt` should contain:
```
the: 8
night: 6
and: 5
i: 4
love: 4
*/

fn main() {

    let word_map = count_words(SONG);

    let trending_words = top_n(&word_map, 5);
    println!("🎵 Top 5 Words in the DJ's Set:");

    trending_words
        .iter()
        .enumerate()
        .for_each(|(i, e)| println!("{}. {} - {} time", i+1, e.0, e.1));

    match write_results(&trending_words, "top_words.txt") {
        Ok(()) => println!("✅ Results saved to top_words.txt"),
        Err(e) => println!("Error : {}", e),
    }
}
