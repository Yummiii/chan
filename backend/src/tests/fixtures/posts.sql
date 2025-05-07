insert into
    posts (
        board_id,
        thread_id,
        user_id,
        content,
        image_id,
        created_at
    )
values
    (
        1,
        null,
        null,
        'This is a test post.',
        'img123',
        1672531199
    ),
    (
        1,
        1,
        null,
        'Another test post.',
        null,
        1672531200
    ),
    (
        2,
        null,
        'abc123',
        'Yet another test post.',
        'img456',
        1672531201
    );
