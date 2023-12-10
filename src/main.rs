use id3::{Tag, TagLike, Version};
use std::env;
use std::fs;
use chrono::Datelike;

// Basic ID3 tag information
struct ID3TagInfo {
    artist: String, title: String, album: String,
    track: u32, year: i32,
}

// Album info struct
struct AlbumInfo {
    artist: String, album: String,
    year: i32,
}

// Track info struct
struct TrackInfo {
    title: String,
    track: u32,
}

//////////////////////////////////////////////////////////////////////////////////////
// AlbumInfo struct handlers
//////////////////////////////////////////////////////////////////////////////////////
impl AlbumInfo {
    //////////////////////////////////////////////////////////////////////////////////////
    // Parse album info from a given string
    //
    // Inputs
    // path - path to string, or just directory name
    //
    // Return: parsed information struct from the string
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn parse(path: &str) -> AlbumInfo {
        let mut split = find_last_char(path.as_bytes(), b'/');
        let mut filename: String = format!("{}", path);

        // Check if path has folders, and get the deepest folder only
        if split > 0 {
            filename.replace_range(split..filename.len(), "");
            split = find_last_char(filename.as_bytes(), b'/');
            if split > 0 {
                filename.replace_range(0..split+1, "");
            }
        }

        let mut _artist: String = format!("{}", filename);
        let mut _year:   String = format!("{}", filename);
        let mut _album:  String = format!("{}", filename);
        let current_date = chrono::Utc::now();
        let minuses = get_char_count(filename.as_bytes(), b'-');

        if filename.len() > 10 || minuses > 0 {
            let pos2 = find_verified_number(&filename, 1800, current_date.year() as usize, 4);
            let pos = pos2 as usize;

            if pos2 >= 0 {                                                  // If the year was in the info, then get it as a middle point. Otherwise try to split with -
                _artist.replace_range(pos.._artist.len(), "");
                _album.replace_range(0..pos+4, "");
                _year.replace_range(pos+4.._year.len(), "");
                _year.replace_range(0..pos, "");
            } else if minuses > 1 {                                         // If there are more than one minus, there's probably a year
                 let first = find_first_char(filename.as_bytes(), b'-');
                 let last  = find_last_char(filename.as_bytes(), b'-');

                _artist.replace_range(first.._artist.len(), "");
                _album.replace_range(0..last+1, "");
                _year.replace_range(last.._year.len(), "");
                _year.replace_range(0..first+1, "");
            } else if minuses == 1 {                                        // Only one minus, only artist and album name
                 let first = find_first_char(filename.as_bytes(), b'-');
                _year   = format!("{}", current_date.year());
                _album.replace_range(0..first+1, "");
                _artist.replace_range(first.._artist.len(), "");
            } else {                                                        // Only artists name is found
                _album  = format!("empty");
            }
        } else {
            _album  = format!("empty");
        }

        _artist = remove_whitespace(&_artist);
        _album  = remove_whitespace(&_album);
        _year   = verify_number(&_year, 1900, current_date.year());

        if _artist == "." && _album == "empty" { _artist = format!("empty"); }

        AlbumInfo { artist: _artist, year: _year.parse().unwrap(), album: _album }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// TrackInfo struct handlers
//////////////////////////////////////////////////////////////////////////////////////
impl TrackInfo {
    //////////////////////////////////////////////////////////////////////////////////////
    // Parse track information from filename
    //
    // Inputs
    // file - a filename or full path to a filename
    //
    // Return: Parsed track information struct
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn parse(file: &str) -> TrackInfo {
        let mut filename: String = format!("{}", file);
        let mut pos = find_last_char(filename.as_bytes(), b'.');
        let split = find_last_char(filename.as_bytes(), b'/');

        // Remove file extension
        filename.replace_range(pos..filename.len(), "");
        if split > 0 {
            filename.replace_range(0..split, "");
        }

        let mut _track: String = format!("{}", filename);
        let mut _title: String = format!("{}", filename);

        // Filename probably as a number in it
        if filename.len() > 5 {
            let pos2 = find_number(filename.as_bytes(), 0, 2);
            if pos2 < 0 { find_number(filename.as_bytes(), 0, 1); }
            pos = pos2 as usize;

            if pos2 >= 0 {                                          // Separate number and track
                _title.replace_range(0..pos+2, "");
                _track.replace_range(pos+2.._track.len(), "");
                _track.replace_range(0..pos, "");
            } else {                                                // Se if there's a minus, and try to separate by that
                pos = find_first_char(_track.as_bytes(), b'-');

                if pos > 0 {
                    _title.replace_range(0..pos+1, "");
                    _track.replace_range(pos.._track.len(), "");
                }
            }
        }

        _title = remove_whitespace(&_title);
        _track = verify_number(&_track, 1, 99);

        TrackInfo { title: _title, track: _track.parse().unwrap() }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Get string from within the given limits
//
// Inputs
// input    - input string
// first    - first char position
// last     - last char position
//
// Return: String between the given bounds
//////////////////////////////////////////////////////////////////////////////////////
fn get_string_between(input: &str, first: usize, last: usize) -> String {
    let mut gutted: String = format!("{}", input);

    gutted.replace_range(last..input.len(), "");
    gutted.replace_range(0..first, "");
    gutted
}

//////////////////////////////////////////////////////////////////////////////////////
// Find a number in string with given limits
//
// Inputs
// str      - Input string
// min      - Minimum accepted value
// max      - Maximum accepted value
// len      - Wanted length of the value
//
// Return position to the found value, or -1 if not found
//////////////////////////////////////////////////////////////////////////////////////
fn find_verified_number(input: &str, min: usize, max: usize, len: usize) -> i32 {
    let mut start: usize = 0;
    let mut _run: bool = true;
    let mut ret: i32 = -1;

    while _run == true {
        let pos = find_number(input.as_bytes(), start, len);
        if pos < 0 { _run = false; break; }
        let pos2 = pos as usize;

        let test = get_string_between(input, pos2, pos2 + len);
        let found = verify_number(&test, min as i32, max as i32);

        if found != "0" { ret = pos; _run = false; break; }
        else { start = pos as usize + len; }
    }

    ret
}

//////////////////////////////////////////////////////////////////////////////////////
// Find a number in a string
//
// Inputs
// input     - input string
// start_pos - possible start position for the string
// size      - number of chars the wanted length of the number
//
// Return: Position where the seeked value was found, or -1 if not found
//////////////////////////////////////////////////////////////////////////////////////
fn find_number(input: &[u8], start_pos: usize, size: usize) -> i32 {
    let mut count: usize = 0;
    let mut start: usize = start_pos;
    let mut found: bool = false;
    let mut total: usize = 0;

    for i in start_pos..input.len() {
        let compare = input[i] as char;

        if compare.is_numeric() { count += 1
        } else { total = count; count = 0; }

        if count == 1 { start = i }

        if total == size { found = true; break; }
    }

    if found { start as i32
    } else { -1 }
}

//////////////////////////////////////////////////////////////////////////////////////
// Clean and verify number string
//
// Inputs
// input    - input value as string
// min      - minimum accepted value
// max      - maximum accepted value
//
// Return: 0 if not a number, not within bounds, or the value as String, if OK
//////////////////////////////////////////////////////////////////////////////////////
fn verify_number(input: &str, min: i32, max: i32) -> String  {
    let mut handler: String = remove_whitespace(&input);

    if ! handler.parse::<i32>().is_ok() { return format!("0") }

    let value: i32 = handler.parse().unwrap();

    if value < min || value > max { handler = format!("0");
    } else {                        handler = format!("{}", value); }

    handler
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
    } else { input = format!("empty"); }

    if value {
        if input.len() > 6 {
            input.replace_range(0..5, "");
            input.truncate(input.len() -1);
        } else {
            input = format!("empty");
        }
    } else {
        if input.len() > 8 {
            input.replace_range(0..6, "");
            input.truncate(input.len() -2);
        } else {
            input = format!("empty");
        }
    }

    remove_whitespace(&input)
}

//////////////////////////////////////////////////////////////////////////////////////
// Find first position of the comparison character
//
// Inputs
// input   - input string
// compare - comparison character
//
// Return: First position of the found character
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
// Return: Last position of the found character
//////////////////////////////////////////////////////////////////////////////////////
fn find_last_char(input: &[u8], compare: u8) -> usize {
    let mut pos: usize = 0;

    for i in 0..input.len() {
        if input[i] == compare { pos = i; }
    }

    pos
}

//////////////////////////////////////////////////////////////////////////////////////
// Get nmber of given characters in a string
//
// Inputs
// input    - source string
// compare  - comparison character
//
// Return: Number of items found in string
//////////////////////////////////////////////////////////////////////////////////////
fn get_char_count(input: &[u8], compare: u8) -> usize {
    let mut count: usize = 0;

    for i in 0..input.len() {
        if input[i] == compare { count += 1; }
    }

    count
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
        if comparison[i] != b' ' && comparison[i] != b'-' && comparison[i] != b'_' && comparison[i] != b'\n' && comparison[i] != b'\t' && comparison[i] != b'/' { start = i; break; }
    }

    let mut j = comparison.len() - 1;
    let mut k: i32 = j as i32; 

    while k >= 0 {
        if comparison[j] != b' ' && comparison[j] != b'-' && comparison[j] != b'_' && comparison[j] != b'\n' && comparison[j] != b'\t' && comparison[j] != b'/' { end = j + 1; break; }
        j -= 1;
        k -= 1;
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

        if _year  == "empty" { _year = format!("0")
        } else if ! _year.parse::<i32>().is_ok() { _year = format!("0"); }

        if _track == "empty" { _track = format!("0")
        } else if ! _track.parse::<i32>().is_ok() { _track = format!("0"); }

        ID3TagInfo { artist: _artist, title: _title, album: _album, track: _track.parse().unwrap(), year: _year.parse().unwrap() }
    }

    //////////////////////////////////////////////////////////////////////////////////////
    // Parse tag data for the audio file
    //
    // Inputs
    // input - input string in format of "ARTIST - YEAR - ALBUM / TRACK - SONGNAME"
    //
    // Return: ID3TagInfo Struct with parsed data
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn parse(input: &str) -> ID3TagInfo {
        let pos = find_last_char(input.as_bytes(), b'/');

        if pos > 0 {
            let atag: AlbumInfo = AlbumInfo::parse(input);
            let ttag: TrackInfo = TrackInfo::parse(input);
            ID3TagInfo { artist: atag.artist, title: ttag.title, album: atag.album, track: ttag.track, year: atag.year }
        } else {
            ID3TagInfo { artist: format!("empty"), title: format!("empty"), album: format!("empty"), track: 0, year: 0 }
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////
    // Force tagdata handlers
    //
    // Inputs
    // _artist  - Artist name
    // _year    - Release year
    // _album   - Album name
    // _track   - Track ID
    // _title   - Title name of the track
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn force(_artist: &str, _year: &str, _album: &str, _track: &str, _title: &str) -> ID3TagInfo {
        ID3TagInfo { artist: format!("{}", _artist), title: format!("{}", _title), album: format!("{}", _album), track: _track.parse().unwrap(), year: _year.parse().unwrap() }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Print out read ID3 tag info
//////////////////////////////////////////////////////////////////////////////////////
fn print_tag(info: &ID3TagInfo, path: &str) {
    print!("\"{}\" '{}' - {} - '{}' : {} - '{}'\n", path, info.artist, info.year, info.album, info.track, info.title);
}

//////////////////////////////////////////////////////////////////////////////////////
// Remove tags from given audio file
//
// Inputs
// path - Path to audio file
// v    - verbose status
//////////////////////////////////////////////////////////////////////////////////////
fn remove_tag(path: &str, info: &ID3TagInfo, v: &str) {
    if is_empty(info) {
        if v == "loud" || v == "verbose" { println!("No need to remove, item is already empty! '{}'", path); }
    } else {
        let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
            Tag::remove_from_path(path)?;
            Ok(())
        };

        if let Err(_err) = do_steps() {
            if v == "loud" { println!("No tags found in '{}'", path); }
        } else {
            if v != "silent" && v != "entry" { println!("Removed tags from '{}'", path); }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Compare two tag handlers
//
// Inputs
// tag  - first tag
// orig - second tag
//
// Return: Number of items that match
//////////////////////////////////////////////////////////////////////////////////////
fn compare_tags(tag: &ID3TagInfo, orig: &ID3TagInfo) -> u8 {
    let mut count: u8 = 0;

    if tag.artist == orig.artist { count += 1; }
    if tag.title  == orig.title  { count += 1; }
    if tag.album  == orig.album  { count += 1; }
    if tag.track  == orig.track  { count += 1; }
    if tag.year   == orig.year   { count += 1; }

    count
}

//////////////////////////////////////////////////////////////////////////////////////
// Check that tag items are correct
//
// Inputs
// tag  - Input tag struct
//
// Return: true if any of the items are incorrect
//////////////////////////////////////////////////////////////////////////////////////
fn is_empty(tag: &ID3TagInfo) -> bool {
    let mut error: i16 = 0;

    if tag.artist == "empty" { error += 1; }
    if tag.title  == "empty" { error += 1; }
    if tag.album  == "empty" { error += 1; }
    if tag.track  <  1       { error += 1; }
    if tag.year   <  1900    { error += 1; }

    if error > 4 { true
    } else { false }
}

//////////////////////////////////////////////////////////////////////////////////////
// Get number of empty items
//////////////////////////////////////////////////////////////////////////////////////
fn _empty_count(tag: &ID3TagInfo) -> i16 {
    let mut error: i16 = 0;

    if        tag.artist == "empty" { error += 1;
    } else if tag.title  == "empty" { error += 1;
    } else if tag.album  == "empty" { error += 1;
    } else if tag.track  <  1       { error += 1;
    } else if tag.year   <  1900    { error += 1;
    }

    error
}

//////////////////////////////////////////////////////////////////////////////////////
// Initialize tag handler with previously read data
//
// Inputs
// source   - Tag info struct
//
// Return: Newly initialized Tag handler
//////////////////////////////////////////////////////////////////////////////////////
fn get_tag(source: &ID3TagInfo) -> Tag {
    let mut target = Tag::new();

    target.set_album(&source.album);
    target.set_title(&source.title);
    target.set_artist(&source.artist);
    target.set_track(source.track);
    target.set_year(source.year);

    target
}

//////////////////////////////////////////////////////////////////////////////////////
// Write tag-data to audio file
//
// Inputs
// path - path to audio file
// tag  - previously parsed tag data
// v    - verbose status
//////////////////////////////////////////////////////////////////////////////////////
fn write_tags(path: &str, tag: &ID3TagInfo, orig: &ID3TagInfo, v: &str) {
    let error = is_empty(tag);

    if !error {
        let count = compare_tags(tag, orig);

        if count < 5 {
            let new_tag: Tag = get_tag(tag);

            let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
                new_tag.write_to_path(path, Version::Id3v24)?;
                Ok(())
            };

            if let Err(_err) = do_steps() {
                if v != "entry" { println!("Failed to update ID3 to '{}'", path); }
            } else {
                if v != "silent" && v != "entry" { println!("Updated ID3 tags to '{}'\n    -> as artist:'{}' year:'{}' album:'{}' track:'{}' title:'{}'", path, tag.artist, tag.year, tag.album, tag.track, tag.title); }
            }
        } else if v == "verbose" || v == "loud" { println!("No need to update, as the information already matches! '{}'", path); }
    } else if v != "entry" {
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
    println!("remove - remove ID3 tag completely");
    println!("-v     - verbose functionality, will print more info");
    println!("-s     - silent verbose functionality, will print out only errors");
    println!("-e     - entry verbose functionality, will print only what file is being handled");
    println!("-l     - loud verbose functionality, will print all info\n");
    println!("OVERWRITE_STRING:");
    println!("Format the string in style of: ARTIST - YEAR - ALBUM / TRACK - SONGNAME");
    println!("Please don't use - or / other than as a splitters.\n");
    println!("The other option is to separate each item for update as it's own string input, in the following order (all required):");
    println!("\"ARTIST\" \"YEAR\" \"ALBUM\" \"TRACK\" \"SONG NAME\"\n\n");
    println!("Examples:");
    println!("id3handler print \"PATH\"");
    println!("id3handler remove \"PATH\"");
    println!("id3handler update \"PATH\"");
    println!("id3handler update \"PATH\" \"STRING AS PATH\"");
    println!("id3handler update \"PATH\" \"ARTIST\" \"YEAR\" \"ALBUM\" \"TRACK\" \"SONG NAME\"");
}

//////////////////////////////////////////////////////////////////////////////////////
// Main functions
//
// Input
// args - Inputs from the commandline
//////////////////////////////////////////////////////////////////////////////////////
fn main() {

    let mut args = env::args().skip(1);

    if args.len() > 1 {
        let mut command:    String = format!("empty");
        let mut path:       String = format!("empty");
        let mut overwrite:  String = format!("empty");
        let mut verbose:    String = format!("normal");
        let mut success:    bool   = true;
        let mut count:      u8     = 0;
        let mut artist:     String = format!("empty");
        let mut year:       String = format!("0");
        let mut album:      String = format!("empty");
        let mut track:      String = format!("0");
        let mut title:      String = format!("empty");

        while let Some(arg) = args.next() {
            let scopy = format!("{}", arg);

            if arg == "print" || arg == "update" || arg == "remove" {
                command = format!("{}", arg);
            } else if arg == "-v" { verbose = format!("verbose");
            } else if arg == "-s" { verbose = format!("silent");
            } else if arg == "-l" { verbose = format!("loud");
            } else if arg == "-e" { verbose = format!("entry");
            } else if path == "empty" && fs::metadata(scopy).is_ok() {
                path = format!("{}", arg);
            } else if arg.contains("/") {
                overwrite = format!("{}", arg);
            } else {
                if count == 0 {
                    artist = format!("{}", arg);
                } else if count == 1 {
                    year = format!("{}", arg);
                } else if count == 2 {
                    album = format!("{}", arg);
                } else if count == 3 {
                    track = format!("{}", arg);
                } else if count == 4 {
                    title = format!("{}", arg);
                } else {
                    println!("Too many inputs! Aborting!");
                    success = false;
                }
                count += 1;
            }
        }

        if fs::metadata(&path).is_ok() && success {
            let tag_data: ID3TagInfo = ID3TagInfo::read(&path);

            if verbose == "loud" || verbose == "entry" { println!("Handling '{}'", path); }

            if command == "print" {
                print_tag(&tag_data, &path);
            } else if command == "update" {
                if count > 0 {
                    let write_tag: ID3TagInfo = ID3TagInfo::force(&artist, &year, &album, &track, &title);
                    write_tags(&path, &write_tag, &tag_data, &verbose);
                } else if overwrite != "empty" {
                    let write_tag: ID3TagInfo = ID3TagInfo::parse(&overwrite);
                    write_tags(&path, &write_tag, &tag_data, &verbose);
                } else {
                    let write_tag: ID3TagInfo = ID3TagInfo::parse(&path);
                    write_tags(&path, &write_tag, &tag_data, &verbose);
                }
            } else if command == "remove" {
                remove_tag(&path, &tag_data, &verbose);
            } else {
                println!("Unknown or failed command {}", command);
                print_help();
            }
        } else {
            if success { println!("File doesn't exists: '{}'", path); }
            print_help();
        }

    } else {
        println!("No input variables given!");
        print_help();
    }
}

