use skills::{Skills, Skill};

#[test]
fn test_load_skills() {
    match Skills::new(&Path::new("./testdata/test_skills.idx"), &Path::new("./testdata/test_skills.mul")) {
        Ok(_skills) => {
            //Passed
        },
        Err(message) => panic!("{}", message)
    }
}

#[test]
fn test_serialize() {
    let in_string = "Sandwich";
    let skill = Skill::new(true, in_string.to_string()).serialize();
    assert_eq!(skill[0], 1u8);
    assert_eq!(skill[1], 'S' as u8);
    assert_eq!(skill.len(), in_string.len() + 2) //One for the clickable prefix, one for string terminal
}