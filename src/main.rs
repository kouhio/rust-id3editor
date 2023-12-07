use id3::{Tag, TagLike, Version};
//use id3::{Tag, TagLike};
use std::env;
use std::fs;

// Basic ID3 tag information
struct ID3TagInfo {
    artist: String, title: String, album: String,
    track: u32, year: i32,
}

//////////////////////////////////////////////////////////////////////////////////////
// Read wanted data from ID3 stream
//
// Inputs
// path - path to audio file
// src  - type of wanted value
//
// Return: wanted value in string, or "empty" if no tag is found
//////////////////////////////////////////////////////////////////////////////////////
fn handle_tag_string(path: &str, src: &str) -> String {
    let mut tag_in: Tag = Default::default();

    let mut do_steps = || -> Result<(), Box<dyn std::error::Error>> {
        tag_in = Tag::read_from_path(path)?;
        Ok(())
    };

    if let Err(_err) = do_steps() {
        return format!("empty");
    }

    let tag = Tag::read_from_path(path).unwrap();
    let mut input: String;
    let mut value: bool = false;

    if        src == "album"  { input = format!("{:?}", tag.album());
    } else if src == "artist" { input = format!("{:?}", tag.artist());
    } else if src == "title"  { input = format!("{:?}", tag.title());
    } else if src == "year" {
        value = true;
        input = format!("{:?}", tag.year());
    } else if src == "track" {
        value = true;
        input = format!("{:?}", tag.track());
    } else { input = format!("null"); }

    if input == "null" {
        if value { input = format!("0"); }
    } else {
        if value {
            input.replace_range(0..5, "");
            input.truncate(input.len() -1);
        } else {
            input.replace_range(0..6, "");
            input.truncate(input.len() -2);
        }
    }

    input
}

//////////////////////////////////////////////////////////////////////////////////////
// Get number of characters in a string
//
// Inputs
// input   - input string
// compare - comparison character
//
// return: Number of found characters
//////////////////////////////////////////////////////////////////////////////////////
fn get_chars(input: &[u8], compare: u8) -> u32 {
    let mut count: u32 = 0;

    for i in 0..input.len() {
        if input[i] == compare { count += 1; }
    }

    count
}

//////////////////////////////////////////////////////////////////////////////////////
// Find first position of the comparison character
//
// Inputs
// input   - input string
// compare - comparison character
//
// return: First position of the found character
//////////////////////////////////////////////////////////////////////////////////////
fn find_first_char(input: &[u8], compare: u8) -> usize {
    let mut pos: usize = 0;

    for i in 0..input.len() {
        if input[i] == compare { pos = i; break; }
    }

    pos
}

//////////////////////////////////////////////////////////////////////////////////////
// Find last position of the comparison character
//
// Inputs
// input   - input string
// compare - comparison character
//
// return: Last position of the found character
//////////////////////////////////////////////////////////////////////////////////////
fn find_last_char(input: &[u8], compare: u8) -> usize {
    let mut pos: usize = 0;

    for i in 0..input.len() {
        if input[i] == compare { pos = i; }
    }

    pos
}

//////////////////////////////////////////////////////////////////////////////////////
// Remove whitespace from string with loops
//
// Inputs
// input - string to be cleaned
//
// Return: cleaned string
//////////////////////////////////////////////////////////////////////////////////////
fn remove_whitespace(input: &str) -> String {
    let mut modified: String = format!("{}", input);
    let mut start: usize = 0;
    let mut end: usize = input.len();
    let comparison: &[u8] = input.as_bytes();

    for i in 0..comparison.len() {
        if comparison[i] != b' ' { start = i; break; }
    }

    let mut j = comparison.len() - 1;
    while j > 0 {
        if comparison[j] != b' ' { end = j + 1; break; }
        j -= 1;
    }

    modified.replace_range(end..modified.len(), "");
    modified.replace_range(0..start, "");

    modified
}

//////////////////////////////////////////////////////////////////////////////////////
// ID3 tag struct handlers
//////////////////////////////////////////////////////////////////////////////////////
impl ID3TagInfo {
    //////////////////////////////////////////////////////////////////////////////////////
    // Read tag data from path
    //
    // Input
    // path - Path to audio file
    //
    // Return: ID3TagInfo Struct with read data
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn read(path: &str) -> ID3TagInfo {
        let mut _artist : String = handle_tag_string(path, "artist");
        let mut _title  : String = handle_tag_string(path, "title");
        let mut _album  : String = handle_tag_string(path, "album");
        let mut _year   : String = handle_tag_string(path, "year");
        let mut _track  : String = handle_tag_string(path, "track");

        if _year  == "empty" { _year = format!("0") };
        if _track == "empty" { _track = format!("0") };

        ID3TagInfo { artist: _artist, title: _title, album: _album, track: _track.parse().unwrap(), year: _year.parse().unwrap() }
    }

    //////////////////////////////////////////////////////////////////////////////////////
    // Write tag data to audio file
    //
    // Inputs
    // input - input string in format of "ARTIST - YEAR - ALBUM / TRACK - SONGNAME"
    //
    // Return: ID3TagInfo Struct with parsed data
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn parse(input: &str) -> ID3TagInfo {
        let splitters = get_chars(input.as_bytes(), b'/');
        let mut _artist : String = format!("empty");
        let mut _title  : String = format!("empty");
        let mut _album  : String = format!("empty");
        let mut _year   : String = format!("0");
        let mut _track  : String = format!("0");
        let mut filename : String = format!("{}", input);
        let mut filepath : String = format!("{}", input);
        let mut error: bool = false;

        let mut pos = find_last_char(input.as_bytes(), b'/');

        if pos > 0 {
            if splitters > 1 {
                filename.replace_range(0..pos+1, "");
                filepath.replace_range(pos..filepath.len(), "");
                pos = find_last_char(filepath.as_bytes(), b'/');
                filepath.replace_range(0..pos+1, "");
            } else if splitters == 1 {
                filename.replace_range(0..pos+1, "");
                filepath.replace_range(pos..filepath.len(), "");
            } else { error = true; }

            if !error {
                // Remove filetype from filename
                pos = find_last_char(filename.as_bytes(), b'.');
                filename.replace_range(pos..filename.len(), "");

                // Get base inits
                _artist = format!("{}", filepath);
                _year   = format!("{}", filepath);
                _album  = format!("{}", filepath);
                _track  = format!("{}", filename);
                _title  = format!("{}", filename);

                // Get album splitters
                let first = find_first_char(filepath.as_bytes(), b'-');
                let last  = find_last_char(filepath.as_bytes(), b'-');

                // Split track number and title
                pos = find_last_char(_track.as_bytes(), b'-');
                _title.replace_range(0..pos+1, "");
                _track.replace_range(pos.._track.len(), "");
                _title = remove_whitespace(&_title);
                _track = remove_whitespace(&_track);

                // Split band, year and album
                _artist.replace_range(first.._artist.len(), "");
                _album.replace_range(0..last+1, "");
                _year.replace_range(last.._year.len(), "");
                _year.replace_range(0..first+1, "");
                _artist = remove_whitespace(&_artist);
                _album  = remove_whitespace(&_album);
                _year   = remove_whitespace(&_year);
            }
        }

        ID3TagInfo { artist: _artist, title: _title, album: _album, track: _track.parse().unwrap(), year: _year.parse().unwrap() }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Print out read ID3 tag info
//////////////////////////////////////////////////////////////////////////////////////
fn print_tag(info: &ID3TagInfo) {
    print!("\n\n'{}' - {} - '{}' : {} - '{}'\n", info.artist, info.year, info.album, info.track, info.title);
}

//////////////////////////////////////////////////////////////////////////////////////
// Remove tags from given audio file
//
// Inputs
// path - Path to audio file
//////////////////////////////////////////////////////////////////////////////////////
fn remove_tag(path: &str) {
    let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
        Tag::remove_from_path(path)?;
        Ok(())
    };

    if let Err(_err) = do_steps() {
        println!("No tags found in '{}'", path);
    } else {
        println!("Removed tags from '{}'", path);
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Write tag-data to audio file
//
// Inputs
// path - path to audio file
// tag  - previously parsed tag data
//////////////////////////////////////////////////////////////////////////////////////
fn write_tags(path: &str, tag: &ID3TagInfo) {
    let mut error: bool = false;

    if        tag.artist == "empty" { error = true;
    } else if tag.title  == "empty" { error = true;
    } else if tag.album  == "empty" { error = true;
    } else if tag.track  <  1       { error = true;
    } else if tag.year   <  1950    { error = true;
    }

    if !error {
        let mut new_tag = Tag::new();
        new_tag.set_album(&tag.album);
        new_tag.set_title(&tag.title);
        new_tag.set_artist(&tag.artist);
        new_tag.set_track(tag.track);
        new_tag.set_year(tag.year);

        let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
            new_tag.write_to_path(path, Version::Id3v24)?;
            Ok(())
        };

        if let Err(_err) = do_steps() {
            println!("Failed to update ID3 to '{}'", path);
        } else {
            println!("Updated ID3 tags to '{}' as artist:'{}' year::'{}' album::'{}' track:'{}' title:'{}'", path, tag.artist, tag.year, tag.album, tag.track, tag.title);
        }
    } else {
        println!("Some input values are incorrect: artist:'{}' title:'{}' album:'{}' track:'{}' year:'{}'. ABORTING!", tag.artist, tag.title, tag.album, tag.track, tag.year);
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Commandline help
//////////////////////////////////////////////////////////////////////////////////////
fn print_help() {
    println!("ID3 Tag handler\n");
    println!("params: COMMAND PATH_TO_FILE OVERWRITE_STRING\n");
    println!("COMMANDS:");
    println!("print  - print tag information from PATH_TO_FILE");
    println!("update - update file tag infomation based on path and filename");
    println!("remove - remove ID3 tag completely\n");
    println!("OVERWRITE_STRING:");
    println!("Format the string in style of: ARTIST - YEAR - ALBUM / TRACK - SONGNAME");
    println!("Please don't use - or / other than as a splitter. This also concerns the filenames");
}

//////////////////////////////////////////////////////////////////////////////////////
// Main functions
//
// Input
// args[1] - Command to be executed
// args[2] - Path to audio file
// args[3] - Possible overwrite info for TAG instead of the path and filename
//////////////////////////////////////////////////////////////////////////////////////
fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {

        if fs::metadata(&args[2]).is_ok() {
            let tag_data: ID3TagInfo = ID3TagInfo::read(&args[2]);

            if args[1] == "print" {
                print_tag(&tag_data);
            } else if args[1] == "update" {
                if args.len() > 3 {
                    let write_tag: ID3TagInfo = ID3TagInfo::parse(&args[3]);
                    write_tags(&args[2], &write_tag);
                } else {
                    let write_tag: ID3TagInfo = ID3TagInfo::parse(&args[2]);
                    write_tags(&args[2], &write_tag);
                }
            } else if args[1] == "remove" {
                remove_tag(&args[2]);
            } else {
                println!("Unknown command {}", args[1]);
            }
        } else {
            println!("File doesn't exists: '{}'", args[2]);
            print_help();
        }

    } else {
        println!("No input variables given!");
        print_help();
    }
}

