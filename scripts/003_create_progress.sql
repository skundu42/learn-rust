-- Track which lessons each user has completed
create table if not exists public.lesson_progress (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  lesson_id integer not null,
  completed_at timestamptz not null default now(),
  unique(user_id, lesson_id)
);

alter table public.lesson_progress enable row level security;

create policy "progress_select_own" on public.lesson_progress
  for select using (auth.uid() = user_id);

create policy "progress_insert_own" on public.lesson_progress
  for insert with check (auth.uid() = user_id);

create policy "progress_delete_own" on public.lesson_progress
  for delete using (auth.uid() = user_id);
