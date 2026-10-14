-- Zidara schema: isolation is a data rule. See docs/adr and the zidara skill (postgres.md).
create schema if not exists private;

-- Roles are cluster-wide; parallel test databases race on this, hence the handler.
-- The password is set outside migrations (ops, or tests/support once per container).
do $$ begin
  create role zidara_app login nobypassrls;
exception when duplicate_object then null;
end $$;

create table tenants (id uuid primary key, name text not null);
create table cohorts (
  id uuid primary key,
  tenant_id uuid not null references tenants(id),
  name text not null,
  lesson_id text not null,
  framework text not null
);
create index cohorts_tenant_id_idx on cohorts (tenant_id);
create table users (
  id uuid primary key,
  tenant_id uuid not null references tenants(id),
  display_name text not null,
  kind text not null check (kind in ('person', 'agent'))
);
create index users_tenant_id_idx on users (tenant_id);
create table memberships (
  user_id uuid not null references users(id),
  cohort_id uuid not null references cohorts(id),
  role text not null check (role in ('learner', 'teacher', 'assessment_agent')),
  primary key (user_id, cohort_id)
);
create index memberships_cohort_id_idx on memberships (cohort_id);
create table events (
  seq bigint generated always as identity primary key,
  id uuid not null unique,
  tenant_id uuid not null references tenants(id),
  cohort_id uuid not null references cohorts(id),
  learner_id uuid references users(id),
  at_ms bigint not null,
  actor jsonb not null,
  body jsonb not null
);
create index events_cohort_order_idx on events (cohort_id, at_ms, seq);
create index events_tenant_id_idx on events (tenant_id);
create index events_learner_id_idx on events (learner_id);
create table applied_envelopes (
  id uuid primary key,
  tenant_id uuid not null references tenants(id),
  cohort_id uuid not null references cohorts(id),
  user_id uuid not null references users(id),
  applied_at timestamptz not null default now()
);
create index applied_envelopes_tenant_id_idx on applied_envelopes (tenant_id);
create index applied_envelopes_cohort_id_idx on applied_envelopes (cohort_id);
create index applied_envelopes_user_id_idx on applied_envelopes (user_id);

create function private.me() returns uuid language sql stable
  as $$ select nullif(current_setting('app.user_id', true), '')::uuid $$;
-- security definer: reads memberships without RLS, so policies on memberships do not recurse.
create function private.my_cohorts() returns setof uuid language sql stable security definer
  set search_path = '' as $$
  select m.cohort_id from public.memberships m where m.user_id = private.me() $$;
create function private.overseen_cohorts() returns setof uuid language sql stable security definer
  set search_path = '' as $$
  select m.cohort_id from public.memberships m
  where m.user_id = private.me() and m.role in ('teacher', 'assessment_agent') $$;

alter table tenants enable row level security;          alter table tenants force row level security;
alter table cohorts enable row level security;          alter table cohorts force row level security;
alter table users enable row level security;            alter table users force row level security;
alter table memberships enable row level security;      alter table memberships force row level security;
alter table events enable row level security;           alter table events force row level security;
alter table applied_envelopes enable row level security; alter table applied_envelopes force row level security;

create policy cohorts_mine on cohorts for select to zidara_app
  using (id in (select private.my_cohorts()));
create policy tenants_mine on tenants for select to zidara_app
  using (id in (select c.tenant_id from cohorts c));
-- Data minimisation: learners see their own rows only; overseers see their cohorts'.
create policy memberships_mine on memberships for select to zidara_app
  using (user_id = (select private.me()) or cohort_id in (select private.overseen_cohorts()));
create policy users_mine on users for select to zidara_app
  using (id = (select private.me()) or id in (select m.user_id from memberships m));
create policy events_read on events for select to zidara_app
  using (cohort_id in (select private.my_cohorts())
         and (learner_id is null or learner_id = (select private.me())
              or cohort_id in (select private.overseen_cohorts())));
create policy events_write on events for insert to zidara_app
  with check (cohort_id in (select private.my_cohorts())
              and (learner_id is null or learner_id = (select private.me())
                   or cohort_id in (select private.overseen_cohorts())));
create policy envelopes_mine on applied_envelopes for all to zidara_app
  using (user_id = (select private.me()))
  with check (user_id = (select private.me()) and cohort_id in (select private.my_cohorts()));

grant usage on schema public, private to zidara_app;
grant execute on all functions in schema private to zidara_app;
grant select on tenants, cohorts, users, memberships to zidara_app;
grant select, insert on events, applied_envelopes to zidara_app;
