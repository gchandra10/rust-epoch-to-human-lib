# Library for demonstration

## Usage

```
cargo add epoch_to_human
```

```
// main.rs

use epoch_to_human::*;

fn main(){
    println!("{:?}",epoch_to_date(1719315296));
    println!("{:?}",date_to_epoch("2024-06-25 11:34:56"));
}
    
```