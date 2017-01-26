//use std::io::Write;
use std::io::prelude::*;

mod futils {
  use std;
  use std::io::prelude::*;
  use std::fs::File;
  use std::io::{BufReader, BufWriter};
  use std::fs::OpenOptions;
  pub struct OFile {
    pub filename:String,
    pub buffered:BufWriter<File>  
  }
  pub struct IFile {
    pub filename:String,
    pub buffered:BufReader<File>
  }
  impl IFile{
    fn new(f:&str) -> IFile {
      let filename = f.to_string();
      let infile = File::open(f).unwrap();
      let buffered = BufReader::new(infile);
      IFile { buffered:buffered, filename:filename}
    }
  }
  impl OFile{
    pub fn new(f:&str) -> OFile {
      let filename = f.to_string();
      let outfile = File::create(f).unwrap();
      let buffered = BufWriter::new(outfile);
      OFile { buffered:buffered, filename:filename}
    }
    pub fn write<'b, T>(&mut self, m:&'b T) -> &mut OFile where T: std::fmt::Display {
      write!(self.buffered, "{}", m);
      self
    }
    pub fn writeln<'b, T>(&mut self, m:&'b T) -> &mut OFile where T: std::fmt::Display {
      writeln!(self.buffered, "{}\n", m);
      self
    }
    pub fn write_debug<'b, T>(&mut self, m:&'b T) -> &mut OFile where T: std::fmt::Debug {
      write!(self.buffered, "{:?}", m);
      self
    }
    pub fn write_str<'b>(&mut self, m:&'b str) -> &mut OFile {
      self.buffered.write(&m.as_bytes()[..]);
      self
    }
    pub fn write_bytes<'b>(&mut self, m:&'b[u8]) -> &mut OFile {
      self.buffered.write(m);
      self
    }
    pub fn newline(&mut self) -> &mut OFile {
      self.buffered.write("\n".as_bytes());
      self
    }
    pub fn flush(&mut self) -> &mut OFile {
      self.buffered.flush();
      self
    }
  }
  pub struct FUtils;

  impl FUtils{
    pub fn infile(f:&str) -> IFile { IFile::new(f) }
    pub fn outfile(f:&str) -> OFile { OFile::new(f) }

    pub fn dumpb(buffer:&mut BufWriter<File>, s:&str){
      let bytes = &s.as_bytes()[..];
      buffer.write(bytes);
    }
    pub fn dumpf(filename:&str, s:&str){
      let mut outfile = File::create(filename).unwrap();
      let mut buffered = BufWriter::new(outfile);
      FUtils::dumpb(&mut buffered, s);
      buffered.flush();
    }
    pub fn appendf(filename:&str, s:&str){
      let outfile = OpenOptions::new().append(true).open(filename).unwrap();
      let mut buffered = BufWriter::new(outfile);
      FUtils::dumpb(&mut buffered, s);
      buffered.flush();
    }
    pub fn err(m:&str){
      writeln!(&mut std::io::stderr(), "{}", m);
    }
    pub fn lines(f:&str) -> Vec<String> {
      let infile = File::open(f).unwrap();
      let buffered = BufReader::new(infile);
      buffered.lines().map(|l|{l.unwrap()}).collect::<Vec<String>>()
    }
    pub fn read(f:&str) -> ! {
      panic!("unimplemented");
    }
  }
}



mod richstr{
  use std;
  use std::time::{Duration, SystemTime};
  use std::fmt::Write;
  use futils::{FUtils, OFile};
  const VERBOSE:bool = true;
  pub struct RichStr {
    pub data:String,
    log:bool,
    buffered:Option<OFile>
  }
  impl<'a> RichStr {
    pub fn logged(filename:Option<&str>) -> RichStr {
      RichStr { log: true, data : String::new(), buffered: filename.map(|f|{ FUtils::outfile(f)} ) }
    }
    pub fn new() -> RichStr {
      RichStr { log: false, data : String::new(), buffered: None }
    }
    pub fn write<'b, T>(&mut self, m:&'b T) -> &mut RichStr where T: std::fmt::Display {
      write!(self.data, "{}", m);
      self
    }
    pub fn writeln<'b, T>(&mut self, m:&'b T) -> &mut RichStr where T: std::fmt::Display {
      write!(self.data, "{}\n", m);
      self
    }
    pub fn newline(&mut self) -> &mut RichStr{
      self.data.push('\n');
      self
    }
    pub fn tab(&mut self) -> &mut RichStr{
      self.data.push('\t');
      self
    }
    pub fn write_debug<'b, T>(&mut self, m:&'b T) -> &mut RichStr where T: std::fmt::Debug {
      write!(self.data, "{:?}", m);
      self
    }
    pub fn write_str<'b>(&mut self, m:&'b str) -> &mut RichStr {
      self.data.push_str(&m);
      self
    }  
    pub fn timestamp(&mut self) -> &mut RichStr {
      self.write_debug(&SystemTime::now());
      self
    }
    pub fn clear(&mut self) -> &mut RichStr {
      self.data.clear();
      self
    }
    pub fn to_stderr(&mut self) -> &mut RichStr {
      FUtils::err(&self.data);
      self
    }
    pub fn flushlog(&mut self) -> &mut RichStr {
      match self.buffered {
        Some(ref mut f) => {
          FUtils::dumpb(&mut f.buffered, &self.data);
          if VERBOSE { FUtils::err("OK!"); }
          f.flush();
        } , 
        _ => {
          self.to_stderr();
        }
      }
      self.clear();
      self
    }
    pub fn dumpf(&mut self, filename:&str) -> &mut RichStr {
      FUtils::dumpf(filename, &self.data);
      self
    }
    
    pub fn appendf(&mut self, filename:&str) -> &mut RichStr {
      FUtils::appendf(filename, &self.data);
      self
    }
  }
  impl Drop for RichStr {
    fn drop(&mut self) {
      if self.log {
        self.flushlog();
      }
    }
  }
}

mod config {
  use std::collections::HashMap;
  use std::collections::HashSet;
  use std::cell::{RefCell, RefMut};
  use std::borrow::Borrow;
  use std::marker::PhantomData;
  use std::env;
  use std::ops::{Index, Deref};
  use std::io::Write;
  use richstr::RichStr;

  pub struct Config {
    pub flags:HashSet<String>,
    pub values:Vec<String>,
    pub keys:HashMap<String, String>,
    pub sys:Vec<String>,
    buffer:RefCell<RichStr>,
  }
  
  impl<'b> Index<&'b str> for Config{
    type Output = String;
    fn index(&self, k:&'b str) ->  &String {
      &self.keys[k]
    }
  }

  impl Deref for Config {
      type Target = HashMap<String, String>;
      fn deref(&self) -> &Self::Target { &self.keys }
  }


  impl Config{
    fn new() -> Config {
      Config { 
        flags : HashSet::new(),
        values : Vec::new(),
        keys : HashMap::new(),
        sys : Vec::new(),
        buffer : RefCell::new(RichStr::logged(None)),
      }
    }
    pub fn log(&self, m:&str){ self.logstr().write_str(m); }
    pub fn has(&self, key:&str) -> bool { self.flags.contains(key) || self.keys.contains_key(key) }
    pub fn logstr(&self) -> RefMut<RichStr> { self.buffer.borrow_mut() }
    pub fn get<'a, 'b>(&'a self, key:&'b str) -> Option<&'a String> { self.keys.get(key) }

    pub fn sys() -> Config {
      let args:Vec<String> = env::args().skip(1).collect();
      Config::parsed(&args)
    }
    pub fn parsed(args_v:&Vec<String>) -> Config {
      let mut args  = Config::new();
      args.logstr().timestamp();
      let mut words:Vec<&str> = Vec::new();
      for arg in args_v.iter() {
        args.sys.push(arg.to_string());
        if arg.chars().next().unwrap() == '-' {
          words.clear();
          words = arg.split(':').map(|s| s).collect();
          let parsed_ok:bool = match words.len() {
            1 => args.flags.insert(arg.to_string()) ,
            2 => args.keys.insert(words[0].to_string(), words[1].to_string()).is_some(),
            _ => {
              args.logstr().newline().write_str("#malformed:").write_str(arg).newline();
              false
            }
          };
        } else {
          args.values.push(arg.to_string());
        }
      }
      args
    }
  }
}

use richstr::RichStr;
use config::Config;

fn main(){
  let s = 1 as u32;
  let mut stream = RichStr::logged(Some("job.log"));
  stream.write_str("hello world... ").write(&s).write_debug(&s).newline().timestamp().write_str("!!!!!!").write(&s);
  stream.writeln(&s);
  stream.dumpf("test.meh");
  stream.appendf("test.meh");
  let c = Config::sys();
  c.log("using config.. ");
  let ref meh = c.get("-hello");
  c.logstr().write_debug(meh);
  c.logstr().newline().write_debug(meh);
}
