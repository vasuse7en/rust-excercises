use ordinal::Ordinal;

pub fn twelve_days_of_christmas() {
    let lyric_array: [&str; 12] = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a leaping",
        "Nine ladies dancing",
        "Eight maids a milking",
        "Seven swans a swimming",
        "Six geese a laying",
        "Five gold rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And partridge in a pear tree",
    ];

    for i in 1..13 {
        let day = Ordinal(i).to_string();
        let stmt1 = format!("On the {} day of Christmas", day);
        let stmt2 = "My true love gave to me";

        println!("{}", stmt1);
        println!("{}", stmt2);

        if i == 1 {
            println!("A partridge in a pear tree")
        } else {
            for j in (1..i + 1).rev() {
                println!("{}", lyric_array[12 - j]);
            }
        }

        println!("\n")
    }
}
