create domain nonempty_text as text
  check (value !~ '^$|^\s+$');
