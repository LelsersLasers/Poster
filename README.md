# Poster

Simple reddit-esque social media like website (hopefully)

## NOTES

### Stuff

- DB
    - SQLite backend, sqlx in rust
- Templates
    - minijinja?
- User auth
    - idk
    - Maybe axum_login
        - https://docs.rs/axum-login/latest/axum_login/#users
- Framework
    - axum

### Models

- User
    - id (primary key, autoincrement, integer)
    - username (unique, not null, text)
    - display name (unique, not null, text)
    - password (not null, text)
- Post
    - id (primary key, autoincrement, integer)
    - title (not null, text)
    - content (text)
    - author (not null, integer, foreign key to user: 1 user to many posts)
    - date (not null, date format??)
    - Upvotes (many to many with user)
        - List of users who upvoted
    - Downvotes (many to many with user)
        - List of users who downvoted
    - Score (not null, integer)
        - Upvotes - Downvotes
- Comment
    - content (not null, text)
    - author (not null, integer - foreign key to user: 1 user to many comments)
    - date (not null, date format??)
    - Upvotes (many to many with user)
        - List of users who upvoted
    - Downvotes (many to many with user)
        - List of users who downvoted
    - Score (not null, integer)
        - Upvotes - Downvotes
    - parent post (foreign key to post 1:many, can be null if parent comment is not)
    - parrent comment (foreign key 1:many to comment, can be null if parent post is not)

### Views

- Home
    - List of posts
    - Top bar
        - Search
        - (if logged in) Logout
        - (if not logged in) Login
        - (if not logged in) Sign up
    - Post button
- Login page
- Sign up page
- Create post page

### Templates

- Base
    - Blank
        - Login/signup
    - Main page
    - Create post page
