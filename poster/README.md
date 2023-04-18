# Poster personal notes

## TODO

- Redirect "/" ("") to "/poster"
    - Right now panics because `NormalizePathLayer::trim_trailing_slash()` turns "/" into "" which is an invalid axum path
- Infinite scroll on main page
    - Don't show same posts
        - When to reset seen_post_ids??
        - **How to handle back button???**
- Polish and CSS
    - Usable on multiple screen sizes, resolutions, and mobile

## CSS

- Visual clarity to collapsed navbar?
- Dynamic? (does it work on mobile?)
