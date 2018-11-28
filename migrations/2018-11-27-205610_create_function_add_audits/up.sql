create function util.add_audits(table_name regclass) returns void as $$
  begin
    execute format('
      alter table %I
        add column
            created_by uuid not null
            references users (user_id),
        add column
            updated_by uuid not null
            references users (user_id)
    ', table_name);
    execute format('
      create trigger set_updated_by_insert
        before insert
        on %I
        for each row
        execute procedure util.set_updated_by()
    ', table_name);
    execute format('
      create trigger keep_created_by_update
        before update
        on %I
        for each row
        execute procedure util.keep_created_by()
    ', table_name);
  end;
$$ language plpgsql;
