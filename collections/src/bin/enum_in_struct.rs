enum Jobstatus {
    Applied,
    Interviewing,
    Offered,
    Rejected,
}

struct Candidate {
    name: String,
    status: Jobstatus,
}

impl Candidate {
    fn new(name: String, status: Jobstatus) -> Candidate {
        Candidate { name, status }
    }

    fn get_status(&self) -> &str {
        match self.status {
            Jobstatus::Applied => "Applied",
            Jobstatus::Interviewing => "Interviewing",
            Jobstatus::Offered => "Offered",
            Jobstatus::Rejected => "Rejected",
        }
    }

    fn update_status(&mut self, new_status: Jobstatus) {
        self.status = new_status;
    }
}

fn main() {
    let mut candidate = Candidate::new(String::from("Alice"), Jobstatus::Applied);
    println!("{} is currently {}", candidate.name, candidate.get_status());

    candidate.update_status(Jobstatus::Interviewing);
    println!("{} is currently {}", candidate.name, candidate.get_status());

    candidate.update_status(Jobstatus::Offered);
    println!("{} is currently {}", candidate.name, candidate.get_status());

    let mut candidate_1 = Candidate::new(String::from("Bob"), Jobstatus::Applied);
    println!(
        "{} is currently {}",
        candidate_1.name,
        candidate_1.get_status()
    );

    candidate_1.update_status(Jobstatus::Rejected);
    println!(
        "{} is currently {}",
        candidate_1.name,
        candidate_1.get_status()
    );
}
