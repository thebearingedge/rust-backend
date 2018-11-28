create function util.set_updated_at() returns trigger as $$
  begin
    new.updated_at = now();
    return new;
  end;
$$ language plpgsql;
