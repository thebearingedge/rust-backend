create domain nonempty_text as text
  constraint is_not_empty
       check (value !~ '^$|^\s+$');
