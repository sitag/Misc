// intersect genomic coordinates

use std::error::Error;

#[derive(Debug, Eq, PartialOrd, PartialEq, Ord)]
struct Coord<C>(C,C);

#[derive(Debug)]
struct Intersect<C> {
    a:Option<Coord<C>>,
    cmn:Option<Coord<C>>,
    b:Option<Coord<C>>
}

type C=i32;
impl Intersect<C> {
    fn span(&self) -> C  {
        Coord::span(&self.a) + Coord::span(&self.b) + Coord::span(&self.cmn)
    }
    fn add_characterstic<T>(&self, v:&mut Vec<T>, e:&T, f:fn(&T, &T) -> T, e_cmn:&T, f_cmn:fn(&T, &T) -> T) -> Result<(), Box<Error>> {
        if let &Some(ref z) = &self.a {
           z.add_characterstic(v, e, f)?;
        }
        if let &Some(ref z) = &self.b {
           z.add_characterstic(v, e, f)?;
        }
        if let &Some(ref z) = &self.cmn {
           z.add_characterstic(v, e_cmn, f_cmn)?;
        }
        Ok(())
    }
    pub fn intersect_coords(&mut self, a:&Coord<C>, b:&Coord<C>) -> Result<(), Box<Error>> {
        if  a < b {
            Coord::intersect(self,  a.0, a.1, b.0, b.1)?; 
        } else {
            Coord::intersect(self, b.0, b.1, a.0, a.1)?;
            self.swap_ab();
        };
        let checked = Coord::check_intersect(a, b, self, true);
        if !checked.is_ok() { self.take(); }
        checked
    }
    
    pub fn swap_ab(&mut self){
        let b = self.b.take();
        self.b = self.a.take();
        self.a = b;
    }
    
    pub fn take(&mut self) -> Intersect<C> {
        Intersect {a: self.a.take(), b :self.b.take(), cmn: self.cmn.take()}
    }
}

impl Coord<C> {
    pub fn add_characterstic<T>(&self, v:&mut Vec<T>, e:&T, f:fn(&T, &T) -> T) -> Result<(), Box<Error>>  {
        if !((self.1 as usize) < v.len()) {
            Err(format!("__DIM_MISMATCH__").into())
        } else {
            for i in self.0 .. self.1 {
                *v.get_mut(i as usize ).unwrap() = f(v.get(i as usize).unwrap(), e);
            }
            Ok(())
        }
    }

    fn span(c:&Option<Coord<C>>) -> C  {
        match c {
            &Some(ref coord) => coord.1 - coord.0, _ => 0
        }
    }


    fn check_intersect(a:&Coord<C>, b:&Coord<C>, intersected:&Intersect<C>, slice_check:bool) -> Result<(), Box<Error>> {
        // check length
        if intersected.span() + Coord::span(&intersected.cmn) != (a.1 - a.0 + b.1 - b.0) {
            return Err(format!("LENGTH_MISMATCH: {:?} {:?} {:?}", intersected, a, b ).into());
        }

        if !slice_check {
            return Ok(());
        }

        use std::cmp::max;
        let tol = 1e-6 as f64;
        let ref one = 1 as f64;
        fn inc(x:&f64, y:&f64) -> f64 { x+y };
        fn inc_twice(x:&f64, y:&f64) -> f64 { x+y+y };
        let n_data_slice = max(max(a.0, b.0), max(a.1, b.1)) + 1;
        let ref mut data_slice = vec![0 as f64;n_data_slice as usize];
        let ref mut data_slice_check = vec![0 as f64;n_data_slice as usize];
        
        // build characterstic functions both ways and check they are equal

        intersected.add_characterstic(data_slice, one, inc, one, inc_twice)?;
        a.add_characterstic(data_slice_check, one, inc)?;
        b.add_characterstic(data_slice_check, one, inc)?;

        let mut integral = 0 as i32;
        for i in 0 .. data_slice.len() {
            integral += data_slice[i] as i32;
            if (data_slice[i] - data_slice_check[i]).abs() > tol {
                return Err(format!("SLICE_MISMATCH: {:?} {:?} {:?} {:?} {:?} {:?}", intersected, a, b, i, data_slice[i], data_slice_check[i] ).into());
            } 
        }
        if integral != intersected.span() + Coord::span(&intersected.cmn) {
            return Err(format!("INTEGRAL_MISMATCH: {:?} {:?} {:?} {:?}", intersected, a, b, integral ).into());
        }
        Ok(()) 
    }

    fn from_start_end(start:C, end:C) -> Option<Coord<C>> {
        if start == end { None } else { Some(Coord(start, end)) }
    }

    
    fn intersect(intersected:&mut Intersect<C>, s1:C, e1:C, s2:C, e2:C) -> Result<(), Box<Error>> {
        if !(  s1 < e1 && s2 < e2 ) {
            intersected.take();
            return Err(format!("INVALID_COORDS? {:?} {:?}", (s1, e1), (s2, e2)).into());
        }
        
        if (s2 < s1 ) || (s1 == s2 && e2 < e1) {
            intersected.take();
            return Err(format!("NOT_SORTED? {:?} {:?}", (s1, e1), (s2, e2)).into());
        };
        
        // at this point (s1, e1) <= (s2, e2)
        if s1 == s2 && e1 <= e2 { // case: [1, 10] intersect [1, 10] 
            intersected.a = None;
            intersected.cmn = Coord::from_start_end(s1, e1);
            intersected.b = Coord::from_start_end(e1, e2);
            Ok(())
        } else if e1 == e2 { // case: [1, 10] intersect [10+, 20] 
            intersected.a = Coord::from_start_end(s1, s2);
            intersected.cmn = Coord::from_start_end(s2, e2);
            intersected.b = None;
            Ok(())
        } else if e1 <= s2 { // case: [1, 10] intersect [10+, 20] 
            intersected.a = Coord::from_start_end(s1, e1);
            intersected.cmn = None;
            intersected.b = Coord::from_start_end(s2, e2);
            Ok(())
        } else if !(e1 > s2 && e1 < e2) {
            // e1 == e2, e1 <= s2 is handled above
            // e1 > e2 is checked with NOT_SORTED error
            intersected.take();
            Err(format!("UNREACHABLE? {:?} {:?}", (s1, e1), (s2, e2)).into())
        } else {
            intersected.a = Coord::from_start_end(s1, s2);
            intersected.cmn = Coord::from_start_end(s2, e1);
            intersected.b = Coord::from_start_end(e1, e2);
            Ok(())
        }
    }
}

fn main(){
    
    let ref a = Coord(1, 10);
    let ref b = Coord(10, 20);
    let ref c = Coord(5, 10);
    let ref d = Coord(5, 10);
    let ref e = Coord(5, 15);
    let ref mut intersected = Intersect { a:None, b:None, cmn:None };
    
    let ok = intersected.intersect_coords(a, b);
    println!("{:?} {:?}", ok, intersected);
    
    let ok = intersected.intersect_coords(b, b);
    println!("{:?} {:?}", ok, intersected);
    
    let ok = intersected.intersect_coords(a, a);
    println!("{:?} {:?}", ok, intersected);
    
    let ok = intersected.intersect_coords(a, c);
    println!("{:?} {:?}", ok, intersected);
    
    let ok = intersected.intersect_coords(d, a);
    println!("{:?} {:?}", ok, intersected);
    
    let ok = intersected.intersect_coords(a, e);
    println!("{:?} {:?}", ok, intersected);
    
    let ok = intersected.intersect_coords(e, a);
    println!("{:?} {:?}", ok, intersected);
}
