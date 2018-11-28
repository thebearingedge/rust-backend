create function util.keep_created_by() returns trigger as $$
  begin
    new.created_by = old.created_by;
    return new;
  end;
$$ language plpgsql;
