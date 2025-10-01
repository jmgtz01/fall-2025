struct Student{
    name: String,
    major: String,
}

impl Student {
    fn new(n:String, m:String) ->Student{
        Student{
            name:n,
            major:m,
        }
    }

    fn get_name(&self) -> &String{
        return &self.name
    }
    fn set_name(&mut self, new_name: String){
        self.name = new_name;
    }

    fn get_major(&self) -> &String{
        return &self.major
    }
    fn set_major(&mut self, new_major: String){
        self.major = new_major;
    }
}



fn main() {
    let mut my_student = Student::new("Julian Gutierrez".to_string(), "Computer Engineering".to_string());
    println!("Student's name is: {}", my_student.name);
    println!("Student's major is: {}", my_student.major);

    my_student.set_name("Julian M. Gutierrez".to_string());
    my_student.set_major("Computer Science".to_string());
    println!("\nStudent's name is: {}", my_student.get_name());
    println!("Student's major is: {}", my_student.get_major());
}
