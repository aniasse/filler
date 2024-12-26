use std::{io, time::Duration};
use crate::{solver::valid_position};
#[derive(Debug,Clone)]
pub struct Player{
    pub id :(char,char),
    pub anfield       : Vec<Vec<char>>,
    pub piece       : Vec<Vec<char>>
}
impl Player{
    pub fn new()->Self{
        Self{
            id : (' ',' '),
            anfield :Vec::new(),
            piece :Vec::new(),
        }
    }
    pub fn add_id(&mut self){
        if self.id.1==' ' {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("error lors de la lecture");
            self.id= if  buffer.contains("p1"){
                ('a','@')
            }else{
                ('s','$')
            }
        }
    }
    pub fn add_anfield(&mut self){
        let mut buffer = String::new();
        if  io::stdin().read_line(&mut buffer).is_err() || buffer.is_empty() {
            std::thread::sleep(Duration::from_secs(10));
        }
        let row: u32= buffer.replace(":", "").split_whitespace().collect::<Vec<&str>>()[2].parse().unwrap();
        let _=io::stdin().read_line(&mut buffer);
        self.anfield=[].to_vec();
        for _ in 0..row{
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("error lors de la lecture");
            let list_char= line.split_whitespace().collect::<Vec<&str>>()[1].replace("\n", "").chars().collect::<Vec<char>>();
            self.anfield.push(list_char);
        }
    }
    pub fn add_piece(&mut self){
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("error lors de la lecture");
        let row: u32= buffer.replace(":", "").split_whitespace().collect::<Vec<&str>>()[2].parse().unwrap();
        self.piece=[].to_vec();
        for _ in 0..row{
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("error lors de la lecture");
            let list_piece= line.replace("\n", "").chars().collect::<Vec<char>>();
            self.piece.push(list_piece);
        }
    }
    pub fn solve(&self){
        let coord= valid_position(self);
        println!("{} {}",coord.0,coord.1);
    }
    
}