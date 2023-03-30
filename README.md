# Poster

Simple reddit-esque social media like website (hopefully)

## NOTES

### TODO

- Redirect "/" to "/poster"
- Permission redirecting
- On signup/login redirect to where they were before
- Post/Comment score
    - CSS/clarity
    - Upvote/downvote is a toggle 
- Polish and CSS
- CLEAN CODE?
    - Or just move on to next project

### Models

- User
    - username (primary key, text)
    - password_hash (not null, text)
- Account
    - id (primary key, autoincrement, integer)
    - display_name (not null, unique text)
    - user (not full, integer, foregin key to user: 1 acount to 1 user)
- Post
    - id (primary key, autoincrement, integer)
    - title (not null, text)
    - content (text)
    - author (not null, integer, foreign key to account: 1 account to many posts)
    - date (not null, date format??)
    - Upvotes (many to many with account)
        - List of accounts who upvoted
    - Downvotes (many to many with accounts)
        - List of accounts who downvoted
    - Score (not null, integer)
        - Upvotes - Downvotes
- Comment
    - id (primary key, autoincrement, integer)
    - content (not null, text)
    - author (not null, integer - foreign key to account: 1 account to many comments)
    - date (not null, date format??)
    - Upvotes (many to many with account)
        - List of accounts who upvoted
    - Downvotes (many to many with accounts)
        - List of accounts who downvoted
    - Score (not null, integer)
        - Upvotes - Downvotes
    - parent post (foreign key to post 1:many, can be null if parent comment is not)
    - parrent comment (foreign key 1:many to comment, can be null if parent post is not)