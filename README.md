# A quick ID3-tag handler, a demonstration of rust

With this rust app you can print, remove or add ID3tag to a single file by specific path-type or an input string.


params: COMMAND PATH_TO_FILE OVERWRITE_STRING

COMMANDS:
print  - print tag information from PATH_TO_FILE
update - update file tag infomation based on path and filename
remove - remove ID3 tag completely

OVERWRITE_STRING:
Format the string in style of: ARTIST - YEAR - ALBUM / TRACK - SONGNAME

