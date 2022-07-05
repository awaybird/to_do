use std::{collections::HashMap, hash::Hash};
use std::io::Read;
use std::str::FromStr;

struct ToDo{
    map: HashMap<String,bool>,
}

impl ToDo {
    fn insert(&mut self,key:String){
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>>{
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    // fn save(self) -> Result<(),std::io::Error>{
    //     let mut content = String::new();
    //     for (k, v ) in self.map{
    //         let record = format!("{}\t{}\n",k,v);
    //         content.push_str(&record)
    //     }
    //     std::fs::write("db.txt", content)
    // }

    // fn new() -> Result<ToDo, std::io::Error> {
    //     let mut f = std::fs::OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .read(true)
    //     .open("db.txt")?;
    //     let mut content = String::new();
    //     f.read_to_string(&mut content)?;
    //     let map: HashMap<String, bool> = content
    //     .lines()
    //     .map(|line| line.splitn(2,'\t').collect::<Vec<&str>>())
    //     .map(|v| (v[0], v[1]))
    //     .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
    //     .collect();
    //     Ok(ToDo{map})
    // }
    fn new() -> Result<ToDo, std::io::Error>{
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.json")?;

        match serde_json::from_reader(f){
            Ok(map) => Ok(ToDo{map}),
            Err(e) if e.is_eof() => Ok(ToDo{
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured:{}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()>{
        match self.map.get_mut(key){
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("please sepcify an action");
    let item = std::env::args().nth(2).expect("please specify an item");
    //println!("{:?},{:?}", action, item);

    let mut todo = ToDo::new().expect("Initalisation of db faild");
    if action == "add"{
        todo.insert(item);
        match todo.save(){
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred:{}", why),
        }
    } else if action == "complete"{
        match todo.complete(&item){
            None => println!("{} is not present in the list", item),
            Some(_) => match todo.save(){
                Ok(_) => println!("tod saved"),
                Err(why) => println!("An error occured: {}", why),
            }
        }
    }
}
