use std::fs;
use std::io::Error;

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde()]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

pub fn find_person(path: &str, name_to_filter: &str) -> Result<Option<Person>, Error> {
    persons_from(path)
        .map(|persons| filter_name(name_to_filter, persons))
}

fn filter_name(name_to_filter: &str, vec: Vec<Person>) -> Option<Person> {
    vec
        .into_iter()
        .find(|person| person.first_name == name_to_filter)
}

fn persons_from(path: &str) -> Result<Vec<Person>, Error> {
    fs::read_to_string(path)
        .map(|content| to_persons(&content))

}



fn to_persons(content: &String) -> Vec<Person> {
    serde_json::from_str(&content).expect("error converting json in vec of persons")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn persons_from_file() {
        let persons_result = persons_from("test_data/persons.json");

        assert_eq!(persons_result.is_ok(), true);

        let persons = &persons_result.unwrap();

        assert_eq!((&persons[0]).first_name, String::from("Mario"));
        assert_eq!((&persons[1]).first_name, String::from("Luigi"));
    }


    #[test]
    fn filter_person_from_persons() {
        let persons_result = find_person("test_data/persons.json", "Mario");

        assert_eq!(persons_result.is_ok(), true);

        let person = persons_result.unwrap().unwrap();

        assert_eq!(person.last_name, String::from("Russo"));
    }


    #[test]
    fn filter_not_existing_person_from_persons() {
        let persons_result = find_person("test_data/persons.json", "notExisting");

        assert_eq!(persons_result.is_ok(), true);
        assert_eq!(persons_result.unwrap().is_none(), true);



    }

    #[test]
    fn filter_not_existing_person_from_persons_file_exception() {
        let persons_result = find_person("test_data/aaapersons.json", "notExisting");

        assert_eq!(persons_result.is_err(), true);
    }


}
