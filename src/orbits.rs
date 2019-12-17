use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub enum Object {
    CenterOfMass,
    Object(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::CenterOfMass => write!(f, "COM"),
            Object::Object(n)    => write!(f, "{}", n),
        }
    }
}

impl FromStr for Object {
    type Err = ();

    fn from_str(s: &str) -> Result<Object,Self::Err> {
        if s == "COM" {
            Ok(Object::CenterOfMass)
        } else {
            Ok(Object::Object(s.to_string()))
        }
    }
}

pub struct UniversalOrbitMap {
    orbits: HashMap<Object,Vec<Object>>
}

impl FromStr for UniversalOrbitMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let lines = s.split('\n');
        let mut orbits: HashMap<Object,Vec<Object>> = HashMap::new();

        for nextline in lines {
            if nextline.trim().len() == 0 {
                continue;
            }
            let mut splits = nextline.split(')');
            let obj1str = splits.next().unwrap_or_else(|| panic!("Bad orbit line: {}", nextline));
            let obj2str = splits.next().unwrap_or_else(|| panic!("Bad orbit line: {}", nextline));
            let obj1 = Object::from_str(&obj1str)?;
            let obj2 = Object::from_str(&obj2str)?;
            if let Some(prev) = orbits.get_mut(&obj1) {
                prev.push(obj2);
            } else {
                orbits.insert(obj1, vec![obj2]);
            }
        }

        Ok(UniversalOrbitMap{ orbits })
    }
}

impl UniversalOrbitMap {
    fn orbits(&self, obj1: &Object, obj2: &Object) -> bool {
        match self.orbits.get(obj2) {
            None => false,
            Some(items) => {
                items.contains(obj1)
            }
        }
    }

    fn indirectly_orbits(&self, obj1: &Object, obj2: &Object) -> bool {
        let mut search_stack = vec![obj2];
        let mut history = vec![];

        while let Some(nextobj) = search_stack.pop() {
            if nextobj == obj1 {
                return true;
            }

            if history.contains(nextobj) {
                continue;
            } else {
                history.push(nextobj.clone());
            }

            match self.orbits.get(nextobj) {
                None =>
                    continue,
                Some(newitems) => {
                    for item in newitems.iter() {
                        search_stack.push(item);
                    }
                }
            }
        }

        false
    }

    fn objects(&self) -> Vec<Object> {
        let mut res = vec![];

        for (key, vals) in self.orbits.iter() {
            if !res.contains(key) { res.push(key.clone()); };
            for val in vals.iter() {
                if !res.contains(val) { res.push(val.clone()); };
            }
        }

        res
    }

    pub fn num_orbits(&self) -> usize {
        let mut result = 0;

        for obj1 in self.objects().iter() {
            for obj2 in self.objects().iter() {
                if (obj1 != obj2) && self.indirectly_orbits(obj1, obj2) {
                    result += 1;
                }
            }
        }

        result
    }
}

#[test]
fn examples() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let map = UniversalOrbitMap::from_str(&input).unwrap();
    assert!(map.orbits(&Object::Object("B".to_string()), &Object::CenterOfMass));
    assert!(map.indirectly_orbits(&Object::Object("B".to_string()), &Object::CenterOfMass));
    assert!(map.indirectly_orbits(&Object::Object("E".to_string()),&Object::CenterOfMass));
    assert_eq!(map.num_orbits(), 42);
}
