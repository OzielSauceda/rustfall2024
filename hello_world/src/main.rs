#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Masters,
    PhD,
}
#[derive(Debug)]

enum Major{
    CS,
    Electrical,
}
#[derive(Debug)]

struct Student{
    name:String,
    grade: GradeLevel,
    major: Major,
}

impl Student {
    fn new(name:String, grade:GradeLevel, major:Major) -> Self{
        Student {
            name: name,
            grade: grade,
            major:major,
        }
    }

    fn intro(&self){
        println!("My name is: {}", self.name);
        match self.grade{
            GradeLevel::Bachelor => println!("\nMy classification level is Bachelors"),
            GradeLevel::Masters => println!("\nMy classification level is Masters"),
            GradeLevel::PhD => println!("\nMy classification level is PhD"),
        }
        
        match self.major{
            Major::CS => println!("\nMy major is Computer Science"),
            Major::Electrical => println!("\nMy major is Electrical Engineering"),
        }


    }
}

fn main() {
    let s1= Student::new(
        "John".to_string(),
        GradeLevel::Bachelor, 
        Major::CS
    );

    s1.intro();

}