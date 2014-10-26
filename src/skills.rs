use mul_reader::MulReader;
use std::io::IoResult;

pub struct Skill {
    clickable: bool,
    name: String
}

impl Skill {
    pub fn new(clickable: bool, name: String) -> Skill {
        Skill {
            clickable: clickable,
            name: name
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![if self.clickable {1} else {0}];
        vec.push_all(self.name.to_c_str().as_bytes());
        vec
    }
}

pub struct Skills {
    skills: Vec<Skill>
}

impl Skills {

    pub fn new(index_path: &Path, mul_path: &Path) -> IoResult<Skills> {
        let maybe_reader = MulReader::new(index_path, mul_path);
        match maybe_reader {
            Ok(mut reader) => {

                //Unpack the lot
                let mut result = vec![];
                let mut id = 0;
        
                loop {
                    match reader.read(id) {
                        Ok(record) => {
                            let slice = record.data.slice(1, record.data.len() - 1);
                            result.push(Skill::new(record.data[0] == 1, slice.to_vec().into_ascii().into_string()))
                        },
                        _ => {
                            break;
                        }
                    }
                    id += 1;
                }

                Ok(Skills {
                    skills: result
                })
            },
            Err(io_error) => Err(io_error)
        }
    }
}
