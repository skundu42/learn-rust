-- Track each user's lesson progress across the curriculum
create table if not exists public.lesson_progress (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  lesson_id integer not null,
  status text not null default 'in_progress' check (status in ('in_progress', 'completed')),
  started_at timestamptz not null default now(),
  last_viewed_at timestamptz not null default now(),
  completed_at timestamptz,
  updated_at timestamptz not null default now(),
  visit_count integer not null default 1 check (visit_count > 0),
  unique(user_id, lesson_id)
);

alter table public.lesson_progress enable row level security;

create policy "progress_select_own" on public.lesson_progress
  for select using (auth.uid() = user_id);

create policy "progress_insert_own" on public.lesson_progress
  for insert with check (auth.uid() = user_id);

create policy "progress_update_own" on public.lesson_progress
  for update using (auth.uid() = user_id)
  with check (auth.uid() = user_id);

create policy "progress_delete_own" on public.lesson_progress
  for delete using (auth.uid() = user_id);
