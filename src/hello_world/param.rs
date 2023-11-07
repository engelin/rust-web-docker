use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Param {
    pub param: Vec<Vec<char>>,
}
impl Param {
    pub fn new() -> Param {
        Param {
            param: vec![vec![' '; 3]; 3],
        }
    }

    fn is_valid_param_from_string(query_param: String) -> (bool, Option<String>) {
        let len = query_param.len();

        if len == 0 || len != 9 {
            log::debug!("len: {}", len);
            return (false, Some(String::from("Invalid param length!")));
        }

        if !query_param.chars().all(|c| c == 'x' || c == 'o' || c == ' ') {
            log::debug!("Invalid param: {}", query_param);
            return (false, Some(String::from("Invalid param value!")));
        }

        (true, None)
    }

    fn string_to_param_vector(query_param: String) -> Vec<Vec<char>> {
        let mut vec: Vec<Vec<char>> = Vec::new();
        for i in 0..3 {
            let mut row: Vec<char> = Vec::new();
            for j in 0..3 {
                let index = i * 3 + j;
                let c = query_param.chars().nth(index).unwrap();
                row.push(c);
            }
            vec.push(row);
        }
        vec
    }

    pub fn set_param(&mut self, query_param: String) -> (bool, Option<String>) {
        let (is_valid, err) = Param::is_valid_param_from_string(query_param.clone());
        if !is_valid {
            return (false, err);
        }

        self.param = Param::string_to_param_vector(query_param);
        (true, None)
    }
}
