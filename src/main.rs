use id3::{Tag, TagLike, Version};
//use id3::{Tag, TagLike};
use std::env;
use std::fs;

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

        if filename.len() > 10 {

            let pos = find_number(filename.as_bytes(), 0, 4);

            // If the year was in the info, then get it as a middle point. Otherwise try to split with -
            if pos > 0 {
                _artist.replace_range(pos.._artist.len(), "");
                _album.replace_range(0..pos+4, "");
                _year.replace_range(pos+4.._year.len(), "");
                _year.replace_range(0..pos, "");
            } else {
                 let first = find_first_char(filename.as_bytes(), b'-');
                 let last  = find_last_char(filename.as_bytes(), b'-');

                _artist.replace_range(first.._artist.len(), "");
                _album.replace_range(0..last+1, "");
                _year.replace_range(last.._year.len(), "");
                _year.replace_range(0..first+1, "");
            }

            _artist = remove_whitespace(&_artist);
            _album  = remove_whitespace(&_album);
            _year   = remove_whitespace(&_year);
        } else {
            _artist = format!("empty");
            _year   = format!("0");
            _album  = format!("empty");
        }

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

        if filename.len() > 5 {
            pos = find_number(filename.as_bytes(), 0, 2);

            if pos > 0 {
                _title.replace_range(0..pos+2, "");
                _track.replace_range(pos+2.._track.len(), "");
                _track.replace_range(0..pos, "");
            } else {
                pos = find_first_char(_track.as_bytes(), b'-');
                _title.replace_range(0..pos+1, "");
                _track.replace_range(pos.._track.len(), "");
            }

            _title = remove_whitespace(&_title);
            _track = remove_whitespace(&_track);
        } else {
            _track = format!("0");
            _title = format!("empty");
        }

        TrackInfo { title: _title, track: _track.parse().unwrap() }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Find a number in a string
//
// Inputs
// input     - input string
// start_pos - possible start position for the string
// size      - number of chars the wanted length of the number
//
// Return: Position where the seeked value was found, or 0 if not found
//////////////////////////////////////////////////////////////////////////////////////
fn find_number(input: &[u8], start_pos: usize, size: usize) -> usize {
    let mut count: usize = 0;
    let mut start: usize = 0;
    let mut found: bool = false;
    let mut total: usize = 0;

    for i in start_pos..input.len() {
        let compare = input[i] as char;

        if compare.is_numeric() { count += 1
        } else { total = count; count = 0; }

        if count == 1 { start = i }

        if total == size { found = true; break; }
    }

    if found { start
    } else { 0 }
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
        if comparison[i] != b' ' && comparison[i] != b'-' && comparison[i] != b'_' && comparison[i] != b'\n' && comparison[i] != b'\t' { start = i; break; }
    }

    let mut j = comparison.len() - 1;
    while j > 0 {
        if comparison[j] != b' ' && comparison[j] != b'-' && comparison[j] != b'_' && comparison[j] != b'\n' && comparison[j] != b'\t' { end = j + 1; break; }
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
fn print_tag(info: &ID3TagInfo) {
    print!("\n\n'{}' - {} - '{}' : {} - '{}'\n", info.artist, info.year, info.album, info.track, info.title);
}

//////////////////////////////////////////////////////////////////////////////////////
// Remove tags from given audio file
//
// Inputs
// path - Path to audio file
//////////////////////////////////////////////////////////////////////////////////////
fn remove_tag(path: &str, info: &ID3TagInfo, v: bool) {
    if is_empty(info) {
        if !v { println!("No need to remove, item is already empty! '{}'", path); }
    } else {
        let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
            Tag::remove_from_path(path)?;
            Ok(())
        };

        if let Err(_err) = do_steps() {
            if !v {println!("No tags found in '{}'", path); }
        } else {
            println!("Removed tags from '{}'", path);
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
fn compare(tag: &ID3TagInfo, orig: &ID3TagInfo) -> u8 {
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
    let mut error: bool = false;

    if        tag.artist == "empty" { error = true;
    } else if tag.title  == "empty" { error = true;
    } else if tag.album  == "empty" { error = true;
    } else if tag.track  <  1       { error = true;
    } else if tag.year   <  1900    { error = true;
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
// v    - verbose boolean
//////////////////////////////////////////////////////////////////////////////////////
fn write_tags(path: &str, tag: &ID3TagInfo, orig: &ID3TagInfo, v: bool) {
    let error = is_empty(tag);

    if !error {
        let count = compare(tag, orig);

        if count < 5 {
            let new_tag: Tag = get_tag(tag);

            let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
                new_tag.write_to_path(path, Version::Id3v24)?;
                Ok(())
            };

            if let Err(_err) = do_steps() {
                println!("Failed to update ID3 to '{}'", path);
            } else {
                println!("Updated ID3 tags to '{}' as artist:'{}' year::'{}' album::'{}' track:'{}' title:'{}'", path, tag.artist, tag.year, tag.album, tag.track, tag.title);
            }
        } else if v {
            println!("No need to update, as the information already matches! '{}'", path);
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
    println!("remove - remove ID3 tag completely");
    println!("-v     - verbose functionality, will print out all info\n");
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
        let mut verbose:    bool   = false;
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
            } else if arg == "-v" { verbose = true;
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

            if command == "print" {
                print_tag(&tag_data);
            } else if command == "update" {
                if count > 0 {
                    let write_tag: ID3TagInfo = ID3TagInfo::force(&artist, &year, &album, &track, &title);
                    write_tags(&path, &write_tag, &tag_data, verbose);
                } else if overwrite != "empty" {
                    let write_tag: ID3TagInfo = ID3TagInfo::parse(&overwrite);
                    write_tags(&path, &write_tag, &tag_data, verbose);
                } else {
                    let write_tag: ID3TagInfo = ID3TagInfo::parse(&path);
                    write_tags(&path, &write_tag, &tag_data, verbose);
                }
            } else if command == "remove" {
                remove_tag(&path, &tag_data, verbose);
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

