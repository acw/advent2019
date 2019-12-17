use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub enum Object {
    CenterOfMass,
    Object(String),
}

impl Object {
    pub fn new(s: &str) -> Object {
        if s == "COM" {
            Object::CenterOfMass
        } else {
            Object::Object(s.to_string())
        }
    }
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
        Ok(Object::new(s))
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
            let obj1str = splits.next()
                                .unwrap_or_else(|| panic!("Bad orbit line: {}", nextline))
                                .trim();
            let obj2str = splits.next()
                                .unwrap_or_else(|| panic!("Bad orbit line: {}", nextline))
                                .trim();
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
        let mut search_stack = vec![(Object::CenterOfMass, 0)];
        let mut total = 0;

        while let Some((nextobj, len)) = search_stack.pop() {
            total += len;

            match self.orbits.get(&nextobj) {
                None => { }
                Some(objs) => {
                    for obj in objs {
                        search_stack.push((obj.clone(), len + 1));
                }
            }
        }
        }

        total
    }

    pub fn show(&self) {
        for (key, values) in self.orbits.iter() {
            print!("{} => ", key);
            for value in values.iter()  {
                print!("{} ", value);
            }
            println!("");
        }
    }

    fn path_from_origin(&self, obj: &Object) -> Option<Vec<Object>> {
        let mut search_stack = vec![(Object::CenterOfMass, vec![Object::CenterOfMass])];
        let mut total = 0;

        while let Some((nextobj, mut path)) = search_stack.pop() {
            if &nextobj == obj {
                path.push(nextobj);
                return Some(path);
            }

            match self.orbits.get(&nextobj) {
                None => { }
                Some(objs) => {
                    path.push(nextobj);
                    for obj in objs {
                        search_stack.push((obj.clone(), path.clone()));
                    }
                }
            }
        }

        panic!("Can't reach object from origin: {}", obj);
    }

    pub fn find_path(&self, obj1: &Object, obj2: &Object) -> Option<Vec<Object>> {
        let obj1path = self.path_from_origin(obj1)?; 
        let obj2path = self.path_from_origin(obj2)?; 

        println!("path1: {:?}", obj1path);
        println!("path2: {:?}", obj2path);

        let joinpoint = find_join_point(&obj1path, &obj2path)?;

        let path1stub = obj1path.iter().skip_while(|x| x != &&joinpoint);
        let path2stub = obj2path.iter().skip_while(|x| x != &&joinpoint);

        let mut result = vec![];

        for x in path1stub { result.push(x.clone()); }
        result.reverse();
        for x in path2stub.skip(1) { result.push(x.clone()); }
        println!("result: {:?}", result);

        Some(result)
    }
}

fn find_join_point<T: Clone + PartialEq>(list1: &Vec<T>, list2: &Vec<T>) -> Option<T> {
    for possible in list1.iter().rev() {
        if list2.contains(possible) {
            return Some(possible.clone());
        }
    }
    None
}

#[test]
fn examples() {
    let input1 = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let map = UniversalOrbitMap::from_str(&input1).unwrap();
    assert!(map.orbits(&Object::new("B"), &Object::CenterOfMass));
    assert!(map.indirectly_orbits(&Object::new("B"), &Object::CenterOfMass));
    assert!(map.indirectly_orbits(&Object::new("E"),&Object::CenterOfMass));
    assert_eq!(map.num_orbits(), 42);
    let day6_contents = std::fs::read("inputs/day6").unwrap();
    let day6_str = std::str::from_utf8(&day6_contents).unwrap();
    let day6map = UniversalOrbitMap::from_str(&day6_str).unwrap();
    assert_eq!(day6map.num_orbits(), 204521);
    let input2 = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    let map2 = UniversalOrbitMap::from_str(input2).unwrap();
    assert_eq!(map2.find_path(&Object::new("YOU"), &Object::new("SAN")).unwrap().len(), 7);
}
