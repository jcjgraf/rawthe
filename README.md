# RawExtractor

RawExtractor, or in short *rex*, is a small utility designed for extracting embedded thumbnails from various raw image formats. Its main goals are simplicity and ease-of-use. Unlike existing tools like exiv2 or exiftools, it doesn't require complex input arguments nor any bash hacking.

## Goals / Features
- Extract embedded thumbnail with largest available resolution
- Preserve metadata on extracted thumbnail
- Ease of use
- (TODO) Offer flexibility in choice of destination and file naming

## Dependencies
- [Exiv2](https://exiv2.org/): C++ Library used to interact with the metadata of the image
- [gexiv2](https://wiki.gnome.org/Projects/gexiv2): GObject wrapper around Exiv2
