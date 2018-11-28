create function util.set_created_at() returns trigger as $$
  begin
    new.created_at = now();
    return new;
  end;
$$ language plpgsql;
