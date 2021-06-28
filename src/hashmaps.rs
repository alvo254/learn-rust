use std::collections::HashMap;

// The type HashMap<K, V>
// stores a mapping of keys of type K to values of type V. It does this via a hashing
// function, which determines how it places these keys and values into memory

pub fn hashing(){
    //-----------------------------creating a new hashmap-------------------------------------
    let mut hashed = HashMap::new();
    hashed.insert(String::from("Blue"), 10);
    hashed.insert(String::from("Gray"), 12);
    println!("Teams: {:?}", hashed);
    //------------------Creating a hash map from a list of teams and a list of scores----------------------------------
    let teams = vec![String::from("Blue"), String::from("Red")];
    println!("{:?}",teams);
    let initial = vec![5, 10];
    let scores:HashMap<_,_> = teams.iter().zip(initial.iter()).collect();
    println!("{:?}", scores);

    //--------------------Accessing the values in a hashmap by providing its name------------------------
    let team_name = String::from("Blue");
    let blue_team_score = hashed.get(&team_name);
    println!("{:?}", blue_team_score);
    let scorez = initial.get(0);
    println!("{:?}",scorez);

    //--------------------------------iterating over hash maps -----------------------------
    for (key, value) in &scores{
        println!("key: {}: value: {}", key, value)
    }

    //---------------------Only Inserting a Value If the Key Has No Value--------------------
    //API for this called that entry takes the key you want to check as a parameter.
    //Using the entry method to only insert if the key does not already have a value
    hashed.entry(String::from("Blue")).or_insert(50);

    //----------------------Updating a Value Based on the Old Value----------------------------
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}",map)

}