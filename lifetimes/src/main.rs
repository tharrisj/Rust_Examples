#[derive(Debug)]
struct Player<'a> {
    name: String,
    job: &'a str,
    level: u32,
}

impl<'a> Player<'a> {
    fn new(name: &str) -> Self {
        Player{
            name: name.to_owned(),
            job: "test",
            level: 1_u32,
        }
    }

    fn set_job(&mut self, set_job: &'a str) {
        self.job = set_job;
    }

    fn get_job(&self) -> &str {
        self.job
    }
}

fn main() {
    let mut me = Player::new("thomas");
    {
        let job = String::from("This is a job");
        me.set_job(&job);
    
        println!("Player is: {:?}", me);
    }
    let job = me.get_job();

    println!("Got Job: {}", job);
}
