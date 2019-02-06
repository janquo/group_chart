# group_chart
create Last.fm albums collage for a group of people

## usage
Manual set up of key.txt and users.txt files required.

### key.txt
The file contains last.fm API key required to use the API.

### users.txt
Nicknames of users to be included in a collage, each in a new line

### manual_tracks.txt
Albums with a tracks value to override. This is due to last.fm not having sufficient data needed to compute a score for every album.

The easiest way to fill this file is to copy entries from the nones.txt when the program asks you to and change values to the number of tracks on a given album. 

Albums from nones.txt that aren't in manual_tracks.txt wont be included in a collage.
