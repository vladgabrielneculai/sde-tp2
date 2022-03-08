struct Vector{
    valoare: Vec<i32>,
}

impl Vector{
    fn new()->Vector{
        return Vector { valoare: Vec::new() };
    }
    fn add(&mut self, nr: i32){
        let mut rnr = 0;
        for i in 0..self.valoare.len(){
            if self.valoare[i]<nr {
                rnr = i+1;
            }
        }
        self.valoare.insert(rnr,nr);
      
    }
    fn remove(&mut self, i:usize){
        self.valoare.remove(i);
    }

   fn print(&self){
       println!("{:?}",self.valoare);  //varianta simplificata

    // varianta complexa
    //    for i in 0..self.valoare{
    //        println!("{}",self.valoare[i]);
    //    }
   }
   
}

fn main(){
    let mut v = Vector::new();
    v.add(9);
    v.add(5);
    v.add(10);
    v.print();
}

#[cfg(test)]
mod tests; 
