create function util.set_updated_by() returns trigger as $$
  begin
    new.updated_by = new.created_by;
    return new;
  end;
$$ language plpgsql;
