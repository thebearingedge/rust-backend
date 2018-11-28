create function util.keep_created_at() returns trigger as $$
  begin
    new.created_at = old.created_at;
    return new;
  end;
$$ language plpgsql;
