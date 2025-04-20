/*
    src/modules/composites/dict.rs
    aEon@khaa.pk
 */

// Metod declarations
pub trait DictBody {

    fn find(&self, key: &str) -> Vec<String>;
    //fn find_u8(&self, key: &str) -> Vec<u8>;
    fn find_u8(&self, key: &str) -> Option<&Vec<u8>>;
    fn keys(&self) -> Vec<String>;
    fn len(&self) -> usize;
    fn len_u8(&self) -> usize;
    fn new() -> Self;
    fn update(&mut self, key: String, value: String) -> usize;
    fn update_u8(&mut self, key: String, value: Vec<u8>) -> usize;
    fn values(&self) -> Vec<String>;
}

// Properties
pub struct Dict {

    pub dict: Vec<Vec<String>>,
    //pub dict_u8: Vec<Vec<u8>>,
    pub dict_u8: Vec<(String, Vec<u8>)>
}

// Implementations
impl DictBody for Dict {

    fn find(&self, key: &str) -> Vec<String> {

        for pair in &self.dict {

            if pair[0].to_string() == key.to_string() {

                //println!("{}, {}", pair[0], pair[1]);
                
                return pair.to_vec();
            }
        }

        // return
        Vec::new()
    }

    fn find_u8(&self, key: &str) -> Option<&Vec<u8>> {

        for (k, v) in &self.dict_u8 {

            if k == key {

                return Some(v);
            }
        }

        None
    }

    /*fn find_u8(&self, key: &str) -> Vec<u8> {

        for pair in &self.dict_u8 {

            if pair[0].to_string() == key.to_string() {

                return pair.to_vec();
            }
        }

        // return
        Vec::new()
    }*/

    fn keys(&self) -> Vec<String> {

        let mut ret = Vec::<String>::new();

        for vec in &self.dict {

            if vec[0].len() > 0 {
            
                ret.push(vec[0].to_string());
            }
        }

        // return
        ret
    }
    
    fn len(&self) -> usize {

        // return
        self.dict.len()
    }

    fn len_u8(&self) -> usize {

        // return
        self.dict_u8.len()
    }

    fn new() -> Self {

        // return            
        Dict {dict: Vec::new(), dict_u8: Vec::new()}
    }
        
    fn update(&mut self, key: String, value: String) -> usize {

        let mut vec = Vec::<String>::new();

        vec.push(key);
        vec.push(value);

        self.dict.push(vec);

        // return
        self.dict.len()
    }

    fn update_u8(&mut self, key: String, value: Vec<u8>) -> usize {

        self.dict_u8.push((key, value));
        self.dict_u8.len()
    }

    /*
    pub fn update_u8(&mut self, key: String, value: Vec<u8>) -> usize {

        let mut vec = Vec::<u8>::new();

        //vec.push(key.as_bytes());
        vec.extend_from_slice(key.as_bytes());
        //vec.push(value);
        vec.extend_from_slice(value.as_slice());

        self.dict_u8.push(vec);

        // return
        self.dict_u8.len()
    } */

    fn values(&self) -> Vec<String> {

        let mut ret = Vec::<String>::new();

        for vec in &self.dict {

            if vec[1].len() > 0 {

                ret.push(vec[1].to_string());
            }
        }

        // return
        ret
    }
}