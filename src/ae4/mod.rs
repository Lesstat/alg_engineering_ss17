use std::str::FromStr;
use std::path::Path;
use std::convert::From;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::time::{Instant, Duration};

use porter_stemmer::stem;
use heapsize::HeapSizeOf;

type InvertedIndex<'a> = HashMap<&'a str, BTreeSet<usize>>;

#[derive(Debug,PartialEq,Eq, HeapSizeOf)]
pub struct Movie {
    pub title: String,
    pub desc: String,
}

impl FromStr for Movie {
    type Err = StrError;
    fn from_str(s: &str) -> Result<Movie, Self::Err> {
        let mut split = s.split("\t");
        let title = split.next().ok_or(StrError { msg: "No title found" })?;
        let desc_old = split
            .next()
            .ok_or(StrError { msg: "No description found" })?
            .to_lowercase();
        let mut desc = String::new();
        for word in desc_old.split(" ") {
            desc.push_str(stem(word).as_str());
            desc.push(' ');
        }

        Ok(Movie {
               title: title.to_owned(),
               desc: desc.to_owned(),
           })

    }
}
pub fn load_movies<P: AsRef<Path>>(file: P) -> Result<Vec<Movie>, StrError> {
    use std::fs::File;
    use std::io::Read;

    let start = Instant::now();
    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let movies: Vec<Movie> = buf.lines()
        .map(|line| {
                 let res = FromStr::from_str(line);
                 res.expect("Line is not valid")
             })
        .collect();
    println!("loading file time: {:?}",
             Instant::now().duration_since(start));
    println!("Movies take {} MB",
             movies.heap_size_of_children() / 1048576);



    Ok(movies)
}

pub fn build_inverted_index(movies: &Vec<Movie>) -> InvertedIndex {
    let start = Instant::now();
    let mut index = HashMap::new();
    for (i, movie) in movies.iter().enumerate() {
        for word in movie.desc.split(" ") {
            index.entry(word).or_insert_with(BTreeSet::new).insert(i);
        }
    }

    println!("index construction took: {:?}",
             Instant::now().duration_since(start));


    index
}

pub fn query_index(index: &InvertedIndex, movies: &Vec<Movie>, query: &String) -> Duration {
    let start = Instant::now();
    let query = query.to_lowercase();
    let mut sets = Vec::new();
    for word in query.trim().split(" ") {
        let stemmed = stem(word);
        let set = match index.get(stemmed.as_str()) {
            Some(set) => set,
            None => continue,
        };
        sets.push(set);
    }

    if sets.is_empty() {
        println!("no results");
        return Instant::now().duration_since(start);
    }
    let mut result = sets[0].clone();
    if sets.len() > 1 {
        for set in &sets[1..] {
            let new_res = result.intersection(set).map(|i| *i).collect();
            result = new_res;
        }
    }
    let finish = Instant::now();

    let count = result.len();
    for i in result {
        println!("{}: {}", i, movies[i].title);
    }
    println!("{} results", count);

    println!("");
    finish.duration_since(start)

}

pub fn naive_query(movies: &Vec<Movie>, query: &String) -> Duration {
    let start = Instant::now();
    let mut result = BTreeSet::new();
    let mut query = stem(query.to_lowercase().trim());
    query.insert(0, ' ');
    query.push(' ');

    for (i, movie) in movies.iter().enumerate() {
        if movie.desc.contains(query.as_str()) {
            result.insert(i);
        }
    }
    let finish = Instant::now();

    let count = result.len();
    for i in result {
        println!("{}: {}", i, movies[i].title)
    }
    println!("{} results", count);

    println!("");
    finish.duration_since(start)
}

#[derive(Debug,PartialEq)]
pub struct StrError {
    msg: &'static str,
}

impl From<::std::io::Error> for StrError {
    fn from(_: ::std::io::Error) -> Self {
        StrError { msg: "io error" }
    }
}



#[cfg(test)]
mod test {

    #[test]
    fn movie_from_str() {
        assert_eq!(Ok(super::Movie {
                          title: "test title".to_owned(),
                          desc: "some movi titl ".to_owned(),
                      }),
                   ::std::str::FromStr::from_str("test title\tSome movie title"));

    }

}
