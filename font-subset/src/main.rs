use read_fonts::{FontRef, TableProvider};
use write_fonts::{
    from_obj::ToOwnedTable,
    tables::head::Head,
    types::LongDateTime,
    FontBuilder,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn seconds_since_font_epoch() -> LongDateTime { 
    let current_time_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    return LongDateTime::new(current_time_millis as i64);
}

fn main() {
    // read
    let path_to_my_font_file = std::path::Path::new("../font-test-data/test_data/ttf/noto_serif_display_trimmed.ttf");
    let font_bytes = std::fs::read(path_to_my_font_file).unwrap();

    // new
    let font = FontRef::new(&font_bytes).expect("failed to read font data");
    let mut head: Head = font.head().expect("missing 'head' table").to_owned_table();
    head.modified = seconds_since_font_epoch();
    
    // modfiy

    // TODO

    // build
    let new_bytes = FontBuilder::new()
        .add_table(&head)
        .unwrap() // errors if we can't compile 'head', unlikely here
        .copy_missing_tables(font)
        .build();

    // write
    std::fs::write("mynewfont.ttf", &new_bytes).unwrap();
}
