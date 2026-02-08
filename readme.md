# miu

Data values and functions used in `Marble It Up!` and `Marble It Up! Ultra`.  

## Game Data
```rust
use miu::data::ultra::Data;

fn main() {
    let data = Data::new().unwrap();
    println!("levels: {:#?}", data.levels());
}
```

## Score
```rust
use miu::Score;

async fn fetch_scores() -> Vec<Score> {
    todo!()
}

fn main() {
    let scores = fetch_scores();
    
    let first = scores[0];
    println!("world record: {}, {:.3} ({})", first.username, first.time, first.platform);
}
```

## Weekly Challenge
```rust
use miu::{Weekly, NameLang};

fn main() {
    let json = "{ ... }";
    let weekly = Weekly::from_json(&json).unwrap();

    let current = weekly.score_buckets.current;
    println!("{}: {:?}", current.get_name(NameLang::En), current.levels);
}
```