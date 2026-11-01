-- Guests (ADR 0012): anyone may start learning without an account. A guest is a person row
-- marked `guest`, a learner in every open class. The app role still cannot write users or
-- memberships; only this function can, and only into open classes, up to a limit.
alter table cohorts add column open boolean not null default false;
alter table users add column guest boolean not null default false;

-- Returns the new guest's id, or null when `max_guests` guests already exist.
create function private.join_as_guest(max_guests integer) returns uuid
  language plpgsql volatile security definer set search_path = '' as $$
declare
  guest uuid := gen_random_uuid();
  tenant uuid;
begin
  -- One guest at a time, so the limit holds under concurrent sign-ups.
  perform pg_advisory_xact_lock(hashtextextended('zidara.guests', 0));
  if (select count(*) from public.users where users.guest) >= max_guests then
    return null;
  end if;
  select c.tenant_id into tenant from public.cohorts c where c.open order by c.name limit 1;
  if tenant is null then
    return null;
  end if;
  insert into public.users (id, tenant_id, display_name, kind, guest)
    values (guest, tenant, 'Guest', 'person', true);
  insert into public.memberships (user_id, cohort_id, role)
    select guest, c.id, 'learner' from public.cohorts c where c.open and c.tenant_id = tenant;
  return guest;
end $$;

-- The nightly reset of the hosted sandbox (ADR 0010): every event goes, and every guest with
-- their memberships; the seeded people and classes stay. Run as the owner, never by the app.
create function private.reset_demo() returns void
  language sql volatile set search_path = '' as $$
  truncate public.events, public.applied_envelopes;
  delete from public.memberships m using public.users u where u.id = m.user_id and u.guest;
  delete from public.users u where u.guest;
$$;

revoke execute on function private.reset_demo() from public;
revoke execute on function private.join_as_guest(integer) from public;
grant execute on function private.join_as_guest(integer) to zidara_app;
