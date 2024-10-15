/*
    src/modules/composites/dict.rs
    aEon@khaa.pk
 */

// Properties
pub trait DictBody {

    fn find(&self, key: &str) -> Vec<String>;
    fn keys(&self) -> Vec<String>;
    fn len(&self) -> usize;    
    fn new() -> Self;
    fn update(&mut self, key: String, value: String) -> usize;
    fn values(&self) -> Vec<String>; 
}

// Methods
pub struct Dict {

    pub dict: Vec<Vec<String>>,
}

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

    fn new() -> Self {

        // return            
        Dict {dict: Vec::new()}
    }
        
    fn update(&mut self, key: String, value: String) -> usize {

        let mut vec = Vec::<String>::new();

        vec.push(key);
        vec.push(value);

        self.dict.push(vec);

        // return
        self.dict.len()
    }

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