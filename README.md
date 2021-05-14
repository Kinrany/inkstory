Convert [instory.su](https://instory.su) internal story format into [Ink](https://www.inklestudios.com/ink/) stories.

This crate provides a CLI that takes a story URL, story ID, or an exported `.json` file,
and generates an equivalent Ink story.

Usage examples:

```sh
# With story URL
inkstory https://instory.su/story/12345 > story.ink

# With a URL that exports JSON
inkstory https://example.com/story.json > story.ink

# With a JSON file
inkstory ./story.json > story.ink

# With a story ID
inkstory 12345 > story.ink
```
