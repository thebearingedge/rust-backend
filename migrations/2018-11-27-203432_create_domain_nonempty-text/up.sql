create domain nonempty_text as citext
  check (value !~ '^$|^\s+$');
