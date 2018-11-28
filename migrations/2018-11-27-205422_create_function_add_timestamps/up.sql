create function util.add_timestamps(table_name regclass) returns void as $$
  begin
    execute format('
      alter table %I
        add column
            created_at timestamptz(6) not null,
        add column
            updated_at timestamptz(6) not null
    ', table_name);
    execute format('
      create trigger set_created_at_insert
        before insert
        on %I
        for each row
        execute procedure util.set_created_at()
    ', table_name);
    execute format('
      create trigger set_updated_at_insert
        before insert
        on %I
        for each row
        execute procedure util.set_updated_at()
    ', table_name);
    execute format('
      create trigger keep_created_at_update
        before update
        on %I
        for each row
        execute procedure util.keep_created_at()
    ', table_name);
    execute format('
      create trigger set_updated_at_update
        before update
        on %I
        for each row
        when (new *<> old)
        execute procedure util.set_updated_at()
    ', table_name);
  end;
$$ language plpgsql;
