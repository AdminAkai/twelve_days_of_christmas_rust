fn main() {
    let day_array: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lyrics_array: [&str; 12] = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me,", day_array[i]);
        for lyric in (0..=i).rev() {
            println!("{}", lyrics_array[lyric]);
        }
    }
}
