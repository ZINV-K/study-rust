// Option
#[derive(Debug, Clone)]
struct Locker {
    number: i32,
    student: Option<String>,
}

impl Locker {
    fn new(number:i32, student:Option<&str> ) -> Self{
        match student {
            None => Self {number, student: None },
            _ => Self {number, student: Some(student.unwrap().to_owned()) },
        }
    }

    fn show_locker(&self) {
        // let mut student:String;
        
        // match self.student {
        //     None => student = "no one".to_owned(),
        //     _ => student = self.student.to_owned().unwrap(),
        // }
        
        // println!("{:?} locker is used by {:?}.", self.number, student);
        
        // let mut student2:&str;

        // match self.student {
        //     None => student2 = "no one",
        //     _ => student2 = self.student.as_ref().unwrap(),
        // }

        // println!("{:?} locker is used by {:?}.", self.number, student2);

        match &self.student {
            Some(name) => println!("{:?} locker is used by {:?}.", self.number, name),
            None => println!("{:?} locker is unassigned.", self.number)
        }

        
    }
}

fn main() {
    let mut lockers:Vec<Locker> = Vec::new();

    lockers.push(Locker::new(0, Some("Anna")));
    lockers.push(Locker::new(1, None));
    lockers.push(Locker::new(2, Some("Billy")));
    lockers.push(Locker::new(3, None));
    lockers.push(Locker::new(4, Some("Chales")));

    for locker in &lockers {
        locker.show_locker();
    }
}
