alter table public.lesson_progress
  add column if not exists status text;

alter table public.lesson_progress
  add column if not exists started_at timestamptz;

alter table public.lesson_progress
  add column if not exists last_viewed_at timestamptz;

alter table public.lesson_progress
  add column if not exists updated_at timestamptz;

alter table public.lesson_progress
  add column if not exists visit_count integer;

update public.lesson_progress
set
  status = coalesce(status, case when completed_at is not null then 'completed' else 'in_progress' end),
  started_at = coalesce(started_at, completed_at, now()),
  last_viewed_at = coalesce(last_viewed_at, completed_at, now()),
  updated_at = coalesce(updated_at, completed_at, now()),
  visit_count = coalesce(visit_count, 1);

alter table public.lesson_progress
  alter column status set default 'in_progress';

alter table public.lesson_progress
  alter column status set not null;

alter table public.lesson_progress
  alter column started_at set default now();

alter table public.lesson_progress
  alter column started_at set not null;

alter table public.lesson_progress
  alter column last_viewed_at set default now();

alter table public.lesson_progress
  alter column last_viewed_at set not null;

alter table public.lesson_progress
  alter column updated_at set default now();

alter table public.lesson_progress
  alter column updated_at set not null;

alter table public.lesson_progress
  alter column visit_count set default 1;

alter table public.lesson_progress
  alter column visit_count set not null;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'lesson_progress_status_check'
  ) then
    alter table public.lesson_progress
      add constraint lesson_progress_status_check
      check (status in ('in_progress', 'completed'));
  end if;

  if not exists (
    select 1
    from pg_constraint
    where conname = 'lesson_progress_visit_count_check'
  ) then
    alter table public.lesson_progress
      add constraint lesson_progress_visit_count_check
      check (visit_count > 0);
  end if;
end
$$;

alter table public.lesson_progress enable row level security;

drop policy if exists "progress_update_own" on public.lesson_progress;

create policy "progress_update_own" on public.lesson_progress
  for update using (auth.uid() = user_id)
  with check (auth.uid() = user_id);
