#[link(name="cpptest", kind="static")]
extern {
  pub fn fromCPP(); 
}
#[link(name="rstest", kind="static")]
extern {
  pub fn fromRust();
}

#[test]
fn test(){
  println!("{}", "fromMain");
  unsafe {
    fromCPP();
    fromRust();
  }
  println!("..ok");
}

fn main(){
  test();
}


