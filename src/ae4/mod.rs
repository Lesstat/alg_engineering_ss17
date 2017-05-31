use std::str::FromStr;
use std::path::Path;
use std::convert::From;
use std::collections::HashMap;
use std::collections::BTreeSet;

#[derive(Debug,PartialEq,Eq)]
pub struct Movie {
    pub title: String,
    pub desc: String,
}

impl FromStr for Movie {
    type Err = StrError;
    fn from_str(s: &str) -> Result<Movie, Self::Err> {
        let mut split = s.split("\t");
        let title = split.next().ok_or(StrError { msg: "No title found" })?;
        let desc = split
            .next()
            .ok_or(StrError { msg: "No description found" })?;

        Ok(Movie {
               title: title.to_owned(),
               desc: desc.to_owned(),
           })

    }
}
pub fn load_movies<P: AsRef<Path>>(file: P) -> Result<Vec<Movie>, StrError> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let movies = buf.lines()
        .map(|line| {
                 let res = FromStr::from_str(line);
                 res.expect("Line is not valid")
             })
        .collect();
    Ok(movies)
}

pub fn build_inverted_index(movies: &Vec<Movie>) -> HashMap<&str, BTreeSet<usize>> {
    let mut index = HashMap::new();
    for (i, movie) in movies.iter().enumerate() {
        for word in movie.desc.split(" ") {
            index.entry(word).or_insert_with(BTreeSet::new).insert(i);
        }
    }

    index
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
                          desc: "Some movie title".to_owned(),
                      }),
                   ::std::str::FromStr::from_str("test title\tSome movie title"));

    }

}