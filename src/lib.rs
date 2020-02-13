mod utils;
mod data;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys::Array;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Copy, Clone,Debug, Eq, PartialEq)]
struct Charcode{
    ch:char,
    count:u8,
}

#[wasm_bindgen]
pub struct Decrypter{
    unsolved:String,
    encrypted:Vec<String>,
    solved:Vec<String>
}

#[wasm_bindgen]
impl Decrypter{

    pub fn get_size(&self) ->usize{
        self.encrypted.len()
    }

    pub fn encrypted_list(&self) -> js_sys::Array {
        let jsarr = Array::new();
        for e in self.encrypted.iter(){
            jsarr.push(&JsValue::from(e));
        }
        jsarr
    }

    pub fn decrypted_list(&self) -> js_sys::Array {
        let jsarr = Array::new();
        for e in self.solved.iter(){
            jsarr.push(&JsValue::from(e));
        }
        jsarr
    }
    
    //Get new Decrypter
    pub fn new()->Decrypter{
        let unsolved = data::get_decode_instructions();
        let solved:Vec<String> = Vec::new();
        let encrypted:Vec<String>=Vec::new();

        Decrypter{
            unsolved,
            encrypted,
            solved
        }
    }

    pub fn decode_and_validate(&mut self){
        for line in self.unsolved.lines(){
            self.encrypted.push(line.to_string());
            let (sector, is_valid) = self.get_valid_checksum_sector(line.to_string());
            if is_valid{
                let decrypted = self.decrypt(line, sector);
                self.solved.push(decrypted);
            }else{
                self.solved.push(String::new());
            }
        }        
    }


    fn get_valid_checksum_sector(&self, line:String) -> (i32,bool){
        let mut char_counts:HashMap<char,Charcode> = HashMap::new();
        let last_dash:usize = line.rfind('-').unwrap();
        let code_only = line[..last_dash].trim_end();
        let sector = &line[last_dash+1..line.rfind('[').unwrap()];
        let checksum = &line[line.find('[').unwrap()+1..line.rfind(']').unwrap()];
    
        for ch in code_only.chars(){
            if ch.is_alphabetic(){
                if char_counts.contains_key(&ch){
                    char_counts.get_mut(&ch).unwrap().count+=1;
                }else{
                    let charcode =Charcode{ch,count:1};
                    char_counts.insert(ch, charcode);
                }
            }
        }
    
        let sector:(i32,bool) = match self.is_real_room(char_counts, checksum){
            true => (sector.parse().unwrap(), true),
            false => (0,false),
        };
        sector    
    }


    fn is_real_room(&self,char_counts:HashMap<char,Charcode>,checksum:&str)->bool{
        let mut cmp_checksum = String::new();
        let mut occurrences:Vec<Charcode> =Vec::new();
    
        for key in char_counts.values(){
            occurrences.push(key.clone());
        }
        occurrences.sort_by(|a,b| {
            if a.count > b.count{
                return std::cmp::Ordering::Less
            }else if a.count < b.count{
                return std::cmp::Ordering::Greater
            }
            a.ch.cmp(&b.ch)
        });
        
        for i in 0..5{
            cmp_checksum.push(occurrences[i].ch);
        }
    
        checksum == cmp_checksum
    }


    fn decrypt(&self, line:&str,sector:i32)->String{
        let mut decrypted = String::new();
        for c in line.chars(){
            if c.is_alphabetic(){
                let ch = self.shift_chars(c, sector);
                decrypted.push(ch);
            }else if c == '-'{
                decrypted.push(' ');
            }
        }
        decrypted
    }


    fn shift_chars(&self, code_only:char, sector:i32)-> char{
        let bcode = code_only.to_string().as_bytes()[0] + (sector % 26) as u8;
        if bcode > 122{
            return (bcode-26) as char;
        }else if bcode < 97{
            return (bcode+26)as char;
        }
        return bcode as char;
    }

}
